#[cfg(test)]
mod tests {
    use crate::error::AppError;
    use axum::http::StatusCode;
    use axum::response::IntoResponse;

    #[test]
    fn test_unauthorized_error() {
        let error = AppError::Unauthorized;
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_invalid_credentials_error() {
        let error = AppError::InvalidCredentials;
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_forbidden_error() {
        let error = AppError::Forbidden;
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[test]
    fn test_not_found_error() {
        let error = AppError::NotFound("Resource".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_validation_error() {
        let error = AppError::Validation("Invalid input".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_conflict_error() {
        let error = AppError::Conflict("Already exists".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::CONFLICT);
    }

    #[test]
    fn test_internal_error() {
        let error = AppError::Internal("Something went wrong".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_error_display() {
        let error = AppError::NotFound("User".to_string());
        assert_eq!(error.to_string(), "Resource not found: User");
    }

    #[test]
    fn test_validation_error_display() {
        let error = AppError::Validation("Email is required".to_string());
        assert_eq!(error.to_string(), "Validation error: Email is required");
    }
}
