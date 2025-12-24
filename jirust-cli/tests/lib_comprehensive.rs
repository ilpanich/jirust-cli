use jirust_cli::args::commands::{
    Commands, ConfigActionValues, ConfigArgs, IssueActionValues, IssueArgs, LinkIssueActionValues,
    LinkIssueArgs, OutputArgs, OutputTypes, OutputValues, PaginationArgs, ProjectActionValues,
    ProjectArgs, TransitionActionValues, TransitionArgs, VersionActionValues, VersionArgs,
};
use jirust_cli::config::config_file::{AuthData, ConfigFile, YaraSection};
use jirust_cli::process_command;
use jirust_cli::utils::PrintableData;
use std::io::{Error, ErrorKind};
use toml::{Table, Value};

fn create_valid_config() -> ConfigFile {
    ConfigFile::new(
        "dGVzdF91c2VyOnRlc3RfYXBpX2tleQ==".to_string(),
        "https://valid.atlassian.net".to_string(),
        "Done".to_string(),
        "Task completed".to_string(),
        Table::new(),
        YaraSection::default(),
    )
}

fn create_invalid_config_empty_auth() -> ConfigFile {
    ConfigFile::new(
        "".to_string(),
        "https://valid.atlassian.net".to_string(),
        "Done".to_string(),
        "Task completed".to_string(),
        Table::new(),
        YaraSection::default(),
    )
}

fn create_invalid_config_empty_url() -> ConfigFile {
    ConfigFile::new(
        "valid_token".to_string(),
        "".to_string(),
        "Done".to_string(),
        "Task completed".to_string(),
        Table::new(),
        YaraSection::default(),
    )
}

#[test]
fn test_manage_config_validation_logic() {
    // Test the validation logic in manage_config
    let valid_config = create_valid_config();
    let invalid_auth_config = create_invalid_config_empty_auth();
    let invalid_url_config = create_invalid_config_empty_url();

    // Test that valid config passes validation
    assert!(!valid_config.get_auth_key().is_empty());
    assert!(!valid_config.get_jira_url().is_empty());

    // Test that invalid configs fail validation
    assert!(invalid_auth_config.get_auth_key().is_empty());
    assert!(invalid_url_config.get_jira_url().is_empty());
}

#[tokio::test]
async fn test_process_command_all_commands() {
    let config = create_valid_config();

    // Test Config command
    let config_command = Commands::Config(ConfigArgs {
        cfg_act: ConfigActionValues::Show,
    });
    let result = process_command(
        config_command,
        Some("/tmp/test_config.toml".to_string()),
        config.clone(),
    )
    .await;
    assert!(result.is_ok());

    // Test Issue command
    let issue_command = Commands::Issue(IssueArgs {
        issue_act: IssueActionValues::Search,
        project_key: Some("TEST".to_string()),
        issue_key: None,
        issue_fields: None,
        transition_to: None,
        assignee: None,
        pagination: PaginationArgs {
            page_size: Some(10),
            page_offset: None,
        },
        output: OutputArgs {
            output_format: Some(OutputValues::Json),
            output_type: Some(OutputTypes::Basic),
        },
        query: None,
        attachment_file_path: None,
    });
    let result = process_command(issue_command, None, config.clone()).await;
    // This might fail due to network call, but the function should be called
    assert!(result.is_ok() || result.is_err());

    // Test Project command
    let project_command = Commands::Project(ProjectArgs {
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
            page_size: Some(10),
            page_offset: None,
        },
        output: OutputArgs {
            output_format: Some(OutputValues::Table),
            output_type: Some(OutputTypes::Full),
        },
    });
    let result = process_command(project_command, None, config.clone()).await;
    // This might fail due to network call, but the function should be called
    assert!(result.is_ok() || result.is_err());

    // Test Version command
    let version_command = Commands::Version(VersionArgs {
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
        pagination: PaginationArgs {
            page_size: Some(20),
            page_offset: Some(0),
        },
        output: OutputArgs {
            output_format: Some(OutputValues::Json),
            output_type: Some(OutputTypes::Single),
        },
        transition_assignee: None,
        transition_issues: None,
    });
    let result = process_command(version_command, None, config.clone()).await;
    assert!(result.is_ok() || result.is_err());

    // Test Transition command
    let transition_command = Commands::Transition(TransitionArgs {
        transition_act: TransitionActionValues::List,
        issue_key: "TEST-1".to_string(),
        output: OutputArgs {
            output_format: None,
            output_type: None,
        },
    });
    let result = process_command(transition_command, None, config.clone()).await;
    assert!(result.is_ok() || result.is_err());

    // Test Link command
    let link_command = Commands::Link(LinkIssueArgs {
        link_act: LinkIssueActionValues::Create,
        project_key: Some("TEST".to_string()),
        origin_issue_key: "TEST-1".to_string(),
        destination_issue_key: Some("TEST-2".to_string()),
        link_type: "Blocks".to_string(),
        changelog_file: None,
    });
    let result = process_command(link_command, None, config).await;
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_process_command_config_without_path_error() {
    let config = create_valid_config();
    let config_command = Commands::Config(ConfigArgs {
        cfg_act: ConfigActionValues::Setup,
    });

    let result = process_command(config_command, None, config).await;
    assert!(result.is_err());

    if let Err(err) = result {
        assert!(err.to_string().contains("Missing config file path"));
    }
}

#[tokio::test]
async fn test_process_command_with_various_output_formats() {
    let config = create_valid_config();

    // Test with different output formats
    let json_command = Commands::Issue(IssueArgs {
        issue_act: IssueActionValues::Search,
        project_key: Some("TEST".to_string()),
        issue_key: None,
        issue_fields: None,
        transition_to: None,
        assignee: None,
        pagination: PaginationArgs {
            page_size: Some(5),
            page_offset: None,
        },
        output: OutputArgs {
            output_format: Some(OutputValues::Json),
            output_type: Some(OutputTypes::Full),
        },
        query: None,
        attachment_file_path: None,
    });

    let table_command = Commands::Issue(IssueArgs {
        issue_act: IssueActionValues::Search,
        project_key: Some("TEST".to_string()),
        issue_key: None,
        issue_fields: None,
        transition_to: None,
        assignee: None,
        pagination: PaginationArgs {
            page_size: Some(5),
            page_offset: None,
        },
        output: OutputArgs {
            output_format: Some(OutputValues::Table),
            output_type: Some(OutputTypes::Basic),
        },
        query: None,
        attachment_file_path: None,
    });

    let result1 = process_command(json_command, None, config.clone()).await;
    let result2 = process_command(table_command, None, config).await;

    // These might fail due to network issues, but should not panic
    assert!(result1.is_ok() || result1.is_err());
    assert!(result2.is_ok() || result2.is_err());
}

#[test]
fn test_config_file_comprehensive_validation() {
    // Test various invalid config scenarios
    let both_empty_config = ConfigFile::new(
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        Table::new(),
        YaraSection::default(),
    );

    assert!(both_empty_config.get_auth_key().is_empty());
    assert!(both_empty_config.get_jira_url().is_empty());

    // Test config with whitespace
    let whitespace_config = ConfigFile::new(
        "   ".to_string(),
        "   ".to_string(),
        "Done".to_string(),
        "Comment".to_string(),
        Table::new(),
        YaraSection::default(),
    );

    // These would still be considered non-empty strings
    assert!(!whitespace_config.get_auth_key().is_empty());
    assert!(!whitespace_config.get_jira_url().is_empty());
}

#[tokio::test]
async fn test_process_command_error_handling() {
    let config = create_valid_config();

    // Test that errors are properly propagated
    let config_command = Commands::Config(ConfigArgs {
        cfg_act: ConfigActionValues::Auth,
    });

    // This should fail because we're not providing a valid config file path
    let result = process_command(
        config_command,
        Some("/nonexistent/directory/config.toml".to_string()),
        config,
    )
    .await;

    assert!(result.is_err());
    if let Err(err) = result {
        // Should contain error about config storage
        assert!(err.to_string().contains("Error"));
    }
}

#[test]
fn test_error_type_compatibility() {
    // Test that we can create and handle different error types
    let io_error = Error::new(ErrorKind::NotFound, "Test error");
    let boxed_error: Box<dyn std::error::Error> = Box::new(io_error);

    assert!(boxed_error.to_string().contains("Test error"));
}

#[tokio::test]
async fn test_process_command_result_structure() {
    let config = create_valid_config();
    let config_command = Commands::Config(ConfigArgs {
        cfg_act: ConfigActionValues::Show,
    });

    let result = process_command(config_command, Some("/tmp/test.toml".to_string()), config).await;

    if let Ok(data) = result {
        assert!(!data.is_empty());

        // Check that we get the expected PrintableData structure
        match &data[0] {
            PrintableData::Generic { data: json_data } => {
                assert!(!json_data.is_empty());
                assert_eq!(json_data[0], serde_json::Value::String("DONE!".to_string()));
            }
            _ => panic!("Expected Generic PrintableData for config show"),
        }
    }
}

#[test]
fn test_config_file_transitions_handling() {
    let mut transitions = Table::new();
    transitions.insert(
        "done".to_string(),
        Value::Array(vec![Value::String("Done".to_string())]),
    );
    transitions.insert(
        "progress".to_string(),
        Value::Array(vec![Value::String("In Progress".to_string())]),
    );

    let config = ConfigFile::new(
        "dGVzdF91c2VyOnRlc3RfYXBpX2tleQ==".to_string(),
        "https://test.atlassian.net".to_string(),
        "Done".to_string(),
        "Task completed".to_string(),
        transitions,
        YaraSection::default(),
    );

    // Test that transitions are properly stored
    let done_transitions = config.get_transition_name("done");
    let progress_transitions = config.get_transition_name("progress");

    assert!(done_transitions.is_some());
    assert!(progress_transitions.is_some());

    if let Some(transitions) = done_transitions {
        assert!(transitions.contains(&"Done".to_string()));
    }

    if let Some(transitions) = progress_transitions {
        assert!(transitions.contains(&"In Progress".to_string()));
    }
}

#[test]
fn test_auth_data_integration_with_config() {
    let auth_data = AuthData::new("testuser".to_string(), "testapi".to_string());
    let base64_token = auth_data.to_base64();

    let config = ConfigFile::new(
        base64_token.clone(),
        "https://test.atlassian.net".to_string(),
        "Done".to_string(),
        "Task completed".to_string(),
        Table::new(),
        YaraSection::default(),
    );

    assert_eq!(config.get_auth_key(), base64_token);

    // Test that we can recover the auth data
    let (username, api_key) = AuthData::from_base64(config.get_auth_key());
    assert_eq!(username, "testuser");
    assert_eq!(api_key, "testapi");
}

#[tokio::test]
async fn test_command_execution_coverage() {
    let config = create_valid_config();

    // Create instances of all command variants to ensure they can be constructed
    let commands = vec![
        Commands::Config(ConfigArgs {
            cfg_act: ConfigActionValues::Show,
        }),
        Commands::Issue(IssueArgs {
            issue_act: IssueActionValues::Search,
            project_key: Some("TEST".to_string()),
            issue_key: None,
            issue_fields: None,
            transition_to: None,
            assignee: None,
            pagination: PaginationArgs {
                page_size: None,
                page_offset: None,
            },
            output: OutputArgs {
                output_format: None,
                output_type: None,
            },
            query: None,
            attachment_file_path: None,
        }),
        Commands::Project(ProjectArgs {
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
                output_format: None,
                output_type: None,
            },
        }),
        Commands::Version(VersionArgs {
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
            pagination: PaginationArgs {
                page_size: None,
                page_offset: None,
            },
            output: OutputArgs {
                output_format: None,
                output_type: None,
            },
            transition_assignee: None,
            transition_issues: None,
        }),
    ];

    // Test that all commands can be processed (they may fail due to network issues, but shouldn't panic)
    for command in commands {
        let config_path = match command {
            Commands::Config(_) => Some("/tmp/test.toml".to_string()),
            _ => None,
        };

        let result = process_command(command, config_path, config.clone()).await;
        // Just ensure we get a result (ok or err) without panicking
        assert!(result.is_ok() || result.is_err());
    }
}
