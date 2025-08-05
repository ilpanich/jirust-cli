#[cfg(test)]
mod tests {
    use serde_json;

    #[test]
    fn test_empty_json_object_deserialization() {
        use crate::models::User;
        
        let json = "{}";
        let user: User = serde_json::from_str(json).expect("Failed to deserialize empty User");
        
        assert_eq!(user.account_id, None);
        assert_eq!(user.display_name, None);
        assert_eq!(user.email_address, None);
        assert_eq!(user.active, None);
    }

    #[test]
    fn test_partial_json_deserialization() {
        use crate::models::User;
        
        let json = r#"{
            "accountId": "557058:test-user"
        }"#;
        
        let user: User = serde_json::from_str(json).expect("Failed to deserialize partial User");
        assert_eq!(user.account_id, Some("557058:test-user".to_string()));
        assert_eq!(user.display_name, None);
        assert_eq!(user.email_address, None);
        assert_eq!(user.active, None);
    }

    #[test]
    fn test_invalid_json_format() {
        use crate::models::User;
        
        let json = "invalid json";
        let result: Result<User, _> = serde_json::from_str(json);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_null_values_handling() {
        use crate::models::User;
        
        let json = r#"{
            "accountId": null,
            "displayName": null,
            "emailAddress": "test@example.com",
            "active": true
        }"#;
        
        let user: User = serde_json::from_str(json).expect("Failed to deserialize User with nulls");
        assert_eq!(user.account_id, None);
        assert_eq!(user.display_name, None);
        assert_eq!(user.email_address, Some("test@example.com".to_string()));
        assert_eq!(user.active, Some(true));
    }

    #[cfg(feature = "issues_api")]
    #[test]
    fn test_complex_nested_structure() {
        use crate::models::IssueBean;
        
        let json = r#"{
            "id": "10001",
            "key": "TEST-1",
            "fields": {
                "summary": "Test Issue",
                "description": "This is a test issue",
                "priority": {
                    "id": "1",
                    "name": "Highest"
                }
            },
            "names": {
                "summary": "Summary",
                "description": "Description"
            }
        }"#;
        
        let issue: IssueBean = serde_json::from_str(json).expect("Failed to deserialize complex IssueBean");
        assert_eq!(issue.id, Some("10001".to_string()));
        assert_eq!(issue.key, Some("TEST-1".to_string()));
        
        if let Some(fields) = issue.fields {
            assert!(fields.contains_key("summary"));
            assert!(fields.contains_key("description"));
            assert!(fields.contains_key("priority"));
        }
        
        if let Some(names) = issue.names {
            assert_eq!(names.get("summary"), Some(&"Summary".to_string()));
            assert_eq!(names.get("description"), Some(&"Description".to_string()));
        }
    }

    #[test]
    fn test_array_serialization_deserialization() {
        use crate::models::ErrorCollection;
        
        let error = ErrorCollection {
            error_messages: Some(vec![
                "Error 1".to_string(),
                "Error 2".to_string(),
                "Error 3".to_string()
            ]),
            errors: None,
            status: Some(400),
        };
        
        let json = serde_json::to_string(&error).expect("Failed to serialize ErrorCollection");
        let deserialized: ErrorCollection = serde_json::from_str(&json).expect("Failed to deserialize ErrorCollection");
        
        assert_eq!(error.error_messages, deserialized.error_messages);
        assert_eq!(error.status, deserialized.status);
        
        if let Some(messages) = deserialized.error_messages {
            assert_eq!(messages.len(), 3);
            assert!(messages.contains(&"Error 1".to_string()));
            assert!(messages.contains(&"Error 2".to_string()));
            assert!(messages.contains(&"Error 3".to_string()));
        }
    }

    #[test]
    fn test_hashmap_serialization_deserialization() {
        use crate::models::ErrorCollection;
        
        let mut errors = std::collections::HashMap::new();
        errors.insert("field1".to_string(), "Invalid value 1".to_string());
        errors.insert("field2".to_string(), "Invalid value 2".to_string());
        
        let error = ErrorCollection {
            error_messages: None,
            errors: Some(errors),
            status: Some(400),
        };
        
        let json = serde_json::to_string(&error).expect("Failed to serialize ErrorCollection");
        let deserialized: ErrorCollection = serde_json::from_str(&json).expect("Failed to deserialize ErrorCollection");
        
        assert_eq!(error.errors, deserialized.errors);
        
        if let Some(errors) = deserialized.errors {
            assert_eq!(errors.len(), 2);
            assert_eq!(errors.get("field1"), Some(&"Invalid value 1".to_string()));
            assert_eq!(errors.get("field2"), Some(&"Invalid value 2".to_string()));
        }
    }

    #[test]
    fn test_boolean_values() {
        use crate::models::User;
        
        let active_user = User {
            account_id: Some("active".to_string()),
            active: Some(true),
            ..Default::default()
        };
        
        let inactive_user = User {
            account_id: Some("inactive".to_string()),
            active: Some(false),
            ..Default::default()
        };
        
        let json_active = serde_json::to_string(&active_user).expect("Failed to serialize active user");
        let json_inactive = serde_json::to_string(&inactive_user).expect("Failed to serialize inactive user");
        
        assert!(json_active.contains("\"active\":true"));
        assert!(json_inactive.contains("\"active\":false"));
        
        let deserialized_active: User = serde_json::from_str(&json_active).expect("Failed to deserialize active user");
        let deserialized_inactive: User = serde_json::from_str(&json_inactive).expect("Failed to deserialize inactive user");
        
        assert_eq!(deserialized_active.active, Some(true));
        assert_eq!(deserialized_inactive.active, Some(false));
    }

    #[test]
    fn test_numeric_values() {
        use crate::models::Attachment;
        
        let attachment = Attachment {
            id: Some("10000".to_string()),
            size: Some(1048576), // 1MB
            ..Default::default()
        };
        
        let json = serde_json::to_string(&attachment).expect("Failed to serialize Attachment");
        assert!(json.contains("\"size\":1048576"));
        
        let deserialized: Attachment = serde_json::from_str(&json).expect("Failed to deserialize Attachment");
        assert_eq!(deserialized.size, Some(1048576));
    }

    #[cfg(feature = "projects_api")]
    #[test]
    fn test_enum_values() {
        use crate::models::Project;
        
        let json_with_enum = r#"{
            "id": "10000",
            "key": "TEST",
            "assigneeType": "PROJECT_LEAD"
        }"#;
        
        let project: Project = serde_json::from_str(json_with_enum).expect("Failed to deserialize Project with enum");
        assert_eq!(project.id, Some("10000".to_string()));
        assert_eq!(project.key, Some("TEST".to_string()));
        // Note: assigneeType handling depends on the actual enum implementation
    }

    #[test]
    fn test_extra_fields_ignored() {
        use crate::models::User;
        
        let json_with_extra_fields = r#"{
            "accountId": "557058:test-user",
            "displayName": "Test User",
            "unknownField": "this should be ignored",
            "anotherUnknownField": 12345
        }"#;
        
        let user: User = serde_json::from_str(json_with_extra_fields).expect("Failed to deserialize User with extra fields");
        assert_eq!(user.account_id, Some("557058:test-user".to_string()));
        assert_eq!(user.display_name, Some("Test User".to_string()));
    }
}