#[cfg(test)]
mod tests {
    use crate::auth::JwtManager;
    use uuid::Uuid;

    #[test]
    fn test_jwt_token_generation_and_verification() {
        let jwt = JwtManager::new("test-secret-key", 24);
        let user_id = Uuid::new_v4();
        let email = "test@example.com";

        let token = jwt.generate_token(user_id, email).unwrap();
        assert!(!token.is_empty());

        let claims = jwt.verify_token(&token).unwrap();
        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.email, email);
    }

    #[test]
    fn test_jwt_token_with_invalid_secret() {
        let jwt1 = JwtManager::new("secret-one", 24);
        let jwt2 = JwtManager::new("secret-two", 24);

        let user_id = Uuid::new_v4();
        let token = jwt1.generate_token(user_id, "test@example.com").unwrap();

        let result = jwt2.verify_token(&token);
        assert!(result.is_err());
    }

    #[test]
    fn test_jwt_claims_contain_correct_data() {
        let jwt = JwtManager::new("test-secret", 48);
        let user_id = Uuid::new_v4();
        let email = "user@domain.com";

        let token = jwt.generate_token(user_id, email).unwrap();
        let claims = jwt.verify_token(&token).unwrap();

        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.email, email);
        assert!(claims.exp > claims.iat);
    }
}
