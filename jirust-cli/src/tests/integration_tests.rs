#[cfg(test)]
mod tests {
    use crate::args::commands::{OutputArgs, OutputTypes, OutputValues, PaginationArgs};
    use crate::config::config_file::{AuthData, ConfigFile};
    use crate::utils::PrintableData;
    use jira_v3_openapi::models::{CreatedIssue, Project, Version};
    use serde_json;
    use std::env;
    use std::process::Command;
    use tempfile::tempdir;
    use toml::{Table, Value};

    fn cli_binary_path() -> std::path::PathBuf {
        let mut binary_path = env::current_exe().expect("test exe path");
        binary_path.pop();
        binary_path.pop();
        binary_path.push(if cfg!(windows) {
            "jirust-cli.exe"
        } else {
            "jirust-cli"
        });
        binary_path
    }

    #[test]
    fn test_end_to_end_config_creation() {
        // Test creating a complete configuration from scratch
        let username = "test_user".to_string();
        let api_key = "test_api_key_12345".to_string();

        let auth_data = AuthData::new(username.clone(), api_key.clone());
        let base64_token = auth_data.to_base64();

        let mut transitions = Table::new();
        transitions.insert("done".to_string(), Value::String("Done".to_string()));
        transitions.insert(
            "progress".to_string(),
            Value::String("In Progress".to_string()),
        );
        transitions.insert("todo".to_string(), Value::String("To Do".to_string()));

        let config = ConfigFile::new(
            base64_token.clone(),
            "https://mycompany.atlassian.net".to_string(),
            "Done".to_string(),
            "Issue has been resolved".to_string(),
            transitions,
        );

        // Verify all parts work together
        assert_eq!(config.get_auth_key(), base64_token);
        assert_eq!(config.get_jira_url(), "https://mycompany.atlassian.net");
        assert_eq!(config.get_standard_resolution(), "Done");
        let done_transitions = config.get_transition_name("done");
        let progress_transitions = config.get_transition_name("progress");

        assert!(done_transitions.is_some());
        assert!(progress_transitions.is_some());

        // Test that the original auth data can be recovered
        let (recovered_username, recovered_api_key) = AuthData::from_base64(config.get_auth_key());
        assert_eq!(recovered_username, username);
        assert_eq!(recovered_api_key, api_key);
    }

    #[test]
    fn test_printable_data_workflow() {
        // Test creating different types of printable data that would be used in real scenarios

        // Project data scenario
        let projects = vec![
            Project {
                id: Some("10000".to_string()),
                key: Some("PROJ1".to_string()),
                name: Some("Project One".to_string()),
                description: Some("First project".to_string()),
                ..Default::default()
            },
            Project {
                id: Some("10001".to_string()),
                key: Some("PROJ2".to_string()),
                name: Some("Project Two".to_string()),
                description: Some("Second project".to_string()),
                ..Default::default()
            },
        ];

        let project_data = PrintableData::Project { projects };

        // Serialize to JSON (as would happen in real usage)
        let json_output =
            serde_json::to_string(&project_data).expect("Failed to serialize project data");
        assert!(json_output.contains("PROJ1"));
        assert!(json_output.contains("PROJ2"));
        assert!(json_output.contains("Project One"));
        assert!(json_output.contains("Project Two"));

        // Version data scenario
        let versions = vec![
            Version {
                id: Some("100".to_string()),
                name: Some("v1.0.0".to_string()),
                description: Some("Initial release".to_string()),
                released: Some(true),
                archived: Some(false),
                ..Default::default()
            },
            Version {
                id: Some("101".to_string()),
                name: Some("v1.1.0".to_string()),
                description: Some("Feature update".to_string()),
                released: Some(false),
                archived: Some(false),
                ..Default::default()
            },
        ];

        let version_data = PrintableData::Version { versions };
        let version_json =
            serde_json::to_string(&version_data).expect("Failed to serialize version data");
        assert!(version_json.contains("v1.0.0"));
        assert!(version_json.contains("v1.1.0"));
        assert!(version_json.contains("Initial release"));
        assert!(version_json.contains("Feature update"));
    }

    #[test]
    fn test_configuration_with_various_transition_types() {
        let mut config = ConfigFile::default();

        // Set up a comprehensive set of transitions that might be used in different workflows
        // Add all transitions individually
        config.add_transition_name("backlog".to_string(), "Backlog".to_string());
        config.add_transition_name(
            "selected".to_string(),
            "Selected for Development".to_string(),
        );
        config.add_transition_name("progress".to_string(), "In Progress".to_string());
        config.add_transition_name("review".to_string(), "In Review".to_string());
        config.add_transition_name("testing".to_string(), "Testing".to_string());
        config.add_transition_name("done".to_string(), "Done".to_string());
        config.add_transition_name("cancelled".to_string(), "Cancelled".to_string());
        config.set_auth_key("dGVzdF91c2VyOnRlc3RfYXBpX2tleQ==".to_string());
        config.set_jira_url("https://test.atlassian.net".to_string());

        // Test retrieval of various transition names
        assert!(config.get_transition_name("backlog").is_some());
        assert!(config.get_transition_name("selected").is_some());
        assert!(config.get_transition_name("progress").is_some());
        assert!(config.get_transition_name("review").is_some());
        assert!(config.get_transition_name("testing").is_some());
        assert!(config.get_transition_name("done").is_some());
        assert!(config.get_transition_name("cancelled").is_some());
        // get_transition_name always returns Some, even for nonexistent keys
        let nonexistent = config.get_transition_name("nonexistent");
        assert!(nonexistent.is_some());
        if let Some(transitions) = nonexistent {
            assert!(transitions.is_empty());
        }

        // Test that all transitions are preserved
        // We can't directly get all transitions count, so let's just verify they exist
    }

    #[test]
    fn test_output_configuration_combinations() {
        // Test different combinations of output configurations
        let json_summary = OutputArgs {
            output_format: Some(OutputValues::Json),
            output_type: Some(OutputTypes::Basic),
        };

        let json_full = OutputArgs {
            output_format: Some(OutputValues::Json),
            output_type: Some(OutputTypes::Full),
        };

        let table_summary = OutputArgs {
            output_format: Some(OutputValues::Table),
            output_type: Some(OutputTypes::Basic),
        };

        let table_full = OutputArgs {
            output_format: Some(OutputValues::Table),
            output_type: Some(OutputTypes::Full),
        };

        // Verify different combinations are properly handled
        assert!(matches!(
            json_summary.output_format,
            Some(OutputValues::Json)
        ));
        assert!(matches!(json_summary.output_type, Some(OutputTypes::Basic)));

        assert!(matches!(json_full.output_format, Some(OutputValues::Json)));
        assert!(matches!(json_full.output_type, Some(OutputTypes::Full)));

        assert!(matches!(
            table_summary.output_format,
            Some(OutputValues::Table)
        ));
        assert!(matches!(
            table_summary.output_type,
            Some(OutputTypes::Basic)
        ));

        assert!(matches!(
            table_full.output_format,
            Some(OutputValues::Table)
        ));
        assert!(matches!(table_full.output_type, Some(OutputTypes::Full)));
    }

    #[test]
    fn test_pagination_scenarios() {
        // Test various pagination configurations
        let no_pagination = PaginationArgs {
            page_size: None,
            page_offset: None,
        };

        let with_page_size = PaginationArgs {
            page_size: Some(50),
            page_offset: None,
        };

        let with_offset = PaginationArgs {
            page_size: Some(20),
            page_offset: Some(100),
        };

        let large_page = PaginationArgs {
            page_size: Some(1000),
            page_offset: Some(0),
        };

        // Verify pagination configurations
        assert!(no_pagination.page_size.is_none());
        assert!(no_pagination.page_offset.is_none());

        assert_eq!(with_page_size.page_size, Some(50));
        assert!(with_page_size.page_offset.is_none());

        assert_eq!(with_offset.page_size, Some(20));
        assert_eq!(with_offset.page_offset, Some(100));

        assert_eq!(large_page.page_size, Some(1000));
        assert_eq!(large_page.page_offset, Some(0));
    }

    #[test]
    fn test_complex_printable_data_serialization() {
        // Test serialization of complex nested data structures
        let created_issues = vec![
            CreatedIssue {
                id: Some("10001".to_string()),
                key: Some("PROJ-1".to_string()),
                ..Default::default()
            },
            CreatedIssue {
                id: Some("10002".to_string()),
                key: Some("PROJ-2".to_string()),
                ..Default::default()
            },
        ];

        let issue_data = PrintableData::IssueCreated {
            issues: created_issues,
        };

        // Serialize and verify structure
        let serialized =
            serde_json::to_string(&issue_data).expect("Failed to serialize issue data");
        assert!(serialized.contains("\"issues\""));
        assert!(serialized.contains("PROJ-1"));
        assert!(serialized.contains("PROJ-2"));
        assert!(serialized.contains("10001"));
        assert!(serialized.contains("10002"));

        // Test with generic JSON data
        let generic_data = vec![
            serde_json::json!({
                "type": "project",
                "data": {
                    "id": "10000",
                    "name": "Test Project",
                    "issues": ["PROJ-1", "PROJ-2", "PROJ-3"]
                }
            }),
            serde_json::json!({
                "type": "summary",
                "data": {
                    "total_projects": 5,
                    "total_issues": 42,
                    "open_issues": 15
                }
            }),
        ];

        let generic_printable = PrintableData::Generic { data: generic_data };
        let generic_serialized =
            serde_json::to_string(&generic_printable).expect("Failed to serialize generic data");
        assert!(generic_serialized.contains("Test Project"));
        assert!(generic_serialized.contains("total_projects"));
        assert!(generic_serialized.contains("PROJ-1"));
    }

    #[test]
    fn test_config_modification_workflow() {
        // Test a realistic workflow of config modification
        let mut config = ConfigFile::default();

        // Initially config should be empty/invalid
        assert!(config.get_auth_key().is_empty());
        assert!(config.get_jira_url().is_empty());

        // Step 1: Set basic auth and URL
        config.set_auth_key("initial_auth_key".to_string());
        config.set_jira_url("https://initial.atlassian.net".to_string());

        assert_eq!(config.get_auth_key(), "initial_auth_key");
        assert_eq!(config.get_jira_url(), "https://initial.atlassian.net");

        // Step 2: Add resolution settings
        config.set_standard_resolution("Fixed".to_string());
        config.set_standard_resolution_comment("Issue has been fixed".to_string());

        assert_eq!(config.get_standard_resolution(), "Fixed");
        assert_eq!(
            config.get_standard_resolution_comment(),
            "Issue has been fixed"
        );

        // Step 3: Add transition names
        let mut transitions = Table::new();
        transitions.insert("fix".to_string(), Value::String("Fix Issue".to_string()));
        config.add_transition_name("fix".to_string(), "Fix Issue".to_string());

        assert!(config.get_transition_name("fix").is_some());

        // Step 4: Update auth key (simulating token refresh)
        config.set_auth_key("refreshed_auth_key".to_string());
        assert_eq!(config.get_auth_key(), "refreshed_auth_key");

        // Step 5: Verify all changes persist
        assert_eq!(config.get_jira_url(), "https://initial.atlassian.net");
        assert_eq!(config.get_standard_resolution(), "Fixed");
        assert!(config.get_transition_name("fix").is_some());
    }

    #[test]
    fn test_cli_config_show_command_executes_via_main() {
        let home_dir = tempdir().expect("create temp home");
        let config_dir = home_dir.path().join(".jirust-cli");
        std::fs::create_dir_all(&config_dir).expect("create config dir");
        let config_path = config_dir.join("jirust-cli.toml");

        let auth = AuthData::new("user".to_string(), "token".to_string()).to_base64();
        let config = ConfigFile::new(
            auth,
            "http://127.0.0.1:0".to_string(),
            "Done".to_string(),
            "Completed".to_string(),
            Table::new(),
        );
        config
            .write_to_file(config_path.to_str().expect("config path"))
            .expect("write config file");

        let output = Command::new(cli_binary_path())
            .arg("config")
            .arg("show")
            .env("HOME", home_dir.path())
            .output()
            .expect("run cli binary");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("DONE!"));
    }

    #[test]
    fn test_cli_config_show_without_home_env_uses_relative_path() {
        let working_dir = tempdir().expect("create temp dir");
        let relative_dir = working_dir.path().join(".jirust-cli");
        std::fs::create_dir_all(&relative_dir).expect("create relative config dir");
        let config_path = relative_dir.join("jirust-cli.toml");

        let auth = AuthData::new("user".to_string(), "token".to_string()).to_base64();
        let config = ConfigFile::new(
            auth,
            "http://127.0.0.1:0".to_string(),
            "Done".to_string(),
            "Completed".to_string(),
            Table::new(),
        );
        config
            .write_to_file(config_path.to_str().expect("config path"))
            .expect("write config file");

        let output = Command::new(cli_binary_path())
            .arg("config")
            .arg("show")
            .current_dir(working_dir.path())
            .env_remove("HOME")
            .output()
            .expect("run cli binary");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("DONE!"));
    }

    #[test]
    fn test_cli_reports_parse_error_for_unknown_flag() {
        let output = Command::new(cli_binary_path())
            .arg("--definitely-invalid-flag")
            .output()
            .expect("run cli binary");

        assert!(!output.status.success());
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.to_lowercase().contains("error"));
    }
}
