use axum::{Extension, Router, routing::get, routing::post};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use xync_server::auth::JwtManager;
use xync_server::handlers;
use xync_server::models::*;
use xync_server::telemetry;
use xync_server::{AppState, Config, Database};

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::register,
        handlers::login,
        handlers::me,
        handlers::create_bookmark,
        handlers::list_bookmarks,
        handlers::get_bookmark,
        handlers::update_bookmark,
        handlers::delete_bookmark,
        handlers::fetch_preview,
        handlers::create_note,
        handlers::list_notes,
        handlers::get_note,
        handlers::update_note,
        handlers::delete_note,
        handlers::create_tag,
        handlers::list_tags,
        handlers::get_tag,
        handlers::update_tag,
        handlers::delete_tag,
        handlers::create_category,
        handlers::list_categories,
        handlers::get_category,
        handlers::update_category,
        handlers::delete_category,
        handlers::liveness,
        handlers::readiness,
    ),
    components(
        schemas(
            CreateUser, LoginUser, UserResponse,
            Bookmark, CreateBookmark, UpdateBookmark, BookmarkPreview,
            Note, CreateNote, UpdateNote,
            Tag, CreateTag, UpdateTag,
            Category, CreateCategory, UpdateCategory,
            handlers::bookmark::PreviewRequest,
            handlers::auth::AuthResponse,
            handlers::health::HealthResponse,
            handlers::health::ReadinessResponse,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "bookmarks", description = "Bookmark management"),
        (name = "notes", description = "Note management"),
        (name = "tags", description = "Tag management"),
        (name = "categories", description = "Category management"),
        (name = "health", description = "Health check endpoints"),
    )
)]
struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::Http::builder()
                        .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}

#[tokio::main]
async fn main() {
    let config = Config::from_env();

    // Initialize telemetry (tracing + optional OpenTelemetry)
    telemetry::init_telemetry(&config);

    tracing::info!(
        service_name = %config.service_name,
        "Starting xync-server"
    );

    let db = Database::connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    db.run_migrations()
        .await
        .expect("Failed to run database migrations");

    tracing::info!("Database connected and migrations applied");

    let jwt = JwtManager::new(&config.jwt_secret, config.jwt_expiration_hours);

    let state = AppState {
        pool: db.pool.clone(),
        jwt: jwt.clone(),
    };

    // Initialize Prometheus metrics
    let metrics_handle = xync_server::metrics::init_metrics();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let api_routes = Router::new()
        .route("/auth/register", post(handlers::register))
        .route("/auth/login", post(handlers::login))
        .route("/auth/me", get(handlers::me))
        .route(
            "/bookmarks",
            post(handlers::create_bookmark).get(handlers::list_bookmarks),
        )
        .route(
            "/bookmarks/{id}",
            get(handlers::get_bookmark)
                .put(handlers::update_bookmark)
                .delete(handlers::delete_bookmark),
        )
        .route("/bookmarks/preview", post(handlers::fetch_preview))
        .route(
            "/notes",
            post(handlers::create_note).get(handlers::list_notes),
        )
        .route(
            "/notes/{id}",
            get(handlers::get_note)
                .put(handlers::update_note)
                .delete(handlers::delete_note),
        )
        .route("/tags", post(handlers::create_tag).get(handlers::list_tags))
        .route(
            "/tags/{id}",
            get(handlers::get_tag)
                .put(handlers::update_tag)
                .delete(handlers::delete_tag),
        )
        .route(
            "/categories",
            post(handlers::create_category).get(handlers::list_categories),
        )
        .route(
            "/categories/{id}",
            get(handlers::get_category)
                .put(handlers::update_category)
                .delete(handlers::delete_category),
        );

    let app = Router::new()
        .nest("/api", api_routes)
        // Health check endpoints
        .route("/health/live", get(handlers::liveness))
        .route("/health/ready", get(handlers::readiness))
        // Prometheus metrics endpoint
        .route(
            "/metrics",
            get(move || {
                let metrics = metrics_handle.render();
                async move { metrics }
            }),
        )
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .layer(Extension(jwt))
        .with_state(state);

    let addr = format!("{}:{}", config.server_host, config.server_port);
    tracing::info!(
        address = %addr,
        swagger_ui = format!("http://{}/swagger-ui/", addr),
        metrics = format!("http://{}/metrics", addr),
        health_live = format!("http://{}/health/live", addr),
        health_ready = format!("http://{}/health/ready", addr),
        "Server started"
    );

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    // Graceful shutdown handling
    let shutdown_signal = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install CTRL+C signal handler");
        tracing::info!("Shutdown signal received, gracefully shutting down...");
        telemetry::shutdown_telemetry();
    };

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal)
        .await
        .unwrap();

    tracing::info!("Server shutdown complete");
}
