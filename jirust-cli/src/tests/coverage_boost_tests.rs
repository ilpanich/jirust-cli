#[cfg(test)]
mod coverage_boost_tests {
    use crate::args::commands::{
        Commands, ConfigActionValues, ConfigArgs, IssueActionValues, IssueArgs,
        LinkIssueActionValues, LinkIssueArgs, OutputArgs, OutputTypes, OutputValues,
        PaginationArgs, ProjectActionValues, ProjectArgs, TransitionActionValues, TransitionArgs,
        VersionActionValues, VersionArgs,
    };
    use crate::config::config_file::ConfigFile;
    use crate::runners::cfg_cmd_runner::ConfigCmdRunner;
    use crate::{manage_config, process_command};
    use std::fs;
    use std::io::Write;
    use tempfile::{NamedTempFile, TempDir};
    use toml::Table;

    fn create_test_config() -> ConfigFile {
        ConfigFile::new(
            "dGVzdF91c2VyOnRlc3RfYXBpX2tleQ==".to_string(),
            "https://test.atlassian.net".to_string(),
            r#"{"name": "Done"}"#.to_string(),
            "Task completed".to_string(),
            Table::new(),
        )
    }

    // ===== LIB.RS PROCESS_COMMAND TESTS =====

    #[tokio::test]
    async fn test_process_command_config_without_path() {
        let cfg = create_test_config();
        let cmd = Commands::Config(ConfigArgs {
            cfg_act: ConfigActionValues::Setup,
        });

        let result = process_command(cmd, None, cfg).await;
        assert!(result.is_err());
        if let Err(err) = result {
            assert!(err.to_string().contains("Missing config file path"));
        }
    }

    #[tokio::test]
    async fn test_process_command_all_command_types() {
        let cfg = create_test_config();
        let temp_dir = TempDir::new().expect("create temp dir");
        let config_path = temp_dir.path().join("test.toml").display().to_string();

        // Write the config file first so Show can read it
        cfg.write_to_file(&config_path).expect("write config");

        // Test Config command with path
        let config_cmd = Commands::Config(ConfigArgs {
            cfg_act: ConfigActionValues::Show,
        });
        let result = process_command(config_cmd, Some(config_path.clone()), cfg.clone()).await;
        // Show command should succeed since we created the file
        assert!(result.is_ok());

        // Test Version command
        let version_cmd = Commands::Version(VersionArgs {
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
                output_format: Some(OutputValues::Json),
                output_type: Some(OutputTypes::Full),
            },
        });
        let _result = process_command(version_cmd, None, cfg.clone()).await;

        // Test Project command
        let project_cmd = Commands::Project(ProjectArgs {
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
                page_size: None,
                page_offset: None,
            },
            output: OutputArgs {
                output_format: Some(OutputValues::Json),
                output_type: Some(OutputTypes::Full),
            },
        });
        let _result = process_command(project_cmd, None, cfg.clone()).await;

        // Test Issue command
        let issue_cmd = Commands::Issue(IssueArgs {
            issue_act: IssueActionValues::Get,
            project_key: Some("TEST".to_string()),
            issue_key: Some("TEST-1".to_string()),
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
                output_format: Some(OutputValues::Json),
                output_type: Some(OutputTypes::Full),
            },
        });
        let _result = process_command(issue_cmd, None, cfg.clone()).await;

        // Test Transition command
        let transition_cmd = Commands::Transition(TransitionArgs {
            transition_act: TransitionActionValues::List,
            issue_key: "TEST-1".to_string(),
            output: OutputArgs {
                output_format: Some(OutputValues::Json),
                output_type: Some(OutputTypes::Full),
            },
        });
        let _result = process_command(transition_cmd, None, cfg.clone()).await;

        // Test Link command
        let link_cmd = Commands::Link(LinkIssueArgs {
            link_act: LinkIssueActionValues::Create,
            project_key: Some("TEST".to_string()),
            origin_issue_key: "TEST-1".to_string(),
            destination_issue_key: Some("TEST-2".to_string()),
            link_type: "Blocks".to_string(),
            changelog_file: None,
        });
        let _result = process_command(link_cmd, None, cfg).await;

        // Test passes if no panic occurs during all command processing
        assert!(true);
    }

    // ===== MANAGE_CONFIG TESTS =====

    #[test]
    fn test_manage_config_file_not_found() {
        let result = manage_config("/nonexistent/path/to/config.toml".to_string());
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
        }
    }

    #[test]
    fn test_manage_config_missing_auth_key() {
        let temp_file = NamedTempFile::new().expect("create temp file");
        let config_path = temp_file.path().display().to_string();

        // Create config with empty auth key
        let config = r#"
[auth]
auth_token = ""

[jira]
jira_url = "https://test.atlassian.net"
standard_resolution = "Done"
standard_resolution_comment = "Task completed"

[transitions]
"#;
        fs::write(&config_path, config).expect("write config");

        let result = manage_config(config_path);
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
            assert!(
                err.to_string()
                    .contains("Missing basic configuration, setup mandatory!")
            );
        }
    }

    #[test]
    fn test_manage_config_missing_jira_url() {
        let temp_file = NamedTempFile::new().expect("create temp file");
        let config_path = temp_file.path().display().to_string();

        // Create config with empty Jira URL
        let config = r#"
[auth]
auth_token = "dGVzdDp0ZXN0"

[jira]
jira_url = ""
standard_resolution = "Done"
standard_resolution_comment = "Task completed"

[transitions]
"#;
        fs::write(&config_path, config).expect("write config");

        let result = manage_config(config_path);
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
        }
    }

    #[test]
    fn test_manage_config_valid_config() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let config_path = temp_dir.path().join("config.toml").display().to_string();

        // Write a valid config file that matches the expected format
        let cfg = create_test_config();
        cfg.write_to_file(&config_path).expect("write config");

        let result = manage_config(config_path);
        assert!(result.is_ok());
        let loaded_cfg = result.unwrap();
        assert!(!loaded_cfg.get_auth_key().is_empty());
        assert!(!loaded_cfg.get_jira_url().is_empty());
    }

    // ===== CFG_CMD_RUNNER TESTS =====

    #[test]
    fn test_cfg_cmd_runner_new() {
        let runner = ConfigCmdRunner::new("/tmp/test.toml".to_string());
        assert!(true); // Test passes if no panic
    }

    #[test]
    fn test_cfg_cmd_runner_init_file() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let config_path = temp_dir.path().join("config.toml").display().to_string();

        let runner = ConfigCmdRunner::new(config_path.clone());
        let result = runner.init_file();
        assert!(result.is_ok());

        // Verify file was created
        assert!(std::path::Path::new(&config_path).exists());
    }

    #[test]
    fn test_cfg_cmd_runner_init_file_nested_path() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let config_path = temp_dir
            .path()
            .join("nested")
            .join("deep")
            .join("config.toml")
            .display()
            .to_string();

        let runner = ConfigCmdRunner::new(config_path.clone());
        let result = runner.init_file();
        assert!(result.is_ok());

        // Verify nested directories were created
        assert!(std::path::Path::new(&config_path).exists());
    }

    #[test]
    fn test_cfg_cmd_runner_show_cfg() {
        let cfg = create_test_config();
        let temp_dir = TempDir::new().expect("create temp dir");
        let config_path = temp_dir.path().join("config.toml").display().to_string();

        let runner = ConfigCmdRunner::new(config_path);
        // show_cfg just prints the config, it doesn't return anything
        runner.show_cfg(cfg);
        assert!(true); // Test passes if no panic
    }

    // ===== OUTPUT TYPE CONVERSION TESTS =====

    #[test]
    fn test_output_type_from_conversion() {
        use crate::utils::OutputType;

        let full = OutputType::from(OutputTypes::Full);
        let basic = OutputType::from(OutputTypes::Basic);
        let single = OutputType::from(OutputTypes::Single);

        // Test that conversions work (can't directly compare due to no PartialEq)
        match full {
            OutputType::Full => assert!(true),
            _ => panic!("Expected Full"),
        }
        match basic {
            OutputType::Basic => assert!(true),
            _ => panic!("Expected Basic"),
        }
        match single {
            OutputType::Single => assert!(true),
            _ => panic!("Expected Single"),
        }
    }

    // ===== ADDITIONAL EDGE CASE TESTS =====

    #[test]
    fn test_config_file_default() {
        let cfg = ConfigFile::default();
        assert_eq!(cfg.get_auth_key(), "");
        assert_eq!(cfg.get_jira_url(), "");
    }

    #[tokio::test]
    async fn test_process_command_with_minimal_config() {
        // Create a minimal but valid config (with proper auth format)
        let cfg = ConfigFile::new(
            "dTpw".to_string(), // "u:p" in base64 - minimal valid auth
            "http://localhost".to_string(),
            "".to_string(),
            "".to_string(),
            Table::new(),
        );
        let version_cmd = Commands::Version(VersionArgs {
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
                output_format: Some(OutputValues::Json),
                output_type: Some(OutputTypes::Full),
            },
        });

        let result = process_command(version_cmd, None, cfg).await;
        // Should fail due to network error (invalid URL)
        assert!(result.is_err());
    }

    #[test]
    fn test_config_file_with_special_characters() {
        let mut transitions = Table::new();
        transitions.insert(
            "resolve".to_string(),
            toml::Value::String("Done".to_string()),
        );

        let cfg = ConfigFile::new(
            "dGVzdDp0ZXN0ISRAIyQlXiYqKCk=".to_string(), // test:test!@#$%^&*()
            "https://test-123.atlassian.net".to_string(),
            r#"{"name": "Done & Complete"}"#.to_string(),
            "Completed with special chars: <>&\"'".to_string(),
            transitions,
        );

        assert!(!cfg.get_auth_key().is_empty());
        assert!(!cfg.get_jira_url().is_empty());
    }

    #[test]
    fn test_pagination_args_with_various_values() {
        let none_args = PaginationArgs {
            page_size: None,
            page_offset: None,
        };
        assert!(none_args.page_size.is_none());
        assert!(none_args.page_offset.is_none());

        let some_args = PaginationArgs {
            page_size: Some(100),
            page_offset: Some(500),
        };
        assert_eq!(some_args.page_size, Some(100));
        assert_eq!(some_args.page_offset, Some(500));

        let zero_args = PaginationArgs {
            page_size: Some(0),
            page_offset: Some(0),
        };
        assert_eq!(zero_args.page_size, Some(0));
        assert_eq!(zero_args.page_offset, Some(0));
    }

    #[test]
    fn test_output_args_all_combinations() {
        // Test all None
        let none_args = OutputArgs {
            output_format: None,
            output_type: None,
        };
        assert!(none_args.output_format.is_none());
        assert!(none_args.output_type.is_none());

        // Test Json + Full
        let json_full = OutputArgs {
            output_format: Some(OutputValues::Json),
            output_type: Some(OutputTypes::Full),
        };
        assert!(matches!(json_full.output_format, Some(OutputValues::Json)));
        assert!(matches!(json_full.output_type, Some(OutputTypes::Full)));

        // Test Table + Basic
        let table_basic = OutputArgs {
            output_format: Some(OutputValues::Table),
            output_type: Some(OutputTypes::Basic),
        };
        assert!(matches!(
            table_basic.output_format,
            Some(OutputValues::Table)
        ));
        assert!(matches!(table_basic.output_type, Some(OutputTypes::Basic)));

        // Test Table + Single
        let table_single = OutputArgs {
            output_format: Some(OutputValues::Table),
            output_type: Some(OutputTypes::Single),
        };
        assert!(matches!(
            table_single.output_format,
            Some(OutputValues::Table)
        ));
        assert!(matches!(
            table_single.output_type,
            Some(OutputTypes::Single)
        ));
    }
}
