#[cfg(test)]
mod tests {
    use serde_json;

    #[cfg(feature = "issues_api")]
    #[test]
    fn test_issue_bean_serialization() {
        use crate::models::IssueBean;
        
        let issue = IssueBean {
            id: Some("10001".to_string()),
            key: Some("TEST-1".to_string()),
            expand: Some("names,schema,operations".to_string()),
            ..Default::default()
        };
        
        let json = serde_json::to_string(&issue).expect("Failed to serialize IssueBean");
        assert!(json.contains("\"id\":\"10001\""));
        assert!(json.contains("\"key\":\"TEST-1\""));
        assert!(json.contains("\"expand\":\"names,schema,operations\""));
    }

    #[cfg(feature = "issues_api")]
    #[test]
    fn test_issue_bean_deserialization() {
        use crate::models::IssueBean;
        
        let json = r#"{
            "id": "10001",
            "key": "TEST-1",
            "expand": "names,schema,operations"
        }"#;
        
        let issue: IssueBean = serde_json::from_str(json).expect("Failed to deserialize IssueBean");
        assert_eq!(issue.id, Some("10001".to_string()));
        assert_eq!(issue.key, Some("TEST-1".to_string()));
        assert_eq!(issue.expand, Some("names,schema,operations".to_string()));
    }

    #[cfg(feature = "projects_api")]
    #[test]
    fn test_project_serialization() {
        use crate::models::Project;
        
        let project = Project {
            id: Some("10000".to_string()),
            key: Some("TEST".to_string()),
            name: Some("Test Project".to_string()),
            archived: Some(false),
            ..Default::default()
        };
        
        let json = serde_json::to_string(&project).expect("Failed to serialize Project");
        assert!(json.contains("\"id\":\"10000\""));
        assert!(json.contains("\"key\":\"TEST\""));
        assert!(json.contains("\"name\":\"Test Project\""));
        assert!(json.contains("\"archived\":false"));
    }

    #[cfg(feature = "projects_api")]
    #[test]
    fn test_project_deserialization() {
        use crate::models::Project;
        
        let json = r#"{
            "id": "10000",
            "key": "TEST",
            "name": "Test Project",
            "archived": false
        }"#;
        
        let project: Project = serde_json::from_str(json).expect("Failed to deserialize Project");
        assert_eq!(project.id, Some("10000".to_string()));
        assert_eq!(project.key, Some("TEST".to_string()));
        assert_eq!(project.name, Some("Test Project".to_string()));
        assert_eq!(project.archived, Some(false));
    }

    #[test]
    fn test_user_serialization() {
        use crate::models::User;
        
        let user = User {
            account_id: Some("557058:test-user".to_string()),
            display_name: Some("Test User".to_string()),
            email_address: Some("test@example.com".to_string()),
            active: Some(true),
            ..Default::default()
        };
        
        let json = serde_json::to_string(&user).expect("Failed to serialize User");
        assert!(json.contains("\"accountId\":\"557058:test-user\""));
        assert!(json.contains("\"displayName\":\"Test User\""));
        assert!(json.contains("\"emailAddress\":\"test@example.com\""));
        assert!(json.contains("\"active\":true"));
    }

    #[test]
    fn test_user_deserialization() {
        use crate::models::User;
        
        let json = r#"{
            "accountId": "557058:test-user",
            "displayName": "Test User",
            "emailAddress": "test@example.com",
            "active": true
        }"#;
        
        let user: User = serde_json::from_str(json).expect("Failed to deserialize User");
        assert_eq!(user.account_id, Some("557058:test-user".to_string()));
        assert_eq!(user.display_name, Some("Test User".to_string()));
        assert_eq!(user.email_address, Some("test@example.com".to_string()));
        assert_eq!(user.active, Some(true));
    }

    #[test]
    fn test_error_collection_serialization() {
        use crate::models::ErrorCollection;
        
        let mut errors = std::collections::HashMap::new();
        errors.insert("field1".to_string(), "Invalid value".to_string());
        
        let error = ErrorCollection {
            error_messages: Some(vec!["Invalid input".to_string()]),
            errors: Some(errors),
            status: Some(400),
        };
        
        let json = serde_json::to_string(&error).expect("Failed to serialize ErrorCollection");
        assert!(json.contains("\"errorMessages\":[\"Invalid input\"]"));
        assert!(json.contains("\"status\":400"));
        assert!(json.contains("\"field1\":\"Invalid value\""));
    }

    #[test]
    fn test_error_collection_deserialization() {
        use crate::models::ErrorCollection;
        
        let json = r#"{
            "errorMessages": ["Invalid input"],
            "errors": {
                "field1": "Invalid value"
            },
            "status": 400
        }"#;
        
        let error: ErrorCollection = serde_json::from_str(json).expect("Failed to deserialize ErrorCollection");
        assert_eq!(error.error_messages, Some(vec!["Invalid input".to_string()]));
        assert_eq!(error.status, Some(400));
        
        if let Some(errors) = error.errors {
            assert_eq!(errors.get("field1"), Some(&"Invalid value".to_string()));
        }
    }

    #[cfg(feature = "version_api")]
    #[test]
    fn test_version_serialization() {
        use crate::models::Version;
        
        let version = Version {
            id: Some("10000".to_string()),
            name: Some("1.0.0".to_string()),
            description: Some("Initial release".to_string()),
            released: Some(false),
            archived: Some(false),
            ..Default::default()
        };
        
        let json = serde_json::to_string(&version).expect("Failed to serialize Version");
        assert!(json.contains("\"id\":\"10000\""));
        assert!(json.contains("\"name\":\"1.0.0\""));
        assert!(json.contains("\"description\":\"Initial release\""));
        assert!(json.contains("\"released\":false"));
        assert!(json.contains("\"archived\":false"));
    }

    #[cfg(feature = "version_api")]
    #[test]
    fn test_version_deserialization() {
        use crate::models::Version;
        
        let json = r#"{
            "id": "10000",
            "name": "1.0.0",
            "description": "Initial release",
            "released": false,
            "archived": false
        }"#;
        
        let version: Version = serde_json::from_str(json).expect("Failed to deserialize Version");
        assert_eq!(version.id, Some("10000".to_string()));
        assert_eq!(version.name, Some("1.0.0".to_string()));
        assert_eq!(version.description, Some("Initial release".to_string()));
        assert_eq!(version.released, Some(false));
        assert_eq!(version.archived, Some(false));
    }

    #[test]
    fn test_optional_fields_serialization() {
        use crate::models::User;
        
        let user = User {
            account_id: Some("557058:test-user".to_string()),
            display_name: None,
            email_address: None,
            active: Some(true),
            ..Default::default()
        };
        
        let json = serde_json::to_string(&user).expect("Failed to serialize User");
        assert!(json.contains("\"accountId\":\"557058:test-user\""));
        assert!(json.contains("\"active\":true"));
        // Should not contain None fields
        assert!(!json.contains("\"displayName\""));
        assert!(!json.contains("\"emailAddress\""));
    }

    #[test]
    fn test_round_trip_serialization() {
        use crate::models::Priority;
        
        let original = Priority {
            id: Some("1".to_string()),
            name: Some("Highest".to_string()),
            description: Some("This problem will block progress.".to_string()),
            ..Default::default()
        };
        
        let json = serde_json::to_string(&original).expect("Failed to serialize Priority");
        let deserialized: Priority = serde_json::from_str(&json).expect("Failed to deserialize Priority");
        
        assert_eq!(original, deserialized);
    }
}