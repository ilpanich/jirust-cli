#[cfg(test)]
mod tests {
    use serde_json;

    #[cfg(feature = "issues_api")]
    #[test]
    fn test_issue_bean_creation() {
        use crate::models::IssueBean;
        
        let issue = IssueBean {
            id: Some("10001".to_string()),
            key: Some("TEST-1".to_string()),
            expand: Some("names,schema,operations".to_string()),
            ..Default::default()
        };
        
        assert_eq!(issue.id, Some("10001".to_string()));
        assert_eq!(issue.key, Some("TEST-1".to_string()));
        assert_eq!(issue.expand, Some("names,schema,operations".to_string()));
    }

    #[cfg(feature = "issues_api")]
    #[test]
    fn test_issue_bean_default() {
        use crate::models::IssueBean;
        
        let issue = IssueBean::default();
        
        assert_eq!(issue.id, None);
        assert_eq!(issue.key, None);
        assert_eq!(issue.expand, None);
        assert_eq!(issue.changelog, None);
    }

    #[cfg(feature = "projects_api")]
    #[test]
    fn test_project_creation() {
        use crate::models::Project;
        
        let project = Project {
            id: Some("10000".to_string()),
            key: Some("TEST".to_string()),
            name: Some("Test Project".to_string()),
            archived: Some(false),
            ..Default::default()
        };
        
        assert_eq!(project.id, Some("10000".to_string()));
        assert_eq!(project.key, Some("TEST".to_string()));
        assert_eq!(project.name, Some("Test Project".to_string()));
        assert_eq!(project.archived, Some(false));
    }

    #[cfg(feature = "projects_api")]
    #[test]
    fn test_project_default() {
        use crate::models::Project;
        
        let project = Project::default();
        
        assert_eq!(project.id, None);
        assert_eq!(project.key, None);
        assert_eq!(project.name, None);
        assert_eq!(project.archived, None);
    }

    #[test]
    fn test_user_creation() {
        use crate::models::User;
        
        let user = User {
            account_id: Some("557058:test-user".to_string()),
            display_name: Some("Test User".to_string()),
            email_address: Some("test@example.com".to_string()),
            active: Some(true),
            ..Default::default()
        };
        
        assert_eq!(user.account_id, Some("557058:test-user".to_string()));
        assert_eq!(user.display_name, Some("Test User".to_string()));
        assert_eq!(user.email_address, Some("test@example.com".to_string()));
        assert_eq!(user.active, Some(true));
    }

    #[test]
    fn test_error_collection_creation() {
        use crate::models::ErrorCollection;
        
        let error = ErrorCollection {
            error_messages: Some(vec!["Invalid input".to_string()]),
            errors: Some(std::collections::HashMap::new()),
            status: Some(400),
        };
        
        assert_eq!(error.error_messages, Some(vec!["Invalid input".to_string()]));
        assert_eq!(error.status, Some(400));
    }

    #[cfg(feature = "version_api")]
    #[test]
    fn test_version_creation() {
        use crate::models::Version;
        
        let version = Version {
            id: Some("10000".to_string()),
            name: Some("1.0.0".to_string()),
            description: Some("Initial release".to_string()),
            released: Some(false),
            archived: Some(false),
            ..Default::default()
        };
        
        assert_eq!(version.id, Some("10000".to_string()));
        assert_eq!(version.name, Some("1.0.0".to_string()));
        assert_eq!(version.description, Some("Initial release".to_string()));
        assert_eq!(version.released, Some(false));
        assert_eq!(version.archived, Some(false));
    }

    #[test]
    fn test_priority_creation() {
        use crate::models::Priority;
        
        let priority = Priority {
            id: Some("1".to_string()),
            name: Some("Highest".to_string()),
            description: Some("This problem will block progress.".to_string()),
            ..Default::default()
        };
        
        assert_eq!(priority.id, Some("1".to_string()));
        assert_eq!(priority.name, Some("Highest".to_string()));
        assert_eq!(priority.description, Some("This problem will block progress.".to_string()));
    }

    #[test]
    fn test_status_creation() {
        use crate::models::Status;
        
        let status = Status {
            resolved: Some(false),
            ..Default::default()
        };
        
        assert_eq!(status.resolved, Some(false));
        assert_eq!(status.icon, None);
    }

    #[test]
    fn test_attachment_creation() {
        use crate::models::Attachment;
        
        let attachment = Attachment {
            id: Some("10000".to_string()),
            filename: Some("test.txt".to_string()),
            size: Some(1024),
            mime_type: Some("text/plain".to_string()),
            ..Default::default()
        };
        
        assert_eq!(attachment.id, Some("10000".to_string()));
        assert_eq!(attachment.filename, Some("test.txt".to_string()));
        assert_eq!(attachment.size, Some(1024));
        assert_eq!(attachment.mime_type, Some("text/plain".to_string()));
    }

    #[test]
    fn test_comment_creation() {
        use crate::models::Comment;
        
        let comment = Comment {
            id: Some("10000".to_string()),
            body: Some(Some(serde_json::json!("This is a test comment"))),
            created: Some("2024-01-01T00:00:00.000+0000".to_string()),
            updated: Some("2024-01-01T00:00:00.000+0000".to_string()),
            ..Default::default()
        };
        
        assert_eq!(comment.id, Some("10000".to_string()));
        assert!(comment.body.is_some());
        assert_eq!(comment.created, Some("2024-01-01T00:00:00.000+0000".to_string()));
        assert_eq!(comment.updated, Some("2024-01-01T00:00:00.000+0000".to_string()));
    }
}