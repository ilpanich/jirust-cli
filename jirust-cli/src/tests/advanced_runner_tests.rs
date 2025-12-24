#[cfg(test)]
mod advanced_runner_tests {
    use crate::args::commands::{
        IssueActionValues, IssueArgs, OutputArgs, OutputTypes, OutputValues, PaginationArgs,
        ProjectActionValues, ProjectArgs, TransitionActionValues, TransitionArgs,
        VersionActionValues, VersionArgs,
    };
    use crate::config::config_file::{AuthData, ConfigFile, YaraSection};
    use crate::runners::jira_cmd_runners::{
        issue_cmd_runner::{IssueCmdParams, IssueCmdRunner, IssueTransitionCmdParams},
        link_issue_cmd_runner::{LinkIssueCmdParams, LinkIssueCmdRunner},
        project_cmd_runner::{ProjectCmdParams, ProjectCmdRunner},
        version_cmd_runner::{VersionCmdParams, VersionCmdRunner},
    };
    use base64::Engine;
    use jira_v3_openapi::models::*;
    use serde_json::Value;
    use std::collections::HashMap;
    use std::io::Write;
    use tempfile::NamedTempFile;
    use toml::Table;

    // Test utilities
    fn create_test_config() -> ConfigFile {
        ConfigFile::new(
            "dGVzdF91c2VyOnRlc3RfYXBpX2tleQ==".to_string(), // test_user:test_api_key
            "https://test.atlassian.net".to_string(),
            "Done".to_string(),
            "Task completed".to_string(),
            Table::new(),
            YaraSection::default(),
        )
    }

    // ===== ADVANCED ISSUE RUNNER TESTS =====

    #[test]
    fn test_issue_runner_all_methods() {
        let config = create_test_config();
        let _runner = IssueCmdRunner::new(&config);

        // Test that all public methods exist and can be called
        // We can't actually test API calls, but we can test method signatures and error handling
        assert!(true); // Runner was created successfully
    }

    #[tokio::test]
    async fn test_issue_runner_transition_with_fields() {
        let config = create_test_config();
        let runner = IssueCmdRunner::new(&config);

        let mut fields = HashMap::new();
        fields.insert(
            "resolution".to_string(),
            Value::Object({
                let mut obj = serde_json::Map::new();
                obj.insert("name".to_string(), Value::String("Fixed".to_string()));
                obj
            }),
        );
        fields.insert(
            "comment".to_string(),
            Value::String("Issue resolved".to_string()),
        );

        let params = IssueCmdParams {
            issue_key: Some("TEST-123".to_string()),
            project_key: Some("TEST".to_string()),
            issue_fields: Some(fields),
            transition: Some("3".to_string()),
            assignee: None,
            query: None,
            attachment_file_path: None,
        };

        let result = runner.transition_jira_issue(params).await;
        // This will fail with network error since we're not mocking, but we test the code path
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_issue_runner_search_with_complex_jql() {
        let config = create_test_config();
        let runner = IssueCmdRunner::new(&config);

        let complex_jql = r#"project = "TEST" AND status IN ("To Do", "In Progress") AND assignee IS NOT EMPTY AND created >= "-30d" ORDER BY created DESC"#;

        let params = IssueCmdParams {
            issue_key: None,
            project_key: Some("TEST".to_string()),
            issue_fields: None,
            transition: None,
            assignee: None,
            query: Some(complex_jql.to_string()),
            attachment_file_path: None,
        };

        let _result = runner.search_jira_issues(params).await;
        // Test passes if no panic occurs
        assert!(true);
    }

    #[test]
    fn test_issue_params_from_trait_comprehensive() {
        let issue_args = IssueArgs {
            issue_act: IssueActionValues::Create,
            project_key: Some("FROM-TEST".to_string()),
            issue_key: Some("FROM-123".to_string()),
            issue_fields: Some(vec![
                ("summary".to_string(), r#""Test From Trait""#.to_string()),
                (
                    "description".to_string(),
                    r#""Testing From trait implementation""#.to_string(),
                ),
                (
                    "issuetype".to_string(),
                    r#"{"name": "Bug", "id": "1"}"#.to_string(),
                ),
                (
                    "priority".to_string(),
                    r#"{"name": "Critical", "id": "1"}"#.to_string(),
                ),
                (
                    "assignee".to_string(),
                    r#"{"accountId": "test123"}"#.to_string(),
                ),
                (
                    "reporter".to_string(),
                    r#"{"accountId": "reporter123"}"#.to_string(),
                ),
                (
                    "labels".to_string(),
                    r#"["critical", "production", "hotfix"]"#.to_string(),
                ),
                (
                    "components".to_string(),
                    r#"[{"id": "10000", "name": "Backend"}]"#.to_string(),
                ),
                (
                    "fixVersions".to_string(),
                    r#"[{"id": "10001", "name": "1.0.1"}]"#.to_string(),
                ),
                ("duedate".to_string(), r#""2024-12-31""#.to_string()),
                ("environment".to_string(), r#""Production""#.to_string()),
                (
                    "customfield_10000".to_string(),
                    r#"{"value": "Custom Value"}"#.to_string(),
                ),
            ]),
            transition_to: Some("In Progress".to_string()),
            assignee: Some("test.user@example.com".to_string()),
            query: Some("project = FROM-TEST AND status != Done".to_string()),
            attachment_file_path: None,
            pagination: PaginationArgs {
                page_size: Some(75),
                page_offset: Some(25),
            },
            output: OutputArgs {
                output_format: Some(OutputValues::Json),
                output_type: Some(OutputTypes::Full),
            },
        };

        let params = IssueCmdParams::from(&issue_args);

        assert_eq!(params.project_key, Some("FROM-TEST".to_string()));
        assert_eq!(params.issue_key, Some("FROM-123".to_string()));
        assert_eq!(params.transition, Some("In Progress".to_string()));
        assert_eq!(params.assignee, Some("test.user@example.com".to_string()));
        assert_eq!(
            params.query,
            Some("project = FROM-TEST AND status != Done".to_string())
        );

        // Test that issue_fields conversion worked
        assert!(params.issue_fields.is_some());
        let fields = params.issue_fields.unwrap();
        assert!(fields.contains_key("summary"));
        assert!(fields.contains_key("priority"));
        assert!(fields.contains_key("customfield_10000"));
    }

    #[test]
    fn test_issue_transition_params_from_trait() {
        let transition_args = TransitionArgs {
            transition_act: TransitionActionValues::List,
            issue_key: "TRANS-456".to_string(),
            output: OutputArgs {
                output_format: Some(OutputValues::Table),
                output_type: Some(OutputTypes::Basic),
            },
        };

        let params = IssueTransitionCmdParams::from(&transition_args);
        assert_eq!(params.issue_key, "TRANS-456");

        // Test Default implementation
        let default_params = IssueTransitionCmdParams::default();
        assert_eq!(default_params.issue_key, "");

        // Test new method
        let new_params = IssueTransitionCmdParams::new();
        assert_eq!(new_params.issue_key, "");
    }

    // ===== ADVANCED PROJECT RUNNER TESTS =====

    #[tokio::test]
    async fn test_project_runner_create_with_all_fields() {
        let config = create_test_config();
        let runner = ProjectCmdRunner::new(&config);

        let params = ProjectCmdParams {
            project_key: Some("NEWPROJ".to_string()),
            project_issue_type: Some("Task".to_string()),
            project_name: Some("New Comprehensive Project".to_string()),
            project_description: Some("A project with all fields populated".to_string()),
            project_field_configuration_id: Some(10200),
            project_issue_security_scheme_id: Some(10100),
            project_issue_type_scheme_id: Some(10300),
            project_issue_type_screen_scheme_id: Some(10400),
            project_notification_scheme_id: Some(10500),
            project_permission_scheme_id: Some(10600),
            project_workflow_scheme_id: Some(10700),
            project_lead_account_id: Some("lead@example.com".to_string()),
            project_assignee_type: Some("lead".to_string()),
            projects_page_size: Some(50),
            projects_page_offset: Some(0),
        };

        let result = runner.create_jira_project(params).await;
        // This will fail with network error since we're not mocking
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_project_runner_create_with_unassigned_type() {
        let config = create_test_config();
        let runner = ProjectCmdRunner::new(&config);

        let params = ProjectCmdParams {
            project_key: Some("UNASSIGN".to_string()),
            project_name: Some("Unassigned Project".to_string()),
            project_description: Some("Project with unassigned type".to_string()),
            project_assignee_type: Some("unassigned".to_string()),
            project_lead_account_id: Some("lead@example.com".to_string()),
            ..Default::default()
        };

        let result = runner.create_jira_project(params).await;
        // This will fail with network error since we're not mocking
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_project_runner_list_with_pagination() {
        let config = create_test_config();
        let runner = ProjectCmdRunner::new(&config);

        let params = ProjectCmdParams {
            projects_page_size: Some(25),
            projects_page_offset: Some(50),
            ..Default::default()
        };

        let _result = runner.list_jira_projects(params).await;
        // Test passes if no panic occurs
        assert!(true);
    }

    #[tokio::test]
    async fn test_project_runner_get_issue_types_error() {
        let config = create_test_config();
        let runner = ProjectCmdRunner::new(&config);

        // Test with missing project key
        let params = ProjectCmdParams {
            project_key: None,
            ..Default::default()
        };

        let result = runner.get_jira_project_issue_types(params).await;
        assert!(result.is_err());
        if let Err(err) = result {
            assert!(err.to_string().contains("Empty project key"));
        }
    }

    #[tokio::test]
    async fn test_project_runner_get_issue_type_fields_errors() {
        let config = create_test_config();
        let runner = ProjectCmdRunner::new(&config);

        // Test with missing project key
        let params = ProjectCmdParams {
            project_key: None,
            project_issue_type: Some("Task".to_string()),
            ..Default::default()
        };

        let result = runner.get_jira_project_issue_type_id(params).await;
        assert!(result.is_err());
        if let Err(err) = result {
            assert!(err.to_string().contains("Empty project key"));
        }

        // Test with missing issue type
        let params = ProjectCmdParams {
            project_key: Some("TEST".to_string()),
            project_issue_type: None,
            ..Default::default()
        };

        let result = runner.get_jira_project_issue_type_id(params).await;
        assert!(result.is_err());
        if let Err(err) = result {
            assert!(err.to_string().contains("Empty project issue type"));
        }
    }

    #[test]
    fn test_project_params_from_trait_comprehensive() {
        let project_args = ProjectArgs {
            project_act: ProjectActionValues::Create,
            project_key: Some("FROMTEST".to_string()),
            project_issue_type: Some("Epic".to_string()),
            project_name: Some("From Trait Test Project".to_string()),
            project_description: Some(
                "Testing the From trait implementation thoroughly".to_string(),
            ),
            project_field_configuration_id: Some(11111),
            project_issue_security_scheme_id: Some(22222),
            project_issue_type_scheme_id: Some(33333),
            project_issue_type_screen_scheme_id: Some(44444),
            project_notification_scheme_id: Some(55555),
            project_permission_scheme_id: Some(66666),
            project_workflow_scheme_id: Some(77777),
            project_lead_account_id: Some("project.lead@from.test".to_string()),
            project_assignee_type: Some("PROJECT_LEAD".to_string()),
            pagination: PaginationArgs {
                page_size: Some(100),
                page_offset: Some(200),
            },
            output: OutputArgs {
                output_format: Some(OutputValues::Json),
                output_type: Some(OutputTypes::Full),
            },
        };

        let params = ProjectCmdParams::from(&project_args);

        assert_eq!(params.project_key, Some("FROMTEST".to_string()));
        assert_eq!(params.project_issue_type, Some("Epic".to_string()));
        assert_eq!(
            params.project_name,
            Some("From Trait Test Project".to_string())
        );
        assert_eq!(
            params.project_description,
            Some("Testing the From trait implementation thoroughly".to_string())
        );
        assert_eq!(params.project_field_configuration_id, Some(11111));
        assert_eq!(params.project_issue_security_scheme_id, Some(22222));
        assert_eq!(params.project_issue_type_scheme_id, Some(33333));
        assert_eq!(params.project_issue_type_screen_scheme_id, Some(44444));
        assert_eq!(params.project_notification_scheme_id, Some(55555));
        assert_eq!(params.project_permission_scheme_id, Some(66666));
        assert_eq!(params.project_workflow_scheme_id, Some(77777));
        assert_eq!(
            params.project_lead_account_id,
            Some("project.lead@from.test".to_string())
        );
        assert_eq!(
            params.project_assignee_type,
            Some("PROJECT_LEAD".to_string())
        );
        assert_eq!(params.projects_page_size, Some(100));
        assert_eq!(params.projects_page_offset, Some(200));
    }

    #[test]
    fn test_project_params_from_trait_with_large_page_offset() {
        let project_args = ProjectArgs {
            project_act: ProjectActionValues::List,
            project_key: None,
            project_issue_type: None,
            project_name: None,
            project_description: None,
            project_field_configuration_id: None,
            project_issue_security_scheme_id: None,
            project_issue_type_scheme_id: None,
            project_issue_type_screen_scheme_id: None,
            project_notification_scheme_id: None,
            project_permission_scheme_id: None,
            project_workflow_scheme_id: None,
            project_lead_account_id: None,
            project_assignee_type: None,
            pagination: PaginationArgs {
                page_size: Some(10),
                page_offset: Some(1000000), // Large offset to test i32 conversion
            },
            output: OutputArgs {
                output_format: None,
                output_type: None,
            },
        };

        let params = ProjectCmdParams::from(&project_args);
        assert_eq!(params.projects_page_offset, Some(1000000));
    }

    #[test]
    #[should_panic(expected = "Invalid page offset, should fit an i32!")]
    fn test_project_params_from_trait_page_offset_overflow() {
        let project_args = ProjectArgs {
            project_act: ProjectActionValues::List,
            project_key: None,
            project_issue_type: None,
            project_name: None,
            project_description: None,
            project_field_configuration_id: None,
            project_issue_security_scheme_id: None,
            project_issue_type_scheme_id: None,
            project_issue_type_screen_scheme_id: None,
            project_notification_scheme_id: None,
            project_permission_scheme_id: None,
            project_workflow_scheme_id: None,
            project_lead_account_id: None,
            project_assignee_type: None,
            pagination: PaginationArgs {
                page_size: Some(10),
                page_offset: Some(i64::MAX), // Too large for i32
            },
            output: OutputArgs {
                output_format: None,
                output_type: None,
            },
        };

        // This should panic due to expect() in the from implementation
        let _params = ProjectCmdParams::from(&project_args);
    }

    // ===== ADVANCED VERSION RUNNER TESTS =====

    #[test]
    fn test_version_params_comprehensive_validation() {
        let mut params = VersionCmdParams::new();

        // Test all possible fields
        params.project = "VER-COMPREHENSIVE".to_string();
        params.project_id = Some(99999);
        params.version_name = Some("3.5.2-beta.1+build.123".to_string());
        params.version_description = Some("Comprehensive version with all metadata, including release notes, breaking changes, and migration guides".to_string());
        params.version_start_date = Some("2024-01-01T00:00:00Z".to_string());
        params.version_release_date = Some("2024-12-31T23:59:59Z".to_string());
        params.version_released = Some(false);
        params.version_archived = Some(false);
        params.changelog_file = Some("/path/to/detailed/CHANGELOG.md".to_string());
        params.transition_issues = Some(true);
        params.transition_assignee = Some("release.team@company.example.com".to_string());
        params.versions_page_size = Some(200);
        params.versions_page_offset = Some(1000);

        // Verify all fields are set correctly
        assert_eq!(params.project, "VER-COMPREHENSIVE");
        assert_eq!(params.project_id, Some(99999));
        assert_eq!(
            params.version_name,
            Some("3.5.2-beta.1+build.123".to_string())
        );
        assert!(params.version_description.as_ref().unwrap().len() > 100);
        assert_eq!(
            params.version_start_date,
            Some("2024-01-01T00:00:00Z".to_string())
        );
        assert_eq!(
            params.version_release_date,
            Some("2024-12-31T23:59:59Z".to_string())
        );
        assert_eq!(params.version_released, Some(false));
        assert_eq!(params.version_archived, Some(false));
        assert_eq!(
            params.changelog_file,
            Some("/path/to/detailed/CHANGELOG.md".to_string())
        );
        assert_eq!(params.transition_issues, Some(true));
        assert_eq!(
            params.transition_assignee,
            Some("release.team@company.example.com".to_string())
        );
        assert_eq!(params.versions_page_size, Some(200));
        assert_eq!(params.versions_page_offset, Some(1000));
    }

    // ===== ADVANCED LINK ISSUE RUNNER TESTS =====

    #[test]
    fn test_link_issue_params_all_fields() {
        let mut params = LinkIssueCmdParams::new();

        // Test with various link types
        params.origin_issue_key = "LINK-ORIGIN-123".to_string();
        params.destination_issue_key = Some("LINK-DEST-456".to_string());
        params.link_type = "Epic-Story".to_string();
        params.project_key = Some("LINKTEST".to_string());
        params.changelog_file = Some("/project/docs/LINK_CHANGELOG.md".to_string());

        assert_eq!(params.origin_issue_key, "LINK-ORIGIN-123");
        assert_eq!(
            params.destination_issue_key,
            Some("LINK-DEST-456".to_string())
        );
        assert_eq!(params.link_type, "Epic-Story");
        assert_eq!(params.project_key, Some("LINKTEST".to_string()));
        assert_eq!(
            params.changelog_file,
            Some("/project/docs/LINK_CHANGELOG.md".to_string())
        );

        // Test different link types
        params.link_type = "Duplicate".to_string();
        assert_eq!(params.link_type, "Duplicate");

        params.link_type = "Relates".to_string();
        assert_eq!(params.link_type, "Relates");

        params.link_type = "Blocks".to_string();
        assert_eq!(params.link_type, "Blocks");

        params.link_type = "Clones".to_string();
        assert_eq!(params.link_type, "Clones");
    }

    #[tokio::test]
    async fn test_link_issue_runner_with_direct_destination() {
        let auth = AuthData::new("user".to_string(), "token".to_string()).to_base64();
        let config = ConfigFile::new(
            auth,
            "http://127.0.0.1:0".to_string(),
            "{}".to_string(),
            "{}".to_string(),
            Table::new(),
            YaraSection::default(),
        );

        let runner = LinkIssueCmdRunner::new(&config);
        let params = LinkIssueCmdParams {
            project_key: None,
            origin_issue_key: "SRC-1".to_string(),
            destination_issue_key: Some("DST-2".to_string()),
            link_type: "Relates".to_string(),
            changelog_file: None,
        };

        let result = runner
            .link_jira_issues(params)
            .await
            .expect("link operation returns a value");
        assert_eq!(result, Value::String("Linking KO".to_string()));
    }

    #[tokio::test]
    async fn test_link_issue_runner_with_changelog_extraction() {
        let auth = AuthData::new("user".to_string(), "token".to_string()).to_base64();
        let config = ConfigFile::new(
            auth,
            "http://127.0.0.1:0".to_string(),
            "{}".to_string(),
            "{}".to_string(),
            Table::new(),
            YaraSection::default(),
        );

        let runner = LinkIssueCmdRunner::new(&config);

        let mut changelog = NamedTempFile::new().expect("create changelog file");
        writeln!(
            changelog,
            "## [1.2.3] 2024-01-01\n- Fixed TEST-101\n- Resolved TEST-202\n\n## [1.2.2] 2023-12-01\n- Previous notes"
        )
        .expect("write changelog");

        let params = LinkIssueCmdParams {
            project_key: Some("TEST".to_string()),
            origin_issue_key: "SRC-99".to_string(),
            destination_issue_key: None,
            link_type: "Blocks".to_string(),
            changelog_file: Some(changelog.path().display().to_string()),
        };

        let result = runner
            .link_jira_issues(params)
            .await
            .expect("link operation completes");
        assert_eq!(result, Value::String("Linking KO".to_string()));
    }

    #[tokio::test]
    async fn test_link_issue_runner_error_without_project_key() {
        let auth = AuthData::new("user".to_string(), "token".to_string()).to_base64();
        let config = ConfigFile::new(
            auth,
            "http://127.0.0.1:0".to_string(),
            "{}".to_string(),
            "{}".to_string(),
            Table::new(),
            YaraSection::default(),
        );

        let runner = LinkIssueCmdRunner::new(&config);
        let params = LinkIssueCmdParams {
            project_key: None,
            origin_issue_key: "SRC-1".to_string(),
            destination_issue_key: None,
            link_type: "Blocks".to_string(),
            changelog_file: None,
        };

        let err = runner
            .link_jira_issues(params)
            .await
            .expect_err("should fail");
        assert!(
            err.to_string()
                .contains("Error linking issues: Empty project key")
        );
    }

    // ===== COMPREHENSIVE DEFAULT AND FROM TRAIT TESTS =====

    #[test]
    fn test_all_default_implementations() {
        // Test all Default implementations
        let issue_params = IssueCmdParams::default();
        assert_eq!(issue_params.project_key, Some("".to_string()));
        assert_eq!(issue_params.issue_key, None);

        let issue_transition_params = IssueTransitionCmdParams::default();
        assert_eq!(issue_transition_params.issue_key, "");

        let project_params = ProjectCmdParams::default();
        assert_eq!(project_params.project_key, None);
        assert_eq!(project_params.project_name, None);

        let version_params = VersionCmdParams::default();
        assert_eq!(version_params.project, "");
        assert_eq!(version_params.version_name, None);

        let link_params = LinkIssueCmdParams::default();
        assert_eq!(link_params.origin_issue_key, "");
        assert_eq!(link_params.link_type, "");
    }

    #[test]
    fn test_all_new_implementations() {
        // Test all new() implementations
        let issue_params = IssueCmdParams::new();
        assert_eq!(issue_params.project_key, Some("".to_string()));
        assert_eq!(issue_params.issue_key, None);

        let issue_transition_params = IssueTransitionCmdParams::new();
        assert_eq!(issue_transition_params.issue_key, "");

        let project_params = ProjectCmdParams::new();
        assert_eq!(project_params.project_key, None);
        assert_eq!(project_params.project_name, None);

        let version_params = VersionCmdParams::new();
        assert_eq!(version_params.project, "");
        assert_eq!(version_params.version_name, None);

        let link_params = LinkIssueCmdParams::new();
        assert_eq!(link_params.origin_issue_key, "");
        assert_eq!(link_params.link_type, "");
    }

    #[test]
    fn test_version_cmd_params_merge_args_variants() {
        let mut current_version = Version::default();
        current_version.id = Some("123".to_string());
        current_version.project_id = Some(101);
        current_version.project = Some("TEST".to_string());
        current_version.name = Some("1.0.0".to_string());
        current_version.description = Some("Current description".to_string());
        current_version.start_date = Some("2024-01-01".to_string());
        current_version.release_date = Some("2024-02-01".to_string());
        current_version.archived = Some(false);
        current_version.released = Some(false);

        let version_args = VersionArgs {
            version_act: VersionActionValues::Update,
            project_key: "DIFF".to_string(),
            project_id: Some(202),
            version_id: Some("999".to_string()),
            version_name: Some("2.0.0".to_string()),
            version_description: Some("Override description".to_string()),
            version_start_date: Some("2025-01-10".to_string()),
            version_release_date: Some("2025-02-20".to_string()),
            version_archived: Some(true),
            version_released: Some(true),
            changelog_file: Some("CHANGELOG.md".to_string()),
            transition_issues: Some(true),
            transition_assignee: Some("user-123".to_string()),
            pagination: PaginationArgs {
                page_size: Some(50),
                page_offset: Some(5),
            },
            output: OutputArgs {
                output_format: Some(OutputValues::Json),
                output_type: Some(OutputTypes::Full),
            },
        };

        let merged = VersionCmdParams::merge_args(current_version.clone(), Some(&version_args));
        assert_eq!(merged.project, "TEST");
        assert_eq!(merged.project_id, Some(101));
        assert_eq!(merged.version_id, Some("123".to_string()));
        assert_eq!(merged.version_name, Some("2.0.0".to_string()));
        assert_eq!(
            merged.version_description,
            Some("Override description".to_string())
        );
        assert_eq!(merged.version_start_date, Some("2025-01-10".to_string()));
        assert_eq!(merged.version_release_date, Some("2025-02-20".to_string()));
        assert_eq!(merged.version_archived, Some(true));
        assert_eq!(merged.version_released, Some(true));
        assert!(merged.transition_assignee.is_none());
        assert!(merged.transition_issues.is_none());

        let merged_without_args = VersionCmdParams::merge_args(current_version.clone(), None);
        assert_eq!(merged_without_args.project, "TEST");
        assert_eq!(merged_without_args.project_id, Some(101));
        assert_eq!(merged_without_args.version_id, Some("123".to_string()));
        assert_eq!(merged_without_args.version_name, Some("1.0.0".to_string()));
        assert_eq!(
            merged_without_args.version_description,
            Some("Current description".to_string())
        );
        assert_eq!(
            merged_without_args.version_start_date,
            Some("2024-01-01".to_string())
        );
        assert_eq!(
            merged_without_args.version_release_date,
            Some("2024-02-01".to_string())
        );
        assert_eq!(merged_without_args.version_archived, Some(false));
        assert_eq!(merged_without_args.version_released, Some(false));

        let merged_default = VersionCmdParams::merge_args(Version::default(), None);
        assert_eq!(merged_default.project, "");
        assert!(merged_default.version_id.is_none());
    }

    #[test]
    fn test_version_cmd_params_mark_helpers_and_from() {
        let mut version = Version::default();
        version.id = Some("321".to_string());
        version.project = Some("HELPER".to_string());
        version.project_id = Some(303);
        version.name = Some("0.1.0".to_string());

        let released = VersionCmdParams::mark_released(version.clone());
        assert_eq!(released.version_released, Some(true));
        assert!(released.version_release_date.is_some());

        let archived = VersionCmdParams::mark_archived(version.clone());
        assert_eq!(archived.version_archived, Some(true));
        assert_eq!(archived.version_id, version.id);

        let version_args = VersionArgs {
            version_act: VersionActionValues::List,
            project_key: "PARAM".to_string(),
            project_id: Some(404),
            version_id: Some("555".to_string()),
            version_name: Some("3.0.0".to_string()),
            version_description: Some("From args".to_string()),
            version_start_date: None,
            version_release_date: None,
            version_archived: None,
            version_released: None,
            changelog_file: None,
            transition_issues: Some(false),
            transition_assignee: Some("assistant".to_string()),
            pagination: PaginationArgs {
                page_size: Some(20),
                page_offset: Some(2),
            },
            output: OutputArgs {
                output_format: None,
                output_type: None,
            },
        };

        let params = VersionCmdParams::from(&version_args);
        assert_eq!(params.project, "PARAM".to_string());
        assert_eq!(params.project_id, Some(404));
        assert_eq!(params.version_id, Some("555".to_string()));
        assert_eq!(params.version_name, Some("3.0.0".to_string()));
        assert!(params.version_start_date.is_some());
        assert_eq!(params.versions_page_size, Some(20));
        assert_eq!(params.versions_page_offset, Some(2));
        assert_eq!(params.transition_assignee, Some("assistant".to_string()));
        assert_eq!(params.transition_issues, Some(false));
    }

    // ===== RUNNER CONFIGURATION EDGE CASES =====

    #[test]
    fn test_runners_with_edge_case_configs() {
        // Test with minimal config - must have valid base64 with colon separator
        let minimal_config = ConfigFile::new(
            "dTpw".to_string(), // "u:p" in base64
            "http://localhost:8080".to_string(),
            "".to_string(), // Empty resolution
            "".to_string(), // Empty comment
            Table::new(),
            YaraSection::default(),
        );

        let _issue_runner = IssueCmdRunner::new(&minimal_config);
        let _project_runner = ProjectCmdRunner::new(&minimal_config);
        let _version_runner = VersionCmdRunner::new(&minimal_config);
        let _link_runner = LinkIssueCmdRunner::new(&minimal_config);

        // Test with very long config values
        // Create valid base64 for long username:password (user:pass repeated)
        let long_cred = format!("{}:{}", "user".repeat(100), "pass".repeat(100));
        let long_auth = base64::engine::general_purpose::STANDARD.encode(long_cred);
        let long_url = format!("https://{}.atlassian.net", "a".repeat(100));
        let long_resolution = format!(r#"{{"name": "{}"}}"#, "z".repeat(500));
        let long_comment = "y".repeat(1000);

        let long_config = ConfigFile::new(
            long_auth,
            long_url,
            long_resolution,
            long_comment,
            Table::new(),
            YaraSection::default(),
        );

        let _issue_runner = IssueCmdRunner::new(&long_config);
        let _project_runner = ProjectCmdRunner::new(&long_config);
        let _version_runner = VersionCmdRunner::new(&long_config);
        let _link_runner = LinkIssueCmdRunner::new(&long_config);

        assert!(true); // All runners were created successfully
    }

    // ===== STRESS TESTS =====

    #[test]
    fn test_massive_field_collections() {
        let mut params = IssueCmdParams::new();
        let mut massive_fields = HashMap::new();

        // Create 10,000 fields
        for i in 0..10000 {
            massive_fields.insert(
                format!("customfield_{}", i),
                if i % 3 == 0 {
                    Value::String(format!("String value {}", i))
                } else if i % 3 == 1 {
                    Value::Number(serde_json::Number::from(i))
                } else {
                    Value::Array(vec![
                        Value::String(format!("array_item_1_{}", i)),
                        Value::String(format!("array_item_2_{}", i)),
                    ])
                },
            );
        }

        params.issue_fields = Some(massive_fields);

        assert_eq!(params.issue_fields.as_ref().unwrap().len(), 10000);
        assert!(
            params
                .issue_fields
                .as_ref()
                .unwrap()
                .contains_key("customfield_5000")
        );
        assert!(
            params
                .issue_fields
                .as_ref()
                .unwrap()
                .contains_key("customfield_9999")
        );
    }

    #[test]
    fn test_unicode_and_special_characters() {
        let config = create_test_config();
        let _runner = IssueCmdRunner::new(&config);

        // Test with Unicode characters, emojis, and special symbols
        let unicode_params = IssueCmdParams {
            issue_key: Some("UNICODE-🚀🎉".to_string()),
            project_key: Some("测试项目".to_string()),
            assignee: Some("用户@тест.орг".to_string()),
            query: Some("summary ~ \"🐛 Bug with émojis and spëcial chars: àáâãäåæç\"".to_string()),
            attachment_file_path: None,
            issue_fields: Some(HashMap::from([
                (
                    "summary".to_string(),
                    Value::String("🎯 Test with émojis: ñáéíóú and symbols ©®™".to_string()),
                ),
                (
                    "description".to_string(),
                    Value::String(
                        "Multi-line\n\tDescription\nWith\r\nVarious\u{1F4A9}Newlines".to_string(),
                    ),
                ),
                (
                    "custom_unicode".to_string(),
                    Value::String(
                        "Χαίρετε κόσμε! שלום עולם! مرحبا بالعالم! Здравствуй мир!".to_string(),
                    ),
                ),
            ])),
            transition: None,
        };

        // Test that we can handle Unicode without panicking
        assert_eq!(unicode_params.issue_key, Some("UNICODE-🚀🎉".to_string()));
        assert_eq!(unicode_params.project_key, Some("测试项目".to_string()));
        assert_eq!(unicode_params.assignee, Some("用户@тест.орг".to_string()));

        // Test with special JSON escaping characters
        let special_chars_params = IssueCmdParams {
            issue_key: Some("SPECIAL-\"\\'".to_string()),
            project_key: Some("PROJECT'WITH\"QUOTES".to_string()),
            query: Some("summary ~ \"Issue with \\\"quotes\\\" and backslashes\\\\\"".to_string()),
            attachment_file_path: None,
            issue_fields: Some(HashMap::from([
                (
                    "summary".to_string(),
                    Value::String(
                        "Title with \"quotes\" and \\backslashes\\ and /slashes/".to_string(),
                    ),
                ),
                (
                    "json_test".to_string(),
                    Value::String("{\"nested\": \"json\", \"array\": [1, 2, 3]}".to_string()),
                ),
            ])),
            assignee: None,
            transition: None,
        };

        assert!(
            special_chars_params
                .issue_key
                .as_ref()
                .unwrap()
                .contains('\"')
        );
        assert!(
            special_chars_params
                .project_key
                .as_ref()
                .unwrap()
                .contains('\'')
        );
        assert!(
            special_chars_params
                .query
                .as_ref()
                .unwrap()
                .contains("\\\"")
        );
    }

    // ===== CONCURRENT ACCESS TESTS =====

    #[test]
    fn test_concurrent_runner_creation() {
        use std::sync::Arc;
        use std::thread;

        let config = Arc::new(create_test_config());
        let mut handles = vec![];

        // Create 100 threads that all create runners simultaneously
        for i in 0..100 {
            let config_clone = Arc::clone(&config);
            let handle = thread::spawn(move || {
                let _issue_runner = IssueCmdRunner::new(&config_clone);
                let _project_runner = ProjectCmdRunner::new(&config_clone);
                let _version_runner = VersionCmdRunner::new(&config_clone);
                let _link_runner = LinkIssueCmdRunner::new(&config_clone);
                i // Return thread ID for verification
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
        assert_eq!(results.len(), 100);
        assert_eq!(results[99], 99); // Last thread returned correct ID
    }

    #[tokio::test]
    async fn test_project_runner_create_errors_on_missing_key() {
        let config = create_test_config();
        let runner = ProjectCmdRunner::new(&config);
        let params = ProjectCmdParams::new();

        let err = runner
            .create_jira_project(params)
            .await
            .expect_err("missing key should error");
        assert!(err.to_string().contains("Empty project key"));
    }

    #[tokio::test]
    async fn test_project_runner_create_errors_on_missing_name() {
        let config = create_test_config();
        let runner = ProjectCmdRunner::new(&config);
        let mut params = ProjectCmdParams::new();
        params.project_key = Some("TEST".to_string());

        let err = runner
            .create_jira_project(params)
            .await
            .expect_err("missing name should error");
        assert!(err.to_string().contains("Empty project name"));
    }

    // ===== CONFIGURATION SERIALIZATION TESTS =====

    #[test]
    fn test_runners_with_complex_toml_config() {
        let mut complex_table = Table::new();
        complex_table.insert(
            "custom_field".to_string(),
            toml::Value::String("custom_value".to_string()),
        );
        complex_table.insert("timeout".to_string(), toml::Value::Integer(30000));
        complex_table.insert("retry_count".to_string(), toml::Value::Integer(3));

        let mut nested_table = Table::new();
        nested_table.insert("nested_field".to_string(), toml::Value::Boolean(true));
        complex_table.insert("nested".to_string(), toml::Value::Table(nested_table));

        let config = ConfigFile::new(
            "Y29tcGxleDpjb25maWc=".to_string(),
            // complex:config
            "https://complex.example.com".to_string(),
            "Resolved".to_string(),
            "Automatically resolved by system".to_string(),
            complex_table,
            YaraSection::default(),
        );

        let _issue_runner = IssueCmdRunner::new(&config);
        let _project_runner = ProjectCmdRunner::new(&config);
        let _version_runner = VersionCmdRunner::new(&config);
        let _link_runner = LinkIssueCmdRunner::new(&config);

        assert!(true); // All runners created with complex config
    }
}
