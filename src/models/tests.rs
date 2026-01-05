#[cfg(test)]
mod tests {
    use crate::models::{
        BookmarkPreview, CreateBookmark, CreateCategory, CreateNote, CreateTag, CreateUser,
        LoginUser, UpdateBookmark, UpdateCategory, UpdateNote, UpdateTag,
    };
    use uuid::Uuid;
    use validator::Validate;

    #[test]
    fn test_create_user_validation_valid() {
        let user = CreateUser {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            name: "Test User".to_string(),
        };
        assert!(user.validate().is_ok());
    }

    #[test]
    fn test_create_user_validation_invalid_email() {
        let user = CreateUser {
            email: "invalid-email".to_string(),
            password: "password123".to_string(),
            name: "Test User".to_string(),
        };
        assert!(user.validate().is_err());
    }

    #[test]
    fn test_create_user_validation_short_password() {
        let user = CreateUser {
            email: "test@example.com".to_string(),
            password: "short".to_string(),
            name: "Test User".to_string(),
        };
        assert!(user.validate().is_err());
    }

    #[test]
    fn test_create_user_validation_empty_name() {
        let user = CreateUser {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            name: "".to_string(),
        };
        assert!(user.validate().is_err());
    }

    #[test]
    fn test_login_user_validation_valid() {
        let login = LoginUser {
            email: "test@example.com".to_string(),
            password: "anypassword".to_string(),
        };
        assert!(login.validate().is_ok());
    }

    #[test]
    fn test_login_user_validation_invalid_email() {
        let login = LoginUser {
            email: "not-an-email".to_string(),
            password: "password".to_string(),
        };
        assert!(login.validate().is_err());
    }

    #[test]
    fn test_create_bookmark_validation_valid() {
        let bookmark = CreateBookmark {
            url: "https://example.com".to_string(),
            title: "Example Site".to_string(),
            description: Some("A description".to_string()),
            category_id: None,
            tag_ids: None,
        };
        assert!(bookmark.validate().is_ok());
    }

    #[test]
    fn test_create_bookmark_validation_invalid_url() {
        let bookmark = CreateBookmark {
            url: "not-a-valid-url".to_string(),
            title: "Example".to_string(),
            description: None,
            category_id: None,
            tag_ids: None,
        };
        assert!(bookmark.validate().is_err());
    }

    #[test]
    fn test_create_bookmark_validation_empty_title() {
        let bookmark = CreateBookmark {
            url: "https://example.com".to_string(),
            title: "".to_string(),
            description: None,
            category_id: None,
            tag_ids: None,
        };
        assert!(bookmark.validate().is_err());
    }

    #[test]
    fn test_create_note_validation_valid() {
        let note = CreateNote {
            title: "My Note".to_string(),
            content: "Some content".to_string(),
        };
        assert!(note.validate().is_ok());
    }

    #[test]
    fn test_create_note_validation_empty_title() {
        let note = CreateNote {
            title: "".to_string(),
            content: "Content".to_string(),
        };
        assert!(note.validate().is_err());
    }

    #[test]
    fn test_create_tag_validation_valid() {
        let tag = CreateTag {
            name: "important".to_string(),
            color: Some("#ff0000".to_string()),
        };
        assert!(tag.validate().is_ok());
    }

    #[test]
    fn test_create_tag_validation_empty_name() {
        let tag = CreateTag {
            name: "".to_string(),
            color: None,
        };
        assert!(tag.validate().is_err());
    }

    #[test]
    fn test_create_category_validation_valid() {
        let category = CreateCategory {
            name: "Work".to_string(),
            description: Some("Work related items".to_string()),
            parent_id: None,
        };
        assert!(category.validate().is_ok());
    }

    #[test]
    fn test_create_category_validation_empty_name() {
        let category = CreateCategory {
            name: "".to_string(),
            description: None,
            parent_id: None,
        };
        assert!(category.validate().is_err());
    }

    #[test]
    fn test_bookmark_preview_default_values() {
        let preview = BookmarkPreview {
            title: None,
            description: None,
            image: None,
            favicon: None,
        };
        assert!(preview.title.is_none());
        assert!(preview.description.is_none());
        assert!(preview.image.is_none());
        assert!(preview.favicon.is_none());
    }

    #[test]
    fn test_update_bookmark_all_optional() {
        let update = UpdateBookmark {
            url: None,
            title: None,
            description: None,
            category_id: None,
            tag_ids: None,
        };
        // All fields are optional, so this should be valid
        assert!(update.url.is_none());
    }

    #[test]
    fn test_update_note_partial() {
        let update = UpdateNote {
            title: Some("New Title".to_string()),
            content: None,
        };
        assert!(update.title.is_some());
        assert!(update.content.is_none());
    }

    #[test]
    fn test_update_tag_partial() {
        let update = UpdateTag {
            name: None,
            color: Some("#00ff00".to_string()),
        };
        assert!(update.name.is_none());
        assert!(update.color.is_some());
    }

    #[test]
    fn test_update_category_with_parent() {
        let parent_id = Uuid::new_v4();
        let update = UpdateCategory {
            name: Some("Updated Name".to_string()),
            description: None,
            parent_id: Some(parent_id),
        };
        assert_eq!(update.parent_id, Some(parent_id));
    }
}
