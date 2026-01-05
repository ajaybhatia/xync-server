use axum::{
    Extension, Router,
    body::Body,
    http::{Method, Request, StatusCode, header},
    routing::{get, post},
};
use http_body_util::BodyExt;
use serde_json::json;
use sqlx::PgPool;
use testcontainers::{ContainerAsync, runners::AsyncRunner};
use testcontainers_modules::postgres::Postgres;
use tokio::sync::OnceCell;
use tower::ServiceExt;

use xync_server::AppState;
use xync_server::auth::JwtManager;
use xync_server::handlers;

static TEST_CONTAINER: OnceCell<ContainerAsync<Postgres>> = OnceCell::const_new();
static TEST_POOL: OnceCell<PgPool> = OnceCell::const_new();

async fn get_test_pool() -> &'static PgPool {
    TEST_POOL
        .get_or_init(|| async {
            let container = TEST_CONTAINER
                .get_or_init(|| async {
                    Postgres::default()
                        .start()
                        .await
                        .expect("Failed to start postgres container")
                })
                .await;

            let host = container.get_host().await.unwrap();
            let port = container.get_host_port_ipv4(5432).await.unwrap();
            let connection_string =
                format!("postgres://postgres:postgres@{}:{}/postgres", host, port);

            let pool = PgPool::connect(&connection_string)
                .await
                .expect("Failed to connect to test database");

            // Run migrations
            sqlx::query(include_str!(
                "../migrations/20240101000000_initial_schema.sql"
            ))
            .execute(&pool)
            .await
            .expect("Failed to run migrations");

            pool
        })
        .await
}

fn create_test_app(pool: PgPool) -> Router {
    let jwt = JwtManager::new("test-secret-key-for-testing", 24);
    let state = AppState {
        pool,
        jwt: jwt.clone(),
    };

    Router::new()
        .route("/api/auth/register", post(handlers::register))
        .route("/api/auth/login", post(handlers::login))
        .route("/api/auth/me", get(handlers::me))
        .route(
            "/api/bookmarks",
            post(handlers::create_bookmark).get(handlers::list_bookmarks),
        )
        .route(
            "/api/bookmarks/{id}",
            get(handlers::get_bookmark)
                .put(handlers::update_bookmark)
                .delete(handlers::delete_bookmark),
        )
        .route(
            "/api/notes",
            post(handlers::create_note).get(handlers::list_notes),
        )
        .route(
            "/api/notes/{id}",
            get(handlers::get_note)
                .put(handlers::update_note)
                .delete(handlers::delete_note),
        )
        .route(
            "/api/tags",
            post(handlers::create_tag).get(handlers::list_tags),
        )
        .route(
            "/api/tags/{id}",
            get(handlers::get_tag)
                .put(handlers::update_tag)
                .delete(handlers::delete_tag),
        )
        .route(
            "/api/categories",
            post(handlers::create_category).get(handlers::list_categories),
        )
        .route(
            "/api/categories/{id}",
            get(handlers::get_category)
                .put(handlers::update_category)
                .delete(handlers::delete_category),
        )
        .layer(Extension(jwt))
        .with_state(state)
}

async fn body_to_string(body: Body) -> String {
    let bytes = body.collect().await.unwrap().to_bytes();
    String::from_utf8(bytes.to_vec()).unwrap()
}

#[tokio::test]
async fn test_user_registration() {
    let pool = get_test_pool().await.clone();
    let app = create_test_app(pool);

    let request = Request::builder()
        .method(Method::POST)
        .uri("/api/auth/register")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            json!({
                "email": "newuser@example.com",
                "password": "password123",
                "name": "New User"
            })
            .to_string(),
        ))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let body = body_to_string(response.into_body()).await;
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert!(json.get("token").is_some());
    assert!(json.get("user").is_some());
}

#[tokio::test]
async fn test_user_registration_duplicate_email() {
    let pool = get_test_pool().await.clone();

    // First registration
    let app1 = create_test_app(pool.clone());
    let request1 = Request::builder()
        .method(Method::POST)
        .uri("/api/auth/register")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            json!({
                "email": "duplicate@example.com",
                "password": "password123",
                "name": "First User"
            })
            .to_string(),
        ))
        .unwrap();
    app1.oneshot(request1).await.unwrap();

    // Second registration with same email
    let app2 = create_test_app(pool);
    let request2 = Request::builder()
        .method(Method::POST)
        .uri("/api/auth/register")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            json!({
                "email": "duplicate@example.com",
                "password": "password456",
                "name": "Second User"
            })
            .to_string(),
        ))
        .unwrap();

    let response = app2.oneshot(request2).await.unwrap();
    assert_eq!(response.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn test_user_login() {
    let pool = get_test_pool().await.clone();

    // Register first
    let app1 = create_test_app(pool.clone());
    let register_request = Request::builder()
        .method(Method::POST)
        .uri("/api/auth/register")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            json!({
                "email": "logintest@example.com",
                "password": "password123",
                "name": "Login Test"
            })
            .to_string(),
        ))
        .unwrap();
    app1.oneshot(register_request).await.unwrap();

    // Login
    let app2 = create_test_app(pool);
    let login_request = Request::builder()
        .method(Method::POST)
        .uri("/api/auth/login")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            json!({
                "email": "logintest@example.com",
                "password": "password123"
            })
            .to_string(),
        ))
        .unwrap();

    let response = app2.oneshot(login_request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = body_to_string(response.into_body()).await;
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert!(json.get("token").is_some());
}

#[tokio::test]
async fn test_user_login_invalid_credentials() {
    let pool = get_test_pool().await.clone();
    let app = create_test_app(pool);

    let request = Request::builder()
        .method(Method::POST)
        .uri("/api/auth/login")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            json!({
                "email": "nonexistent@example.com",
                "password": "wrongpassword"
            })
            .to_string(),
        ))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_get_current_user() {
    let pool = get_test_pool().await.clone();

    // Register and get token
    let app1 = create_test_app(pool.clone());
    let register_request = Request::builder()
        .method(Method::POST)
        .uri("/api/auth/register")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            json!({
                "email": "metest@example.com",
                "password": "password123",
                "name": "Me Test"
            })
            .to_string(),
        ))
        .unwrap();

    let register_response = app1.oneshot(register_request).await.unwrap();
    let body = body_to_string(register_response.into_body()).await;
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    let token = json["token"].as_str().unwrap();

    // Get current user
    let app2 = create_test_app(pool);
    let me_request = Request::builder()
        .method(Method::GET)
        .uri("/api/auth/me")
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app2.oneshot(me_request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = body_to_string(response.into_body()).await;
    let user: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(user["email"], "metest@example.com");
}

#[tokio::test]
async fn test_unauthorized_access() {
    let pool = get_test_pool().await.clone();
    let app = create_test_app(pool);

    let request = Request::builder()
        .method(Method::GET)
        .uri("/api/auth/me")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_create_and_list_bookmarks() {
    let pool = get_test_pool().await.clone();

    // Register and get token
    let app1 = create_test_app(pool.clone());
    let register_request = Request::builder()
        .method(Method::POST)
        .uri("/api/auth/register")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            json!({
                "email": "bookmarktest@example.com",
                "password": "password123",
                "name": "Bookmark Test"
            })
            .to_string(),
        ))
        .unwrap();

    let register_response = app1.oneshot(register_request).await.unwrap();
    let body = body_to_string(register_response.into_body()).await;
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    let token = json["token"].as_str().unwrap().to_string();

    // Create bookmark
    let app2 = create_test_app(pool.clone());
    let create_request = Request::builder()
        .method(Method::POST)
        .uri("/api/bookmarks")
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .body(Body::from(
            json!({
                "url": "https://example.com",
                "title": "Example Site",
                "description": "An example website"
            })
            .to_string(),
        ))
        .unwrap();

    let create_response = app2.oneshot(create_request).await.unwrap();
    assert_eq!(create_response.status(), StatusCode::CREATED);

    // List bookmarks
    let app3 = create_test_app(pool);
    let list_request = Request::builder()
        .method(Method::GET)
        .uri("/api/bookmarks")
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let list_response = app3.oneshot(list_request).await.unwrap();
    assert_eq!(list_response.status(), StatusCode::OK);

    let body = body_to_string(list_response.into_body()).await;
    let bookmarks: Vec<serde_json::Value> = serde_json::from_str(&body).unwrap();
    assert!(!bookmarks.is_empty());
}

#[tokio::test]
async fn test_create_and_list_notes() {
    let pool = get_test_pool().await.clone();

    // Register and get token
    let app1 = create_test_app(pool.clone());
    let register_request = Request::builder()
        .method(Method::POST)
        .uri("/api/auth/register")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            json!({
                "email": "notetest@example.com",
                "password": "password123",
                "name": "Note Test"
            })
            .to_string(),
        ))
        .unwrap();

    let register_response = app1.oneshot(register_request).await.unwrap();
    let body = body_to_string(register_response.into_body()).await;
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    let token = json["token"].as_str().unwrap().to_string();

    // Create note
    let app2 = create_test_app(pool.clone());
    let create_request = Request::builder()
        .method(Method::POST)
        .uri("/api/notes")
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .body(Body::from(
            json!({
                "title": "My First Note",
                "content": "This is the content of my note"
            })
            .to_string(),
        ))
        .unwrap();

    let create_response = app2.oneshot(create_request).await.unwrap();
    assert_eq!(create_response.status(), StatusCode::CREATED);

    // List notes
    let app3 = create_test_app(pool);
    let list_request = Request::builder()
        .method(Method::GET)
        .uri("/api/notes")
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let list_response = app3.oneshot(list_request).await.unwrap();
    assert_eq!(list_response.status(), StatusCode::OK);

    let body = body_to_string(list_response.into_body()).await;
    let notes: Vec<serde_json::Value> = serde_json::from_str(&body).unwrap();
    assert!(!notes.is_empty());
}

#[tokio::test]
async fn test_validation_errors() {
    let pool = get_test_pool().await.clone();

    // Invalid email format
    let app1 = create_test_app(pool.clone());
    let request1 = Request::builder()
        .method(Method::POST)
        .uri("/api/auth/register")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            json!({
                "email": "not-an-email",
                "password": "password123",
                "name": "Test"
            })
            .to_string(),
        ))
        .unwrap();

    let response1 = app1.oneshot(request1).await.unwrap();
    assert_eq!(response1.status(), StatusCode::BAD_REQUEST);

    // Short password
    let app2 = create_test_app(pool);
    let request2 = Request::builder()
        .method(Method::POST)
        .uri("/api/auth/register")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            json!({
                "email": "valid@example.com",
                "password": "short",
                "name": "Test"
            })
            .to_string(),
        ))
        .unwrap();

    let response2 = app2.oneshot(request2).await.unwrap();
    assert_eq!(response2.status(), StatusCode::BAD_REQUEST);
}
