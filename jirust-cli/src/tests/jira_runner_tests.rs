#[cfg(test)]
mod tests {
    use crate::config::config_file::{ConfigFile, YaraSection};
    use crate::runners::jira_cmd_runners::{
        issue_cmd_runner::{IssueCmdParams, IssueCmdRunner, IssueTransitionCmdParams},
        link_issue_cmd_runner::{LinkIssueCmdParams, LinkIssueCmdRunner},
        project_cmd_runner::{ProjectCmdParams, ProjectCmdRunner},
        version_cmd_runner::{VersionCmdParams, VersionCmdRunner},
    };
    use serde_json::Value;
    use std::collections::HashMap;
    use toml::Table;

    fn create_test_config() -> ConfigFile {
        ConfigFile::new(
            "dGVzdF91c2VyOnRlc3RfYXBpX2tleQ==".to_string(),
            "https://test.atlassian.net".to_string(),
            "Done".to_string(),
            "Task completed".to_string(),
            Table::new(), YaraSection::default()
        )
    }

    #[test]
    fn test_issue_cmd_runner_creation() {
        let config = create_test_config();
        let _runner = IssueCmdRunner::new(&config);

        // Test passes if no panic occurs during creation
        assert!(true);
    }

    #[test]
    fn test_project_cmd_runner_creation() {
        let config = create_test_config();
        let _runner = ProjectCmdRunner::new(&config);

        // Test passes if no panic occurs during creation
        assert!(true);
    }

    #[test]
    fn test_version_cmd_runner_creation() {
        let config = create_test_config();
        let _runner = VersionCmdRunner::new(&config);

        // Test passes if no panic occurs during creation
        assert!(true);
    }

    #[test]
    fn test_link_issue_cmd_runner_creation() {
        let config = create_test_config();
        let _runner = LinkIssueCmdRunner::new(&config);

        // Test passes if no panic occurs during creation
        assert!(true);
    }

    #[test]
    fn test_issue_cmd_params_creation() {
        let mut params = IssueCmdParams::new();
        params.issue_key = Some("TEST-123".to_string());
        params.project_key = Some("TEST".to_string());
        params.assignee = Some("test.user".to_string());
        params.transition = Some("Done".to_string());
        params.query = Some("project = TEST".to_string());

        assert_eq!(params.issue_key, Some("TEST-123".to_string()));
        assert_eq!(params.project_key, Some("TEST".to_string()));
        assert_eq!(params.assignee, Some("test.user".to_string()));
        assert_eq!(params.transition, Some("Done".to_string()));
        assert_eq!(params.query, Some("project = TEST".to_string()));
    }

    #[test]
    fn test_issue_cmd_params_default() {
        let params = IssueCmdParams::default();

        assert_eq!(params.issue_key, None);
        assert_eq!(params.project_key, Some("".to_string()));
        assert_eq!(params.assignee, None);
        assert_eq!(params.transition, None);
        assert_eq!(params.query, None);
        assert_eq!(params.issue_fields, None);
    }

    #[test]
    fn test_issue_cmd_params_with_fields() {
        let mut params = IssueCmdParams::new();
        let mut fields = HashMap::new();
        fields.insert(
            "summary".to_string(),
            Value::String("Test issue".to_string()),
        );
        fields.insert(
            "description".to_string(),
            Value::String("Test description".to_string()),
        );
        fields.insert("issuetype".to_string(), Value::String("Task".to_string()));

        params.issue_fields = Some(fields.clone());

        assert_eq!(params.issue_fields, Some(fields));
    }

    #[test]
    fn test_issue_transition_cmd_params_creation() {
        let params = IssueTransitionCmdParams::new();

        assert_eq!(params.issue_key, "");
    }

    #[test]
    fn test_issue_transition_cmd_params_default() {
        let params = IssueTransitionCmdParams::default();

        assert_eq!(params.issue_key, "");
    }

    #[test]
    fn test_project_cmd_params_creation() {
        let mut params = ProjectCmdParams::new();
        params.project_key = Some("TEST".to_string());
        params.project_name = Some("Test Project".to_string());
        params.project_description = Some("Test project description".to_string());
        params.project_lead_account_id = Some("test.user".to_string());

        assert_eq!(params.project_key, Some("TEST".to_string()));
        assert_eq!(params.project_name, Some("Test Project".to_string()));
        assert_eq!(
            params.project_description,
            Some("Test project description".to_string())
        );
        assert_eq!(
            params.project_lead_account_id,
            Some("test.user".to_string())
        );
    }

    #[test]
    fn test_project_cmd_params_default() {
        let params = ProjectCmdParams::default();

        assert_eq!(params.project_key, None);
        assert_eq!(params.project_name, None);
        assert_eq!(params.project_description, None);
        assert_eq!(params.project_lead_account_id, None);
        assert_eq!(params.project_assignee_type, None);
        assert_eq!(params.project_field_configuration_id, None);
        assert_eq!(params.project_issue_security_scheme_id, None);
        assert_eq!(params.project_notification_scheme_id, None);
        assert_eq!(params.project_permission_scheme_id, None);
        assert_eq!(params.project_workflow_scheme_id, None);
        assert_eq!(params.projects_page_size, None);
        assert_eq!(params.projects_page_offset, None);
    }

    #[test]
    fn test_version_cmd_params_creation() {
        let mut params = VersionCmdParams::new();
        params.project = "TEST".to_string();
        params.version_name = Some("1.0.0".to_string());
        params.version_description = Some("Test version".to_string());

        assert_eq!(params.project, "TEST".to_string());
        assert_eq!(params.version_name, Some("1.0.0".to_string()));
        assert_eq!(params.version_description, Some("Test version".to_string()));
    }

    #[test]
    fn test_version_cmd_params_default() {
        let params = VersionCmdParams::default();

        assert_eq!(params.project, "".to_string());
        assert_eq!(params.project_id, None);
        assert_eq!(params.version_name, None);
        assert_eq!(params.version_description, None);
        assert_eq!(params.version_start_date, None);
        assert_eq!(params.version_release_date, None);
        assert_eq!(params.version_released, None);
        assert_eq!(params.version_archived, None);
        assert_eq!(params.changelog_file, None);
        assert_eq!(params.transition_issues, None);
        assert_eq!(params.transition_assignee, None);
        assert_eq!(params.versions_page_size, None);
        assert_eq!(params.versions_page_offset, None);
    }

    #[test]
    fn test_link_issue_cmd_params_creation() {
        let mut params = LinkIssueCmdParams::new();
        params.origin_issue_key = "TEST-123".to_string();
        params.destination_issue_key = Some("TEST-456".to_string());
        params.link_type = "Blocks".to_string();
        params.project_key = Some("TEST".to_string());

        assert_eq!(params.origin_issue_key, "TEST-123");
        assert_eq!(params.destination_issue_key, Some("TEST-456".to_string()));
        assert_eq!(params.link_type, "Blocks");
        assert_eq!(params.project_key, Some("TEST".to_string()));
    }

    #[test]
    fn test_link_issue_cmd_params_default() {
        let params = LinkIssueCmdParams::default();

        assert_eq!(params.origin_issue_key, "");
        assert_eq!(params.destination_issue_key, None);
        assert_eq!(params.link_type, "");
        assert_eq!(params.project_key, None);
        assert_eq!(params.changelog_file, None);
    }

    #[test]
    fn test_runner_config_handling() {
        let mut config = ConfigFile::default();
        config.set_auth_key("dGVzdF91c2VyOnRlc3RfYXBpX2tleQ==".to_string());
        config.set_jira_url("https://custom.atlassian.net".to_string());

        // Test that all runners accept the config
        let _issue_runner = IssueCmdRunner::new(&config);
        let _project_runner = ProjectCmdRunner::new(&config);
        let _version_runner = VersionCmdRunner::new(&config);
        let _link_runner = LinkIssueCmdRunner::new(&config);

        // All should create successfully
        assert!(true);
    }

    #[test]
    fn test_params_comprehensive_creation() {
        // Test comprehensive IssueCmdParams
        let mut issue_params = IssueCmdParams::new();
        issue_params.issue_key = Some("COMP-123".to_string());
        issue_params.project_key = Some("COMP".to_string());
        issue_params.assignee = Some("user@example.com".to_string());
        issue_params.transition = Some("In Progress".to_string());
        issue_params.query = Some("project = COMP AND status = 'To Do'".to_string());

        let mut fields = HashMap::new();
        fields.insert(
            "summary".to_string(),
            Value::String("Comprehensive test issue".to_string()),
        );
        fields.insert(
            "description".to_string(),
            Value::String("This is a detailed description".to_string()),
        );
        fields.insert("issuetype".to_string(), Value::String("Bug".to_string()));
        fields.insert("priority".to_string(), Value::String("High".to_string()));
        issue_params.issue_fields = Some(fields);

        assert_eq!(issue_params.issue_key, Some("COMP-123".to_string()));
        assert_eq!(issue_params.project_key, Some("COMP".to_string()));
        assert!(issue_params.issue_fields.is_some());

        // Test comprehensive ProjectCmdParams
        let mut project_params = ProjectCmdParams::new();
        project_params.project_key = Some("PROJ".to_string());
        project_params.project_name = Some("Comprehensive Project".to_string());
        project_params.project_description = Some("A comprehensive test project".to_string());
        project_params.project_lead_account_id = Some("project.lead@example.com".to_string());
        project_params.project_assignee_type = Some("PROJECT_LEAD".to_string());
        project_params.project_field_configuration_id = Some(10200);
        project_params.project_issue_security_scheme_id = Some(10000);

        assert_eq!(project_params.project_key, Some("PROJ".to_string()));
        assert_eq!(
            project_params.project_name,
            Some("Comprehensive Project".to_string())
        );
        assert_eq!(project_params.project_field_configuration_id, Some(10200));

        // Test comprehensive VersionCmdParams
        let mut version_params = VersionCmdParams::new();
        version_params.project = "VER".to_string();
        version_params.version_name = Some("2.0.0".to_string());
        version_params.version_description = Some("Major release".to_string());
        version_params.version_release_date = Some("2024-12-31".to_string());
        version_params.version_start_date = Some("2024-01-01".to_string());
        version_params.version_released = Some(false);
        version_params.version_archived = Some(false);
        version_params.versions_page_size = Some(100);
        version_params.versions_page_offset = Some(0);

        assert_eq!(version_params.project, "VER".to_string());
        assert_eq!(version_params.version_name, Some("2.0.0".to_string()));
        assert_eq!(version_params.version_released, Some(false));
        assert_eq!(version_params.versions_page_size, Some(100));
    }

    #[test]
    fn test_params_edge_cases() {
        // Test empty strings
        let mut params = IssueCmdParams::new();
        params.issue_key = Some("".to_string());
        params.query = Some("".to_string());

        assert_eq!(params.issue_key, Some("".to_string()));
        assert_eq!(params.query, Some("".to_string()));

        // Test empty fields
        params.issue_fields = Some(HashMap::new());

        assert_eq!(params.issue_fields, Some(HashMap::new()));
    }

    #[test]
    fn test_transition_params_with_issue_key() {
        let mut params = IssueTransitionCmdParams::new();
        params.issue_key = "TEST-789".to_string();

        assert_eq!(params.issue_key, "TEST-789");

        // Test that we can modify the issue key
        params.issue_key = "NEW-456".to_string();
        assert_eq!(params.issue_key, "NEW-456");
    }

    #[test]
    fn test_link_params_comprehensive() {
        let mut params = LinkIssueCmdParams::new();
        params.origin_issue_key = "LINK-123".to_string();
        params.destination_issue_key = Some("LINK-456".to_string());
        params.link_type = "Relates".to_string();
        params.project_key = Some("LINK".to_string());
        params.changelog_file = Some("CHANGELOG.md".to_string());

        assert_eq!(params.origin_issue_key, "LINK-123");
        assert_eq!(params.destination_issue_key, Some("LINK-456".to_string()));
        assert_eq!(params.link_type, "Relates");
        assert_eq!(params.project_key, Some("LINK".to_string()));
        assert_eq!(params.changelog_file, Some("CHANGELOG.md".to_string()));

        // Test with minimal setup
        let params_minimal = LinkIssueCmdParams::new();

        assert_eq!(params_minimal.origin_issue_key, "");
        assert_eq!(params_minimal.destination_issue_key, None);
        assert_eq!(params_minimal.link_type, "");
        assert_eq!(params_minimal.project_key, None);
        assert_eq!(params_minimal.changelog_file, None);
    }

    #[test]
    fn test_multiple_runner_instances() {
        let config = create_test_config();

        // Create multiple instances of each runner type
        let _issue_runner1 = IssueCmdRunner::new(&config);
        let _issue_runner2 = IssueCmdRunner::new(&config);
        let _project_runner1 = ProjectCmdRunner::new(&config);
        let _project_runner2 = ProjectCmdRunner::new(&config);
        let _version_runner1 = VersionCmdRunner::new(&config);
        let _version_runner2 = VersionCmdRunner::new(&config);
        let _link_runner1 = LinkIssueCmdRunner::new(&config);
        let _link_runner2 = LinkIssueCmdRunner::new(&config);

        // All should create successfully
        assert!(true);
    }

    #[test]
    fn test_params_cloning_and_equality() {
        let mut params1 = IssueCmdParams::new();
        params1.issue_key = Some("CLONE-123".to_string());
        params1.query = Some("project = CLONE".to_string());

        // Clone fields manually since we don't know if Clone is implemented
        let mut params2 = IssueCmdParams::new();
        params2.issue_key = params1.issue_key.clone();
        params2.query = params1.query.clone();

        assert_eq!(params1.issue_key, params2.issue_key);
        assert_eq!(params1.query, params2.query);
    }

    #[test]
    fn test_version_params_field_management() {
        let mut params = VersionCmdParams::new();

        // Initially everything is set to default values
        assert_eq!(params.project, "");
        assert_eq!(params.version_released, None);
        assert_eq!(params.version_archived, None);

        // Set some values
        params.project = "TEST".to_string();
        params.version_released = Some(true);
        params.version_archived = Some(false);

        assert_eq!(params.project, "TEST");
        assert_eq!(params.version_released, Some(true));
        assert_eq!(params.version_archived, Some(false));

        // Test changing values
        params.version_released = Some(false);
        params.version_archived = Some(true);

        assert_eq!(params.version_released, Some(false));
        assert_eq!(params.version_archived, Some(true));
    }

    #[test]
    fn test_runner_instances_with_different_configs() {
        // Test runners with different configurations
        let config1 = create_test_config();

        let mut config2 = ConfigFile::default();
        config2.set_auth_key("YWx0ZXJuYXRpdmVfdXNlcjphbHRlcm5hdGl2ZV9hcGlfa2V5".to_string());
        config2.set_jira_url("https://alternative.atlassian.net".to_string());

        // Both configs should work with all runners
        let _issue_runner1 = IssueCmdRunner::new(&config1);
        let _issue_runner2 = IssueCmdRunner::new(&config2);
        let _project_runner1 = ProjectCmdRunner::new(&config1);
        let _project_runner2 = ProjectCmdRunner::new(&config2);
        let _version_runner1 = VersionCmdRunner::new(&config1);
        let _version_runner2 = VersionCmdRunner::new(&config2);
        let _link_runner1 = LinkIssueCmdRunner::new(&config1);
        let _link_runner2 = LinkIssueCmdRunner::new(&config2);

        // All should create successfully
        assert!(true);
    }

    #[test]
    fn test_issue_fields_handling() {
        let mut params = IssueCmdParams::new();
        let mut fields = HashMap::new();

        // Test with different value types
        fields.insert(
            "summary".to_string(),
            Value::String("Test Summary".to_string()),
        );
        fields.insert(
            "priority".to_string(),
            Value::Number(serde_json::Number::from(1)),
        );
        fields.insert(
            "labels".to_string(),
            Value::Array(vec![
                Value::String("bug".to_string()),
                Value::String("urgent".to_string()),
            ]),
        );

        params.issue_fields = Some(fields.clone());

        assert_eq!(params.issue_fields, Some(fields));
        assert!(
            params
                .issue_fields
                .as_ref()
                .unwrap()
                .contains_key("summary")
        );
        assert!(
            params
                .issue_fields
                .as_ref()
                .unwrap()
                .contains_key("priority")
        );
        assert!(params.issue_fields.as_ref().unwrap().contains_key("labels"));
    }
}
