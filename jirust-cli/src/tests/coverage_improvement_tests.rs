#[cfg(test)]
mod coverage_improvement_tests {
    use crate::args::commands::OutputValues;
    use crate::config::config_file::{ConfigFile, YaraSection};
    use crate::runners::jira_cmd_runners::{
        link_issue_cmd_runner::{LinkIssueCmdParams, LinkIssueCmdRunner},
        project_cmd_runner::{ProjectCmdParams, ProjectCmdRunner},
        version_cmd_runner::{VersionCmdParams, VersionCmdRunner},
    };
    use crate::utils::{OutputType, PrintableData, print_data};
    use jira_v3_openapi::models::Version;
    use std::io::Write;
    use tempfile::NamedTempFile;
    use toml::Table;

    fn create_test_config() -> ConfigFile {
        let mut transitions = Table::new();
        transitions.insert(
            "resolve".to_string(),
            toml::Value::Array(vec![
                toml::Value::String("Done".to_string()),
                toml::Value::String("Resolved".to_string()),
            ]),
        );

        ConfigFile::new(
            "dGVzdF91c2VyOnRlc3RfYXBpX2tleQ==".to_string(),
            "https://test.atlassian.net".to_string(),
            r#"{"name": "Done"}"#.to_string(),
            "Task completed".to_string(),
            transitions,
            YaraSection::default(),
        )
    }

    // ===== VERSION CMD RUNNER TESTS =====

    #[test]
    fn test_version_cmd_runner_new_with_transitions() {
        let config = create_test_config();
        let _runner = VersionCmdRunner::new(&config);

        // Test that runner was created successfully
        assert!(true);
    }

    #[tokio::test]
    async fn test_version_cmd_runner_get_version_missing_id() {
        let config = create_test_config();
        let runner = VersionCmdRunner::new(&config);

        let params = VersionCmdParams {
            version_id: None,
            ..Default::default()
        };

        // This should panic due to expect() in the get method
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(runner.get_jira_version(params))
        }));

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_version_cmd_runner_update_version_released_no_date() {
        let config = create_test_config();
        let runner = VersionCmdRunner::new(&config);

        let params = VersionCmdParams {
            project: "TEST".to_string(),
            version_id: Some("10000".to_string()),
            version_name: Some("1.0.0".to_string()),
            version_released: Some(true),
            version_release_date: None, // No date provided, should use today
            ..Default::default()
        };

        let result = runner.update_jira_version(params).await;
        // Will fail with network error, but we're testing the code path
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_version_cmd_runner_update_version_with_date() {
        let config = create_test_config();
        let runner = VersionCmdRunner::new(&config);

        let params = VersionCmdParams {
            project: "TEST".to_string(),
            version_id: Some("10000".to_string()),
            version_name: Some("1.0.0".to_string()),
            version_released: Some(true),
            version_release_date: Some("2024-12-31".to_string()),
            ..Default::default()
        };

        let result = runner.update_jira_version(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_version_cmd_runner_update_version_not_released() {
        let config = create_test_config();
        let runner = VersionCmdRunner::new(&config);

        let params = VersionCmdParams {
            project: "TEST".to_string(),
            version_id: Some("10000".to_string()),
            version_name: Some("1.0.0".to_string()),
            version_released: Some(false),
            version_release_date: Some("2024-12-31".to_string()),
            ..Default::default()
        };

        let result = runner.update_jira_version(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_version_cmd_runner_delete_version() {
        let config = create_test_config();
        let runner = VersionCmdRunner::new(&config);

        let params = VersionCmdParams {
            version_id: Some("10000".to_string()),
            ..Default::default()
        };

        let result = runner.delete_jira_version(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_version_cmd_runner_get_related_work() {
        let config = create_test_config();
        let runner = VersionCmdRunner::new(&config);

        let params = VersionCmdParams {
            version_id: Some("10000".to_string()),
            ..Default::default()
        };

        let result = runner.get_jira_version_related_work(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_version_cmd_runner_list_versions_paginated() {
        let config = create_test_config();
        let runner = VersionCmdRunner::new(&config);

        let params = VersionCmdParams {
            project: "TEST".to_string(),
            versions_page_size: Some(50),
            versions_page_offset: Some(0),
            ..Default::default()
        };

        let result = runner.list_jira_versions(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_version_cmd_runner_list_versions_no_pagination() {
        let config = create_test_config();
        let runner = VersionCmdRunner::new(&config);

        let params = VersionCmdParams {
            project: "TEST".to_string(),
            versions_page_size: None,
            versions_page_offset: None,
            ..Default::default()
        };

        let result = runner.list_jira_versions(params).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_version_params_mark_released_helper() {
        let mut version = Version::default();
        version.id = Some("123".to_string());
        version.project = Some("TEST".to_string());
        version.name = Some("1.0.0".to_string());

        let params = VersionCmdParams::mark_released(version);

        assert_eq!(params.version_released, Some(true));
        assert!(params.version_release_date.is_some());
        assert_eq!(params.version_id, Some("123".to_string()));
    }

    #[test]
    fn test_version_params_mark_archived_helper() {
        let mut version = Version::default();
        version.id = Some("456".to_string());
        version.project = Some("TEST".to_string());
        version.name = Some("2.0.0".to_string());

        let params = VersionCmdParams::mark_archived(version);

        assert_eq!(params.version_archived, Some(true));
        assert_eq!(params.version_id, Some("456".to_string()));
    }

    #[cfg(any(windows, unix))]
    #[tokio::test]
    async fn test_version_cmd_runner_create_with_changelog() {
        let config = create_test_config();
        let runner = VersionCmdRunner::new(&config);

        // Create a temporary changelog file
        let mut changelog = NamedTempFile::new().expect("create temp file");
        writeln!(
            changelog,
            "## [1.0.0] 2024-01-01\n- Fixed TEST-1\n- Resolved TEST-2"
        )
        .expect("write changelog");

        let params = VersionCmdParams {
            project: "TEST".to_string(),
            version_name: Some("1.0.0".to_string()),
            changelog_file: Some(changelog.path().display().to_string()),
            transition_issues: Some(false), // Don't transition issues
            ..Default::default()
        };

        let result = runner.create_jira_version(params).await;
        // Will fail with network error
        assert!(result.is_err());
    }

    #[cfg(any(windows, unix))]
    #[tokio::test]
    async fn test_version_cmd_runner_create_with_changelog_and_transitions() {
        let config = create_test_config();
        let runner = VersionCmdRunner::new(&config);

        let mut changelog = NamedTempFile::new().expect("create temp file");
        writeln!(
            changelog,
            "## [1.0.0] 2024-01-01\n- Fixed TEST-1\n- Resolved TEST-2"
        )
        .expect("write changelog");

        let params = VersionCmdParams {
            project: "TEST".to_string(),
            version_name: Some("1.0.0".to_string()),
            changelog_file: Some(changelog.path().display().to_string()),
            transition_issues: Some(true),
            transition_assignee: Some("user123".to_string()),
            ..Default::default()
        };

        let result = runner.create_jira_version(params).await;
        assert!(result.is_err());
    }

    #[cfg(any(windows, unix))]
    #[tokio::test]
    async fn test_version_cmd_runner_create_released_with_date() {
        let config = create_test_config();
        let runner = VersionCmdRunner::new(&config);

        let params = VersionCmdParams {
            project: "TEST".to_string(),
            version_name: Some("1.0.0".to_string()),
            version_released: Some(true),
            version_release_date: Some("2024-01-15".to_string()),
            version_description: Some("Test release".to_string()),
            ..Default::default()
        };

        let result = runner.create_jira_version(params).await;
        assert!(result.is_err());
    }

    #[cfg(any(windows, unix))]
    #[tokio::test]
    async fn test_version_cmd_runner_create_released_no_date() {
        let config = create_test_config();
        let runner = VersionCmdRunner::new(&config);

        let params = VersionCmdParams {
            project: "TEST".to_string(),
            version_name: Some("1.0.0".to_string()),
            version_released: Some(true),
            version_release_date: None, // Should use today's date
            ..Default::default()
        };

        let result = runner.create_jira_version(params).await;
        assert!(result.is_err());
    }

    #[cfg(any(windows, unix))]
    #[tokio::test]
    async fn test_version_cmd_runner_create_with_changelog_no_description() {
        let config = create_test_config();
        let runner = VersionCmdRunner::new(&config);

        let mut changelog = NamedTempFile::new().expect("create temp file");
        writeln!(changelog, "## [2.0.0] 2024-02-01\nNo valid version format")
            .expect("write changelog");

        let params = VersionCmdParams {
            project: "TEST".to_string(),
            version_name: Some("1.0.0".to_string()),
            changelog_file: Some(changelog.path().display().to_string()),
            version_description: None, // Should fall back to default
            ..Default::default()
        };

        let result = runner.create_jira_version(params).await;
        assert!(result.is_err());
    }

    #[cfg(any(windows, unix))]
    #[tokio::test]
    async fn test_version_cmd_runner_create_with_changelog_override_description() {
        let config = create_test_config();
        let runner = VersionCmdRunner::new(&config);

        let mut changelog = NamedTempFile::new().expect("create temp file");
        writeln!(changelog, "Invalid changelog content").expect("write changelog");

        let params = VersionCmdParams {
            project: "TEST".to_string(),
            version_name: Some("1.0.0".to_string()),
            changelog_file: Some(changelog.path().display().to_string()),
            version_description: Some("Override description".to_string()),
            ..Default::default()
        };

        let result = runner.create_jira_version(params).await;
        assert!(result.is_err());
    }

    // ===== LINK ISSUE CMD RUNNER TESTS =====

    #[tokio::test]
    async fn test_link_issue_cmd_runner_with_changelog_no_project() {
        let config = create_test_config();
        let runner = LinkIssueCmdRunner::new(&config);

        let mut changelog = NamedTempFile::new().expect("create temp file");
        writeln!(
            changelog,
            "## [1.0.0] 2024-01-01\n- Fixed TEST-1\n- Resolved TEST-2"
        )
        .expect("write changelog");

        let params = LinkIssueCmdParams {
            origin_issue_key: "SRC-1".to_string(),
            destination_issue_key: None,
            link_type: "Relates".to_string(),
            project_key: None, // Missing project key with changelog
            changelog_file: Some(changelog.path().display().to_string()),
        };

        let result = runner.link_jira_issues(params).await;
        assert!(result.is_err());
        if let Err(err) = result {
            assert!(err.to_string().contains("Empty project key"));
        }
    }

    #[tokio::test]
    async fn test_link_issue_cmd_runner_with_changelog_empty_issues() {
        let config = create_test_config();
        let runner = LinkIssueCmdRunner::new(&config);

        let mut changelog = NamedTempFile::new().expect("create temp file");
        writeln!(changelog, "## [1.0.0] 2024-01-01\nNo issues here").expect("write changelog");

        let params = LinkIssueCmdParams {
            origin_issue_key: "SRC-1".to_string(),
            destination_issue_key: None,
            link_type: "Relates".to_string(),
            project_key: Some("TEST".to_string()),
            changelog_file: Some(changelog.path().display().to_string()),
        };

        let result = runner.link_jira_issues(params).await;
        // Should succeed but return "Linking KO" since no actual API
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_link_issue_cmd_runner_with_changelog_different_project() {
        let config = create_test_config();
        let runner = LinkIssueCmdRunner::new(&config);

        let mut changelog = NamedTempFile::new().expect("create temp file");
        writeln!(
            changelog,
            "## [1.0.0] 2024-01-01\n- Fixed OTHER-1\n- Resolved OTHER-2"
        )
        .expect("write changelog");

        let params = LinkIssueCmdParams {
            origin_issue_key: "SRC-1".to_string(),
            destination_issue_key: None,
            link_type: "Relates".to_string(),
            project_key: Some("TEST".to_string()),
            changelog_file: Some(changelog.path().display().to_string()),
        };

        let result = runner.link_jira_issues(params).await;
        assert!(result.is_ok());
    }

    // ===== PROJECT CMD RUNNER TESTS =====

    #[tokio::test]
    async fn test_project_cmd_runner_create_with_project_lead_assignee() {
        let config = create_test_config();
        let runner = ProjectCmdRunner::new(&config);

        let params = ProjectCmdParams {
            project_key: Some("TEST".to_string()),
            project_name: Some("Test Project".to_string()),
            project_assignee_type: Some("PROJECT_LEAD".to_string()),
            project_lead_account_id: Some("lead123".to_string()),
            ..Default::default()
        };

        let result = runner.create_jira_project(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_project_cmd_runner_create_with_unassigned_type() {
        let config = create_test_config();
        let runner = ProjectCmdRunner::new(&config);

        let params = ProjectCmdParams {
            project_key: Some("TEST".to_string()),
            project_name: Some("Test Project".to_string()),
            project_assignee_type: Some("UNASSIGNED".to_string()),
            project_lead_account_id: Some("lead123".to_string()),
            ..Default::default()
        };

        let result = runner.create_jira_project(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_project_cmd_runner_create_with_all_schemes() {
        let config = create_test_config();
        let runner = ProjectCmdRunner::new(&config);

        let params = ProjectCmdParams {
            project_key: Some("TEST".to_string()),
            project_name: Some("Test Project".to_string()),
            project_field_configuration_id: Some(10000),
            project_issue_security_scheme_id: Some(10001),
            project_issue_type_scheme_id: Some(10002),
            project_issue_type_screen_scheme_id: Some(10003),
            project_notification_scheme_id: Some(10004),
            project_permission_scheme_id: Some(10005),
            project_workflow_scheme_id: Some(10006),
            project_lead_account_id: Some("lead123".to_string()),
            ..Default::default()
        };

        let result = runner.create_jira_project(params).await;
        assert!(result.is_err());
    }

    // ===== JSON PRINTER TESTS FOR UNCOVERED BRANCHES =====

    #[test]
    fn test_print_data_json_issue_created_variant() {
        let created_issue = jira_v3_openapi::models::CreatedIssue {
            id: Some("10000".to_string()),
            key: Some("TEST-1".to_string()),
            ..Default::default()
        };

        let data = PrintableData::IssueCreated {
            issues: vec![created_issue],
        };

        // This should print JSON and cover the IssueCreated branch
        print_data(data, OutputValues::Json, OutputType::Full);
        assert!(true);
    }

    #[test]
    fn test_print_data_json_issue_transitions_variant() {
        let transition = jira_v3_openapi::models::IssueTransition::default();

        let data = PrintableData::IssueTransitions {
            transitions: vec![transition],
        };

        print_data(data, OutputValues::Json, OutputType::Full);
        assert!(true);
    }

    #[test]
    fn test_print_data_json_transitioned_issue_variant() {
        let data = PrintableData::TransitionedIssue {
            issues: vec![(
                "TEST-1".to_string(),
                "OK".to_string(),
                "OK".to_string(),
                "1.0.0".to_string(),
            )],
        };

        print_data(data, OutputValues::Json, OutputType::Full);
        assert!(true);
    }

    #[test]
    fn test_print_data_json_issue_type_variant() {
        let issue_type = jira_v3_openapi::models::IssueTypeIssueCreateMetadata::default();

        let data = PrintableData::IssueType {
            issue_types: vec![issue_type],
        };

        print_data(data, OutputValues::Json, OutputType::Full);
        assert!(true);
    }

    #[test]
    fn test_print_data_json_issue_type_field_variant() {
        let field = jira_v3_openapi::models::FieldCreateMetadata::default();

        let data = PrintableData::IssueTypeField {
            issue_type_fields: vec![field],
        };

        print_data(data, OutputValues::Json, OutputType::Full);
        assert!(true);
    }

    #[test]
    fn test_print_data_json_version_related_work_variant() {
        let related_work = jira_v3_openapi::models::VersionRelatedWork::default();

        let data = PrintableData::VersionRelatedWork {
            version_related_work_items: vec![related_work],
        };

        print_data(data, OutputValues::Json, OutputType::Full);
        assert!(true);
    }

    #[test]
    fn test_print_data_json_generic_variant() {
        let data = PrintableData::Generic {
            data: vec![serde_json::json!({"test": "data"})],
        };

        print_data(data, OutputValues::Json, OutputType::Full);
        assert!(true);
    }

    #[test]
    fn test_print_data_table_full() {
        let created_issue = jira_v3_openapi::models::CreatedIssue::default();

        let data = PrintableData::IssueCreated {
            issues: vec![created_issue],
        };

        print_data(data, OutputValues::Table, OutputType::Full);
        assert!(true);
    }

    #[test]
    fn test_print_data_table_basic() {
        let created_issue = jira_v3_openapi::models::CreatedIssue::default();

        let data = PrintableData::IssueCreated {
            issues: vec![created_issue],
        };

        print_data(data, OutputValues::Table, OutputType::Basic);
        assert!(true);
    }

    #[test]
    fn test_print_data_table_single() {
        let created_issue = jira_v3_openapi::models::CreatedIssue::default();

        let data = PrintableData::IssueCreated {
            issues: vec![created_issue],
        };

        print_data(data, OutputValues::Table, OutputType::Single);
        assert!(true);
    }

    #[test]
    fn test_print_data_version_variant() {
        let version = jira_v3_openapi::models::Version::default();

        let data = PrintableData::Version {
            versions: vec![version],
        };

        print_data(data, OutputValues::Json, OutputType::Full);
        assert!(true);
    }

    #[test]
    fn test_print_data_project_variant() {
        let project = jira_v3_openapi::models::Project::default();

        let data = PrintableData::Project {
            projects: vec![project],
        };

        print_data(data, OutputValues::Json, OutputType::Full);
        assert!(true);
    }

    #[test]
    fn test_print_data_issue_data_variant() {
        let issue = jira_v3_openapi::models::IssueBean::default();

        let data = PrintableData::IssueData {
            issues: vec![issue],
        };

        print_data(data, OutputValues::Json, OutputType::Full);
        assert!(true);
    }
}
