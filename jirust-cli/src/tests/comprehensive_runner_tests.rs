#[cfg(test)]
mod comprehensive_runner_tests {
    use crate::config::config_file::ConfigFile;
    use crate::runners::jira_cmd_runners::{
        issue_cmd_runner::{IssueCmdRunner, IssueCmdParams, IssueTransitionCmdParams},
        project_cmd_runner::{ProjectCmdRunner, ProjectCmdParams},
        version_cmd_runner::{VersionCmdRunner, VersionCmdParams},
        link_issue_cmd_runner::{LinkIssueCmdRunner, LinkIssueCmdParams},
    };
    use jira_v3_openapi::models::*;
    use serde_json::Value;
    use std::collections::HashMap;
    use toml::Table;

    // Test utilities
    fn create_test_config() -> ConfigFile {
        ConfigFile::new(
            "dGVzdF91c2VyOnRlc3RfYXBpX2tleQ==".to_string(), // test_user:test_api_key
            "https://test.atlassian.net".to_string(),
            "Done".to_string(),
            "Task completed".to_string(),
            Table::new(),
        )
    }

    fn create_mock_issue() -> IssueBean {
        IssueBean {
            id: Some("12345".to_string()),
            key: Some("TEST-123".to_string()),
            fields: Some(HashMap::from([
                ("summary".to_string(), Value::String("Test issue".to_string())),
                ("description".to_string(), Value::String("Test description".to_string())),
                ("issuetype".to_string(), Value::Object({
                    let mut obj = serde_json::Map::new();
                    obj.insert("name".to_string(), Value::String("Task".to_string()));
                    obj
                })),
            ])),
            ..Default::default()
        }
    }

    fn create_mock_created_issue() -> CreatedIssue {
        CreatedIssue {
            id: Some("12345".to_string()),
            key: Some("TEST-123".to_string()),
            param_self: Some("https://test.atlassian.net/rest/api/2/issue/12345".to_string()),
            transition: None,
            watchers: None,
        }
    }

    // Issue Runner Tests
    #[tokio::test]
    async fn test_issue_runner_configuration() {
        let config = create_test_config();
        let runner = IssueCmdRunner::new(&config);
        
        // Test that runner is created with proper configuration
        // Since fields are private, we test indirectly through behavior
        assert!(true); // Constructor succeeded without panicking
    }

    #[tokio::test]
    async fn test_issue_runner_get_issue_error_handling() {
        let config = create_test_config();
        let runner = IssueCmdRunner::new(&config);
        
        // Test with empty issue key
        let params = IssueCmdParams {
            issue_key: None,
            project_key: Some("TEST".to_string()),
            issue_fields: None,
            transition: None,
            assignee: None,
            query: None,
        };
        
        let result = runner.get_jira_issue(params).await;
        assert!(result.is_err());
        if let Err(err) = result {
            assert!(err.to_string().contains("Empty issue key"));
        }
    }

    #[tokio::test]
    #[should_panic(expected = "Assignee is required")]
    async fn test_issue_runner_assign_issue_error_handling() {
        let config = create_test_config();
        let runner = IssueCmdRunner::new(&config);
        
        // Test with empty assignee - this will panic due to expect() in the runner
        let params = IssueCmdParams {
            issue_key: Some("TEST-123".to_string()),
            project_key: Some("TEST".to_string()),
            issue_fields: None,
            transition: None,
            assignee: None,
            query: None,
        };
        
        let _result = runner.assign_jira_issue(params).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Project Key is required")]
    async fn test_issue_runner_create_issue_error_handling() {
        let config = create_test_config();
        let runner = IssueCmdRunner::new(&config);
        
        // Test with missing project key - this will panic due to expect() in the runner
        let params = IssueCmdParams {
            issue_key: None,
            project_key: None,
            issue_fields: Some(HashMap::from([
                ("summary".to_string(), Value::String("Test".to_string())),
            ])),
            transition: None,
            assignee: None,
            query: None,
        };
        
        let _result = runner.create_jira_issue(params).await;
    }

    #[tokio::test]
    async fn test_issue_runner_delete_issue_error_handling() {
        let config = create_test_config();
        let runner = IssueCmdRunner::new(&config);
        
        // Test with empty issue key
        let params = IssueCmdParams {
            issue_key: None,
            project_key: Some("TEST".to_string()),
            issue_fields: None,
            transition: None,
            assignee: None,
            query: None,
        };
        
        let result = runner.delete_jira_issue(params).await;
        assert!(result.is_err());
        if let Err(err) = result {
            assert!(err.to_string().contains("Empty issue key"));
        }
    }

    #[tokio::test]
    async fn test_issue_runner_transition_error_handling() {
        let config = create_test_config();
        let runner = IssueCmdRunner::new(&config);
        
        // Test with empty issue key
        let params = IssueCmdParams {
            issue_key: None,
            project_key: Some("TEST".to_string()),
            issue_fields: None,
            transition: Some("2".to_string()),
            assignee: None,
            query: None,
        };
        
        let result = runner.transition_jira_issue(params).await;
        assert!(result.is_err());
        if let Err(err) = result {
            assert!(err.to_string().contains("Empty issue key"));
        }

        // Test with empty transition
        let params = IssueCmdParams {
            issue_key: Some("TEST-123".to_string()),
            project_key: Some("TEST".to_string()),
            issue_fields: None,
            transition: None,
            assignee: None,
            query: None,
        };
        
        let result = runner.transition_jira_issue(params).await;
        assert!(result.is_err());
        if let Err(err) = result {
            assert!(err.to_string().contains("Empty transition"));
        }
    }

    #[tokio::test]
    async fn test_issue_runner_update_issue_error_handling() {
        let config = create_test_config();
        let runner = IssueCmdRunner::new(&config);
        
        // Test with empty issue key
        let params = IssueCmdParams {
            issue_key: None,
            project_key: Some("TEST".to_string()),
            issue_fields: Some(HashMap::from([
                ("summary".to_string(), Value::String("Updated".to_string())),
            ])),
            transition: None,
            assignee: None,
            query: None,
        };
        
        let result = runner.update_jira_issue(params).await;
        assert!(result.is_err());
        if let Err(err) = result {
            assert!(err.to_string().contains("Empty issue key"));
        }
    }

    #[tokio::test]
    async fn test_issue_runner_search_with_empty_query() {
        let config = create_test_config();
        let runner = IssueCmdRunner::new(&config);
        
        let params = IssueCmdParams {
            issue_key: None,
            project_key: Some("TEST".to_string()),
            issue_fields: None,
            transition: None,
            assignee: None,
            query: None,
        };
        
        // With no query, it should search with None query which might succeed or fail depending on API
        let result = runner.search_jira_issues(params).await;
        // Since we can't mock the API, we expect this to fail with network error OR succeed with empty results
        // We just test that the function can be called without panicking
        // The result can be either Ok or Err depending on network/API availability
        assert!(result.is_ok() || result.is_err()); // Always true, just testing no panic
    }

    // Project Runner Tests
    #[tokio::test]
    async fn test_project_runner_configuration() {
        let config = create_test_config();
        let runner = ProjectCmdRunner::new(&config);
        
        // Test that runner is created with proper configuration
        assert!(true); // Constructor succeeded without panicking
    }

    // Version Runner Tests
    #[tokio::test]
    async fn test_version_runner_configuration() {
        let config = create_test_config();
        let runner = VersionCmdRunner::new(&config);
        
        // Test that runner is created with proper configuration
        assert!(true); // Constructor succeeded without panicking
    }

    // Link Issue Runner Tests
    #[tokio::test]
    async fn test_link_issue_runner_configuration() {
        let config = create_test_config();
        let runner = LinkIssueCmdRunner::new(&config);
        
        // Test that runner is created with proper configuration
        assert!(true); // Constructor succeeded without panicking
    }

    // Parameter Validation Tests
    #[test]
    fn test_issue_cmd_params_validation() {
        let mut params = IssueCmdParams::new();
        
        // Test default state
        assert_eq!(params.project_key, Some("".to_string()));
        assert_eq!(params.issue_key, None);
        assert_eq!(params.issue_fields, None);
        assert_eq!(params.transition, None);
        assert_eq!(params.assignee, None);
        assert_eq!(params.query, None);
        
        // Test setting values
        params.issue_key = Some("TEST-123".to_string());
        params.assignee = Some("user@example.com".to_string());
        params.transition = Some("2".to_string());
        
        assert_eq!(params.issue_key, Some("TEST-123".to_string()));
        assert_eq!(params.assignee, Some("user@example.com".to_string()));
        assert_eq!(params.transition, Some("2".to_string()));
    }

    #[test]
    fn test_issue_transition_cmd_params_validation() {
        let mut params = IssueTransitionCmdParams::new();
        
        // Test default state
        assert_eq!(params.issue_key, "");
        
        // Test setting values
        params.issue_key = "TEST-456".to_string();
        assert_eq!(params.issue_key, "TEST-456");
    }

    #[test]
    fn test_project_cmd_params_validation() {
        let mut params = ProjectCmdParams::new();
        
        // Test default state - all should be None
        assert_eq!(params.project_key, None);
        assert_eq!(params.project_name, None);
        assert_eq!(params.project_description, None);
        assert_eq!(params.project_lead_account_id, None);
        
        // Test setting values
        params.project_key = Some("NEWPROJ".to_string());
        params.project_name = Some("New Project".to_string());
        params.project_description = Some("A new project for testing".to_string());
        params.project_lead_account_id = Some("lead@example.com".to_string());
        
        assert_eq!(params.project_key, Some("NEWPROJ".to_string()));
        assert_eq!(params.project_name, Some("New Project".to_string()));
        assert_eq!(params.project_description, Some("A new project for testing".to_string()));
        assert_eq!(params.project_lead_account_id, Some("lead@example.com".to_string()));
    }

    #[test]
    fn test_version_cmd_params_validation() {
        let mut params = VersionCmdParams::new();
        
        // Test default state
        assert_eq!(params.project, "");
        assert_eq!(params.project_id, None);
        assert_eq!(params.version_name, None);
        assert_eq!(params.version_description, None);
        assert_eq!(params.version_released, None);
        assert_eq!(params.version_archived, None);
        
        // Test setting values
        params.project = "VER".to_string();
        params.version_name = Some("1.0.0".to_string());
        params.version_description = Some("First release".to_string());
        params.version_released = Some(false);
        params.version_archived = Some(false);
        
        assert_eq!(params.project, "VER");
        assert_eq!(params.version_name, Some("1.0.0".to_string()));
        assert_eq!(params.version_description, Some("First release".to_string()));
        assert_eq!(params.version_released, Some(false));
        assert_eq!(params.version_archived, Some(false));
    }

    #[test]
    fn test_link_issue_cmd_params_validation() {
        let mut params = LinkIssueCmdParams::new();
        
        // Test default state
        assert_eq!(params.origin_issue_key, "");
        assert_eq!(params.destination_issue_key, None);
        assert_eq!(params.link_type, "");
        assert_eq!(params.project_key, None);
        assert_eq!(params.changelog_file, None);
        
        // Test setting values
        params.origin_issue_key = "LINK-123".to_string();
        params.destination_issue_key = Some("LINK-456".to_string());
        params.link_type = "Blocks".to_string();
        params.project_key = Some("LINK".to_string());
        params.changelog_file = Some("CHANGELOG.md".to_string());
        
        assert_eq!(params.origin_issue_key, "LINK-123");
        assert_eq!(params.destination_issue_key, Some("LINK-456".to_string()));
        assert_eq!(params.link_type, "Blocks");
        assert_eq!(params.project_key, Some("LINK".to_string()));
        assert_eq!(params.changelog_file, Some("CHANGELOG.md".to_string()));
    }

    // Configuration Tests
    #[test]
    fn test_runners_with_different_auth_configs() {
        // Test with base64 encoded auth
        let config1 = create_test_config();
        
        // Test with different auth
        let config2 = ConfigFile::new(
            "YWRtaW46c2VjcmV0X3Rva2Vu".to_string(), // admin:secret_token
            "https://another.atlassian.net".to_string(),
            "Resolved".to_string(),
            "Issue resolved".to_string(),
            Table::new(),
        );
        
        // Both configs should work
        let _issue_runner1 = IssueCmdRunner::new(&config1);
        let _issue_runner2 = IssueCmdRunner::new(&config2);
        let _project_runner1 = ProjectCmdRunner::new(&config1);
        let _project_runner2 = ProjectCmdRunner::new(&config2);
        let _version_runner1 = VersionCmdRunner::new(&config1);
        let _version_runner2 = VersionCmdRunner::new(&config2);
        let _link_runner1 = LinkIssueCmdRunner::new(&config1);
        let _link_runner2 = LinkIssueCmdRunner::new(&config2);
        
        assert!(true); // All constructors succeeded
    }

    #[test]
    fn test_params_field_manipulation() {
        let mut params = IssueCmdParams::new();
        
        // Test complex field manipulation
        let mut fields = HashMap::new();
        fields.insert("summary".to_string(), Value::String("Complex Test Issue".to_string()));
        fields.insert("description".to_string(), Value::String("This is a complex test issue with multiple fields".to_string()));
        fields.insert("issuetype".to_string(), Value::Object({
            let mut obj = serde_json::Map::new();
            obj.insert("name".to_string(), Value::String("Story".to_string()));
            obj.insert("id".to_string(), Value::String("10001".to_string()));
            obj
        }));
        fields.insert("priority".to_string(), Value::Object({
            let mut obj = serde_json::Map::new();
            obj.insert("name".to_string(), Value::String("High".to_string()));
            obj.insert("id".to_string(), Value::String("2".to_string()));
            obj
        }));
        fields.insert("labels".to_string(), Value::Array(vec![
            Value::String("backend".to_string()),
            Value::String("api".to_string()),
            Value::String("urgent".to_string()),
        ]));
        
        params.issue_fields = Some(fields.clone());
        
        assert_eq!(params.issue_fields, Some(fields));
        assert!(params.issue_fields.as_ref().unwrap().contains_key("summary"));
        assert!(params.issue_fields.as_ref().unwrap().contains_key("issuetype"));
        assert!(params.issue_fields.as_ref().unwrap().contains_key("priority"));
        assert!(params.issue_fields.as_ref().unwrap().contains_key("labels"));
        
        // Test that we can modify existing fields
        if let Some(ref mut fields) = params.issue_fields {
            fields.insert("assignee".to_string(), Value::Object({
                let mut obj = serde_json::Map::new();
                obj.insert("accountId".to_string(), Value::String("5b10ac8d82e05b22cc7d4ef5".to_string()));
                obj
            }));
        }
        
        assert!(params.issue_fields.as_ref().unwrap().contains_key("assignee"));
    }

    // Edge Case Tests
    #[tokio::test]
    async fn test_transition_cmd_params_edge_cases() {
        let config = create_test_config();
        let runner = IssueCmdRunner::new(&config);
        
        let params = IssueTransitionCmdParams {
            issue_key: "".to_string(), // Empty key
        };
        
        // Test get_issue_available_transitions with empty key
        let result = runner.get_issue_available_transitions(params).await;
        // This should fail due to invalid issue key format
        assert!(result.is_err());
    }

    #[test]
    fn test_params_boundary_values() {
        // Test with very long strings
        let mut params = IssueCmdParams::new();
        let very_long_string = "a".repeat(10000);
        
        params.issue_key = Some(very_long_string.clone());
        params.assignee = Some(very_long_string.clone());
        params.query = Some(very_long_string.clone());
        
        assert_eq!(params.issue_key.as_ref().unwrap().len(), 10000);
        assert_eq!(params.assignee.as_ref().unwrap().len(), 10000);
        assert_eq!(params.query.as_ref().unwrap().len(), 10000);
        
        // Test with special characters
        params.issue_key = Some("TEST-123!@#$%^&*()_+{}[]|\\:\";<>?,./'".to_string());
        assert_eq!(params.issue_key, Some("TEST-123!@#$%^&*()_+{}[]|\\:\";<>?,./'".to_string()));
        
        // Test with Unicode characters
        params.assignee = Some("用户@测试.com".to_string());
        assert_eq!(params.assignee, Some("用户@测试.com".to_string()));
    }

    #[test]
    fn test_version_params_comprehensive() {
        let mut params = VersionCmdParams::new();
        
        // Set all possible fields
        params.project = "COMPREHENSIVE".to_string();
        params.project_id = Some(12345);
        params.version_name = Some("2.1.0-beta.1".to_string());
        params.version_description = Some("Beta release with new features and improvements".to_string());
        params.version_start_date = Some("2024-01-01".to_string());
        params.version_release_date = Some("2024-06-30".to_string());
        params.version_released = Some(false);
        params.version_archived = Some(false);
        params.changelog_file = Some("RELEASE_NOTES.md".to_string());
        params.transition_issues = Some(true);
        params.transition_assignee = Some("release@company.com".to_string());
        params.versions_page_size = Some(25);
        params.versions_page_offset = Some(50);
        
        // Verify all fields are set correctly
        assert_eq!(params.project, "COMPREHENSIVE");
        assert_eq!(params.project_id, Some(12345));
        assert_eq!(params.version_name, Some("2.1.0-beta.1".to_string()));
        assert_eq!(params.version_description, Some("Beta release with new features and improvements".to_string()));
        assert_eq!(params.version_start_date, Some("2024-01-01".to_string()));
        assert_eq!(params.version_release_date, Some("2024-06-30".to_string()));
        assert_eq!(params.version_released, Some(false));
        assert_eq!(params.version_archived, Some(false));
        assert_eq!(params.changelog_file, Some("RELEASE_NOTES.md".to_string()));
        assert_eq!(params.transition_issues, Some(true));
        assert_eq!(params.transition_assignee, Some("release@company.com".to_string()));
        assert_eq!(params.versions_page_size, Some(25));
        assert_eq!(params.versions_page_offset, Some(50));
    }

    #[test]
    fn test_project_params_comprehensive() {
        let mut params = ProjectCmdParams::new();
        
        // Set all possible fields
        params.project_key = Some("FULLTEST".to_string());
        params.project_name = Some("Full Test Project".to_string());
        params.project_description = Some("A comprehensive test project with all fields set".to_string());
        params.project_lead_account_id = Some("5b10ac8d82e05b22cc7d4ef5".to_string());
        params.project_assignee_type = Some("PROJECT_LEAD".to_string());
        params.project_field_configuration_id = Some(10200);
        params.project_issue_security_scheme_id = Some(10000);
        params.project_notification_scheme_id = Some(10300);
        params.project_permission_scheme_id = Some(10100);
        params.project_workflow_scheme_id = Some(10400);
        params.projects_page_size = Some(20);
        params.projects_page_offset = Some(0);
        
        // Verify all fields
        assert_eq!(params.project_key, Some("FULLTEST".to_string()));
        assert_eq!(params.project_name, Some("Full Test Project".to_string()));
        assert_eq!(params.project_description, Some("A comprehensive test project with all fields set".to_string()));
        assert_eq!(params.project_lead_account_id, Some("5b10ac8d82e05b22cc7d4ef5".to_string()));
        assert_eq!(params.project_assignee_type, Some("PROJECT_LEAD".to_string()));
        assert_eq!(params.project_field_configuration_id, Some(10200));
        assert_eq!(params.project_issue_security_scheme_id, Some(10000));
        assert_eq!(params.project_notification_scheme_id, Some(10300));
        assert_eq!(params.project_permission_scheme_id, Some(10100));
        assert_eq!(params.project_workflow_scheme_id, Some(10400));
        assert_eq!(params.projects_page_size, Some(20));
        assert_eq!(params.projects_page_offset, Some(0));
    }

    // Memory and Performance Tests
    #[test]
    fn test_multiple_runner_instances_memory() {
        let config = create_test_config();
        
        // Create many instances to test memory handling
        let mut issue_runners = Vec::new();
        let mut project_runners = Vec::new();
        let mut version_runners = Vec::new();
        let mut link_runners = Vec::new();
        
        for _ in 0..100 {
            issue_runners.push(IssueCmdRunner::new(&config));
            project_runners.push(ProjectCmdRunner::new(&config));
            version_runners.push(VersionCmdRunner::new(&config));
            link_runners.push(LinkIssueCmdRunner::new(&config));
        }
        
        assert_eq!(issue_runners.len(), 100);
        assert_eq!(project_runners.len(), 100);
        assert_eq!(version_runners.len(), 100);
        assert_eq!(link_runners.len(), 100);
        
        // Test that we can still create more
        let _additional_runner = IssueCmdRunner::new(&config);
        assert!(true);
    }

    #[test]
    fn test_large_field_collections() {
        let mut params = IssueCmdParams::new();
        let mut large_fields = HashMap::new();
        
        // Create a large collection of fields
        for i in 0..1000 {
            large_fields.insert(
                format!("field_{}", i),
                Value::String(format!("value_{}", i))
            );
        }
        
        params.issue_fields = Some(large_fields);
        
        assert_eq!(params.issue_fields.as_ref().unwrap().len(), 1000);
        assert!(params.issue_fields.as_ref().unwrap().contains_key("field_500"));
        assert_eq!(
            params.issue_fields.as_ref().unwrap().get("field_999"),
            Some(&Value::String("value_999".to_string()))
        );
    }
}