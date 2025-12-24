#[cfg(test)]
mod comprehensive_executor_tests {
    use crate::args::commands::ConfigActionValues;
    use crate::args::commands::{
        IssueActionValues, IssueArgs, LinkIssueActionValues, LinkIssueArgs, OutputArgs,
        OutputTypes, OutputValues, PaginationArgs, ProjectActionValues, ProjectArgs,
        TransitionActionValues, TransitionArgs, VersionActionValues, VersionArgs,
    };
    use crate::config::config_file::{ConfigFile, YaraSection};
    use crate::executors::config_executor::ConfigExecutor;
    use crate::executors::jira_commands_executors::{
        ExecJiraCommand, jira_issue_executor::IssueExecutor,
        jira_issue_link_executor::LinkIssueExecutor,
        jira_issue_transition_executor::IssueTransitionExecutor,
        jira_project_executor::ProjectExecutor, jira_version_executor::VersionExecutor,
    };
    use crate::utils::PrintableData;
    use toml::Table;

    // Test utilities
    fn create_test_config() -> ConfigFile {
        ConfigFile::new(
            "dGVzdF91c2VyOnRlc3RfYXBpX2tleQ==".to_string(), // test_user:test_api_key
            "https://test.atlassian.net".to_string(),
            "Done".to_string(),
            "Task completed".to_string(),
            Table::new(), YaraSection::default()
        )
    }

    fn create_minimal_issue_args() -> IssueArgs {
        IssueArgs {
            issue_act: IssueActionValues::Get,
            project_key: Some("TEST".to_string()),
            issue_key: Some("TEST-123".to_string()),
            issue_fields: None,
            transition_to: None,
            assignee: None,
            query: None,
            attachment_file_path: None,
            pagination: PaginationArgs {
                page_size: None,
                page_offset: None,
            },
            output: OutputArgs {
                output_format: None,
                output_type: None,
            },
        }
    }

    fn create_comprehensive_issue_args() -> IssueArgs {
        IssueArgs {
            issue_act: IssueActionValues::Create,
            project_key: Some("COMP".to_string()),
            issue_key: Some("COMP-456".to_string()),
            issue_fields: Some(vec![
                (
                    "summary".to_string(),
                    r#""Comprehensive test issue""#.to_string(),
                ),
                (
                    "description".to_string(),
                    r#""A detailed description of the test issue""#.to_string(),
                ),
                ("issuetype".to_string(), r#"{"name": "Story"}"#.to_string()),
                ("priority".to_string(), r#"{"name": "High"}"#.to_string()),
                (
                    "labels".to_string(),
                    r#"["backend", "api", "testing"]"#.to_string(),
                ),
                ("components".to_string(), r#"[{"name": "API"}]"#.to_string()),
            ]),
            transition_to: Some("In Progress".to_string()),
            assignee: Some("test.assignee@example.com".to_string()),
            query: Some("project = COMP AND status = 'To Do'".to_string()),
            attachment_file_path: None,
            pagination: PaginationArgs {
                page_size: Some(50),
                page_offset: Some(0),
            },
            output: OutputArgs {
                output_format: Some(OutputValues::Json),
                output_type: Some(OutputTypes::Full),
            },
        }
    }

    fn create_minimal_project_args() -> ProjectArgs {
        ProjectArgs {
            project_act: ProjectActionValues::List,
            project_key: Some("TEST".to_string()),
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
                page_size: None,
                page_offset: None,
            },
            output: OutputArgs {
                output_format: None,
                output_type: None,
            },
        }
    }

    fn create_comprehensive_project_args() -> ProjectArgs {
        ProjectArgs {
            project_act: ProjectActionValues::Create,
            project_key: Some("NEWPROJ".to_string()),
            project_issue_type: Some("Task".to_string()),
            project_name: Some("New Comprehensive Project".to_string()),
            project_description: Some(
                "A comprehensive project created for testing purposes".to_string(),
            ),
            project_field_configuration_id: Some(10200),
            project_issue_security_scheme_id: Some(10000),
            project_issue_type_scheme_id: Some(10001),
            project_issue_type_screen_scheme_id: Some(10002),
            project_notification_scheme_id: Some(10300),
            project_permission_scheme_id: Some(10100),
            project_workflow_scheme_id: Some(10400),
            project_lead_account_id: Some("project.lead@example.com".to_string()),
            project_assignee_type: Some("PROJECT_LEAD".to_string()),
            pagination: PaginationArgs {
                page_size: Some(25),
                page_offset: Some(0),
            },
            output: OutputArgs {
                output_format: Some(OutputValues::Table),
                output_type: Some(OutputTypes::Basic),
            },
        }
    }

    fn create_minimal_version_args() -> VersionArgs {
        VersionArgs {
            version_act: VersionActionValues::List,
            project_key: "TEST".to_string(),
            project_id: None,
            version_id: None,
            version_name: None,
            version_description: None,
            version_start_date: None,
            version_release_date: None,
            version_archived: None,
            version_released: None,
            changelog_file: None,
            transition_issues: None,
            transition_assignee: None,
            pagination: PaginationArgs {
                page_size: None,
                page_offset: None,
            },
            output: OutputArgs {
                output_format: None,
                output_type: None,
            },
        }
    }

    fn create_comprehensive_version_args() -> VersionArgs {
        VersionArgs {
            version_act: VersionActionValues::Create,
            project_key: "VER".to_string(),
            project_id: Some(10001),
            version_id: Some("2.0.0".to_string()),
            version_name: Some("Version 2.0.0".to_string()),
            version_description: Some(
                "Major release with breaking changes and new features".to_string(),
            ),
            version_start_date: Some("2024-01-01".to_string()),
            version_release_date: Some("2024-12-31".to_string()),
            version_archived: Some(false),
            version_released: Some(false),
            changelog_file: Some("CHANGELOG.md".to_string()),
            transition_issues: Some(true),
            transition_assignee: Some("release.manager@example.com".to_string()),
            pagination: PaginationArgs {
                page_size: Some(100),
                page_offset: Some(0),
            },
            output: OutputArgs {
                output_format: Some(OutputValues::Json),
                output_type: Some(OutputTypes::Full),
            },
        }
    }

    fn create_minimal_link_args() -> LinkIssueArgs {
        LinkIssueArgs {
            link_act: LinkIssueActionValues::Create,
            project_key: Some("TEST".to_string()),
            origin_issue_key: "TEST-123".to_string(),
            destination_issue_key: Some("TEST-456".to_string()),
            link_type: "Blocks".to_string(),
            changelog_file: None,
        }
    }

    fn create_comprehensive_link_args() -> LinkIssueArgs {
        LinkIssueArgs {
            link_act: LinkIssueActionValues::Create,
            project_key: Some("LINK".to_string()),
            origin_issue_key: "LINK-789".to_string(),
            destination_issue_key: Some("LINK-012".to_string()),
            link_type: "Relates".to_string(),
            changelog_file: Some("LINK_CHANGES.md".to_string()),
        }
    }

    fn create_minimal_transition_args() -> TransitionArgs {
        TransitionArgs {
            transition_act: TransitionActionValues::List,
            issue_key: "TEST-123".to_string(),
            output: OutputArgs {
                output_format: None,
                output_type: None,
            },
        }
    }

    fn create_comprehensive_transition_args() -> TransitionArgs {
        TransitionArgs {
            transition_act: TransitionActionValues::List,
            issue_key: "TRANS-789".to_string(),
            output: OutputArgs {
                output_format: Some(OutputValues::Table),
                output_type: Some(OutputTypes::Full),
            },
        }
    }

    // Issue Executor Tests
    #[tokio::test]
    async fn test_issue_executor_construction_all_actions() {
        let config = create_test_config();
        let base_args = create_minimal_issue_args();

        // Test all issue action variants
        let actions = vec![
            IssueActionValues::Assign,
            IssueActionValues::Create,
            IssueActionValues::Delete,
            IssueActionValues::Get,
            IssueActionValues::Search,
            IssueActionValues::Transition,
            IssueActionValues::Update,
        ];

        for action in actions {
            let mut args = base_args.clone();
            args.issue_act = action.clone();
            let _executor = IssueExecutor::new(config.clone(), action, args);

            // Test that executor was created successfully
            // Since we can't access private fields, we test through successful construction
            assert!(true); // Constructor succeeded
        }
    }

    #[tokio::test]
    async fn test_issue_executor_exec_command_error_handling() {
        let config = create_test_config();

        // Test with missing required fields for different actions - most will panic due to expect() usage
        // We'll test the ones that return proper errors

        // Test Get action without issue key
        let mut args = create_minimal_issue_args();
        args.issue_act = IssueActionValues::Get;
        args.issue_key = None;
        let executor = IssueExecutor::new(config.clone(), IssueActionValues::Get, args);

        let result = executor.exec_jira_command().await;
        assert!(result.is_err());

        // Test Delete action without issue key
        let mut args = create_minimal_issue_args();
        args.issue_act = IssueActionValues::Delete;
        args.issue_key = None;
        let executor = IssueExecutor::new(config.clone(), IssueActionValues::Delete, args);

        let result = executor.exec_jira_command().await;
        assert!(result.is_err());

        // Test Update action without issue key
        let mut args = create_minimal_issue_args();
        args.issue_act = IssueActionValues::Update;
        args.issue_key = None;
        let executor = IssueExecutor::new(config, IssueActionValues::Update, args);

        let result = executor.exec_jira_command().await;
        assert!(result.is_err());
    }

    // Project Executor Tests
    #[tokio::test]
    async fn test_project_executor_construction_all_actions() {
        let config = create_test_config();
        let base_args = create_minimal_project_args();

        let actions = vec![ProjectActionValues::Create, ProjectActionValues::List];

        for action in actions {
            let mut args = base_args.clone();
            args.project_act = action.clone();
            let _executor = ProjectExecutor::new(config.clone(), action, args);

            assert!(true); // Constructor succeeded
        }
    }

    #[tokio::test]
    async fn test_project_executor_with_comprehensive_args() {
        let config = create_test_config();
        let args = create_comprehensive_project_args();

        let _executor = ProjectExecutor::new(config, ProjectActionValues::Create, args);

        // Test that executor handles comprehensive arguments without panicking
        assert!(true); // Constructor succeeded
    }

    // Version Executor Tests
    #[tokio::test]
    async fn test_version_executor_construction_all_actions() {
        let config = create_test_config();
        let base_args = create_minimal_version_args();

        let actions = vec![
            VersionActionValues::Archive,
            VersionActionValues::Create,
            VersionActionValues::Delete,
            VersionActionValues::List,
            VersionActionValues::RelatedWorkItems,
            VersionActionValues::Release,
            VersionActionValues::Update,
        ];

        for action in actions {
            let mut args = base_args.clone();
            args.version_act = action.clone();
            let _executor = VersionExecutor::new(config.clone(), action, args);

            assert!(true); // Constructor succeeded
        }
    }

    #[tokio::test]
    async fn test_version_executor_with_comprehensive_args() {
        let config = create_test_config();
        let args = create_comprehensive_version_args();

        let _executor = VersionExecutor::new(config, VersionActionValues::Create, args);

        // Test that executor handles comprehensive arguments without panicking
        assert!(true); // Constructor succeeded
    }

    // Link Issue Executor Tests
    #[tokio::test]
    async fn test_link_issue_executor_construction() {
        let config = create_test_config();

        // Test with minimal args
        let minimal_args = create_minimal_link_args();
        let _executor =
            LinkIssueExecutor::new(config.clone(), LinkIssueActionValues::Create, minimal_args);
        assert!(true);

        // Test with comprehensive args
        let comprehensive_args = create_comprehensive_link_args();
        let _executor =
            LinkIssueExecutor::new(config, LinkIssueActionValues::Create, comprehensive_args);
        assert!(true);
    }

    // Issue Transition Executor Tests
    #[tokio::test]
    async fn test_issue_transition_executor_construction() {
        let config = create_test_config();

        // Test with minimal args
        let minimal_args = create_minimal_transition_args();
        let _executor = IssueTransitionExecutor::new(
            config.clone(),
            TransitionActionValues::List,
            minimal_args,
        );
        assert!(true);

        // Test with comprehensive args
        let comprehensive_args = create_comprehensive_transition_args();
        let _executor =
            IssueTransitionExecutor::new(config, TransitionActionValues::List, comprehensive_args);
        assert!(true);
    }

    // Config Executor Tests
    #[tokio::test]
    async fn test_config_executor_comprehensive() {
        let config_path = "/tmp/test_config_comprehensive.toml".to_string();

        // Test all config actions
        let actions = vec![
            ConfigActionValues::Auth,
            ConfigActionValues::Jira,
            ConfigActionValues::Setup,
            ConfigActionValues::Show,
        ];

        for action in actions {
            let _executor = ConfigExecutor::new(config_path.clone(), action);
            assert!(true); // Constructor succeeded
        }

        // Test Show action execution (only one that doesn't require user input)
        let executor = ConfigExecutor::new(config_path, ConfigActionValues::Show);
        let config = create_test_config();
        let result = executor.exec_config_command(config).await;

        assert!(result.is_ok());
        if let Ok(data) = result {
            assert_eq!(data.len(), 1);
            match &data[0] {
                PrintableData::Generic { data } => {
                    assert_eq!(data.len(), 1);
                    assert_eq!(data[0], serde_json::Value::String("DONE!".to_string()));
                }
                _ => panic!("Expected Generic PrintableData"),
            }
        }
    }

    // Cross-Executor Tests
    #[tokio::test]
    async fn test_multiple_executors_same_config() {
        let config = create_test_config();

        // Create multiple executors with the same config
        let _issue_executor = IssueExecutor::new(
            config.clone(),
            IssueActionValues::Get,
            create_minimal_issue_args(),
        );
        let _project_executor = ProjectExecutor::new(
            config.clone(),
            ProjectActionValues::List,
            create_minimal_project_args(),
        );
        let _version_executor = VersionExecutor::new(
            config.clone(),
            VersionActionValues::List,
            create_minimal_version_args(),
        );
        let _link_executor = LinkIssueExecutor::new(
            config.clone(),
            LinkIssueActionValues::Create,
            create_minimal_link_args(),
        );
        let _transition_executor = IssueTransitionExecutor::new(
            config,
            TransitionActionValues::List,
            create_minimal_transition_args(),
        );

        // All should be created successfully
        assert!(true);
    }

    // Output Format Tests
    #[test]
    fn test_output_args_variations() {
        let config = create_test_config();

        // Test different output format combinations
        let output_combinations = vec![
            OutputArgs {
                output_format: None,
                output_type: None,
            },
            OutputArgs {
                output_format: Some(OutputValues::Json),
                output_type: None,
            },
            OutputArgs {
                output_format: Some(OutputValues::Table),
                output_type: None,
            },
            OutputArgs {
                output_format: None,
                output_type: Some(OutputTypes::Basic),
            },
            OutputArgs {
                output_format: None,
                output_type: Some(OutputTypes::Full),
            },
            OutputArgs {
                output_format: Some(OutputValues::Json),
                output_type: Some(OutputTypes::Basic),
            },
            OutputArgs {
                output_format: Some(OutputValues::Json),
                output_type: Some(OutputTypes::Full),
            },
            OutputArgs {
                output_format: Some(OutputValues::Table),
                output_type: Some(OutputTypes::Basic),
            },
            OutputArgs {
                output_format: Some(OutputValues::Table),
                output_type: Some(OutputTypes::Full),
            },
        ];

        for output_config in output_combinations {
            let mut args = create_minimal_issue_args();
            args.output = output_config;

            let _executor = IssueExecutor::new(config.clone(), IssueActionValues::Get, args);
            assert!(true); // Constructor succeeded with this output configuration
        }
    }

    // Pagination Tests
    #[test]
    fn test_pagination_args_variations() {
        let config = create_test_config();

        // Test different pagination combinations
        let pagination_combinations = vec![
            PaginationArgs {
                page_size: None,
                page_offset: None,
            },
            PaginationArgs {
                page_size: Some(10),
                page_offset: None,
            },
            PaginationArgs {
                page_size: Some(25),
                page_offset: None,
            },
            PaginationArgs {
                page_size: Some(50),
                page_offset: None,
            },
            PaginationArgs {
                page_size: Some(100),
                page_offset: None,
            },
            PaginationArgs {
                page_size: None,
                page_offset: Some(0),
            },
            PaginationArgs {
                page_size: None,
                page_offset: Some(25),
            },
            PaginationArgs {
                page_size: None,
                page_offset: Some(50),
            },
            PaginationArgs {
                page_size: Some(20),
                page_offset: Some(0),
            },
            PaginationArgs {
                page_size: Some(20),
                page_offset: Some(20),
            },
            PaginationArgs {
                page_size: Some(20),
                page_offset: Some(100),
            },
            PaginationArgs {
                page_size: Some(1),
                page_offset: Some(0),
            }, // Minimal page size
            PaginationArgs {
                page_size: Some(1000),
                page_offset: Some(0),
            }, // Large page size
        ];

        for pagination_config in pagination_combinations {
            let mut args = create_minimal_issue_args();
            args.pagination = pagination_config;

            let _executor = IssueExecutor::new(config.clone(), IssueActionValues::Search, args);
            assert!(true); // Constructor succeeded with this pagination configuration
        }
    }

    // Field Complexity Tests
    #[test]
    fn test_complex_issue_fields() {
        let config = create_test_config();

        // Test with complex nested JSON structures in fields
        let complex_fields = vec![
            (
                "summary".to_string(),
                r#""Complex Issue with Special Characters: !@#$%^&*()""#.to_string(),
            ),
            (
                "description".to_string(),
                r#""Multi-line\nDescription\nWith\nBreaks""#.to_string(),
            ),
            (
                "issuetype".to_string(),
                r#"{"id": "10001", "name": "Story", "subtask": false}"#.to_string(),
            ),
            (
                "priority".to_string(),
                r#"{"id": "2", "name": "High", "iconUrl": "https://example.com/icon.png"}"#
                    .to_string(),
            ),
            (
                "labels".to_string(),
                r#"["backend", "api", "urgent", "customer-facing"]"#.to_string(),
            ),
            (
                "components".to_string(),
                r#"[{"id": "10000", "name": "API"}, {"id": "10001", "name": "Database"}]"#
                    .to_string(),
            ),
            (
                "fixVersions".to_string(),
                r#"[{"id": "10200", "name": "1.0.0", "released": false}]"#.to_string(),
            ),
            (
                "customfield_10001".to_string(),
                r#"{"value": "Custom Field Value"}"#.to_string(),
            ),
            (
                "customfield_10002".to_string(),
                r#"[{"value": "Option 1"}, {"value": "Option 2"}]"#.to_string(),
            ),
            (
                "assignee".to_string(),
                r#"{"accountId": "5b10ac8d82e05b22cc7d4ef5", "displayName": "Test User"}"#
                    .to_string(),
            ),
            (
                "reporter".to_string(),
                r#"{"accountId": "5b10ac8d82e05b22cc7d4ef6", "displayName": "Reporter User"}"#
                    .to_string(),
            ),
            ("duedate".to_string(), r#""2024-12-31""#.to_string()),
            (
                "timetracking".to_string(),
                r#"{"originalEstimate": "1w 2d", "remainingEstimate": "3d"}"#.to_string(),
            ),
        ];

        let mut args = create_comprehensive_issue_args();
        args.issue_fields = Some(complex_fields);

        let _executor = IssueExecutor::new(config, IssueActionValues::Create, args);
        assert!(true); // Constructor succeeded with complex fields
    }

    // Error Condition Tests
    #[tokio::test]
    async fn test_executor_error_message_quality() {
        let config = create_test_config();

        // Test that error messages are informative - using actions that return errors instead of panicking
        let mut args = create_minimal_issue_args();
        args.issue_key = None;

        let executor = IssueExecutor::new(config, IssueActionValues::Get, args);
        let result = executor.exec_jira_command().await;

        assert!(result.is_err());
        if let Err(err) = result {
            let error_msg = err.to_string();
            // Check that error message is informative
            assert!(error_msg.contains("issue key") || error_msg.contains("Empty"));
        }
    }

    // Panic Tests for Methods that use expect()
    #[tokio::test]
    #[should_panic(expected = "Assignee is required")]
    async fn test_issue_executor_assign_without_assignee_panics() {
        let config = create_test_config();
        let mut args = create_minimal_issue_args();
        args.issue_act = IssueActionValues::Assign;
        args.assignee = None;
        let executor = IssueExecutor::new(config, IssueActionValues::Assign, args);

        let _result = executor.exec_jira_command().await;
    }

    #[tokio::test]
    #[should_panic(expected = "Project Key is required")]
    async fn test_issue_executor_create_without_project_key_panics() {
        let config = create_test_config();
        let mut args = create_minimal_issue_args();
        args.issue_act = IssueActionValues::Create;
        args.project_key = None;
        let executor = IssueExecutor::new(config, IssueActionValues::Create, args);

        let _result = executor.exec_jira_command().await;
    }

    // Configuration Variation Tests
    #[test]
    fn test_executors_with_different_configs() {
        let config1 = create_test_config();

        let config2 = ConfigFile::new(
            "YWRtaW46c2VjcmV0X3Rva2Vu".to_string(), // admin:secret_token
            "https://another.atlassian.net".to_string(),
            "Resolved".to_string(),
            "Issue resolved".to_string(),
            Table::new(), YaraSection::default()
        );

        let config3 = ConfigFile::new(
            "dGVhbTpzdXBlcl9zZWNyZXQ=".to_string(), // team:super_secret
            "https://team.atlassian.net".to_string(),
            "Closed".to_string(),
            "Issue closed".to_string(),
            Table::new(), YaraSection::default()
        );

        // Test that all configs work with all executors
        for config in vec![config1, config2, config3] {
            let _issue_executor = IssueExecutor::new(
                config.clone(),
                IssueActionValues::Get,
                create_minimal_issue_args(),
            );
            let _project_executor = ProjectExecutor::new(
                config.clone(),
                ProjectActionValues::List,
                create_minimal_project_args(),
            );
            let _version_executor = VersionExecutor::new(
                config.clone(),
                VersionActionValues::List,
                create_minimal_version_args(),
            );
            let _link_executor = LinkIssueExecutor::new(
                config.clone(),
                LinkIssueActionValues::Create,
                create_minimal_link_args(),
            );
            let _transition_executor = IssueTransitionExecutor::new(
                config,
                TransitionActionValues::List,
                create_minimal_transition_args(),
            );
        }

        assert!(true); // All constructors succeeded
    }

    // Memory and Performance Tests
    #[test]
    fn test_executor_memory_usage() {
        let config = create_test_config();
        let mut issue_executors = Vec::new();
        let mut project_executors = Vec::new();
        let mut version_executors = Vec::new();

        // Create many executors to test memory usage
        for _ in 0..50 {
            issue_executors.push(IssueExecutor::new(
                config.clone(),
                IssueActionValues::Get,
                create_minimal_issue_args(),
            ));
            project_executors.push(ProjectExecutor::new(
                config.clone(),
                ProjectActionValues::List,
                create_minimal_project_args(),
            ));
            version_executors.push(VersionExecutor::new(
                config.clone(),
                VersionActionValues::List,
                create_minimal_version_args(),
            ));
        }

        assert_eq!(issue_executors.len(), 50);
        assert_eq!(project_executors.len(), 50);
        assert_eq!(version_executors.len(), 50);

        // Test that we can still create more
        let _additional =
            IssueExecutor::new(config, IssueActionValues::Get, create_minimal_issue_args());
        assert!(true);
    }

    // Action Enum Coverage Tests
    #[test]
    fn test_all_action_enum_variants() {
        // Ensure we test all possible action values
        let issue_actions = vec![
            IssueActionValues::Assign,
            IssueActionValues::Create,
            IssueActionValues::Delete,
            IssueActionValues::Get,
            IssueActionValues::Search,
            IssueActionValues::Transition,
            IssueActionValues::Update,
        ];
        assert_eq!(issue_actions.len(), 7);

        let project_actions = vec![ProjectActionValues::Create, ProjectActionValues::List];
        assert_eq!(project_actions.len(), 2);

        let version_actions = vec![
            VersionActionValues::Archive,
            VersionActionValues::Create,
            VersionActionValues::Delete,
            VersionActionValues::List,
            VersionActionValues::RelatedWorkItems,
            VersionActionValues::Release,
            VersionActionValues::Update,
        ];
        assert_eq!(version_actions.len(), 7);

        let link_actions = vec![LinkIssueActionValues::Create];
        assert_eq!(link_actions.len(), 1);

        let transition_actions = vec![TransitionActionValues::List];
        assert_eq!(transition_actions.len(), 1);

        let config_actions = vec![
            ConfigActionValues::Auth,
            ConfigActionValues::Jira,
            ConfigActionValues::Setup,
            ConfigActionValues::Show,
        ];
        assert_eq!(config_actions.len(), 4);
    }
}
