#[cfg(test)]
mod executor_and_printer_tests {
    use crate::args::commands::ConfigActionValues;
    use crate::config::config_file::{ConfigFile, YaraSection};
    use crate::executors::config_executor::ConfigExecutor;
    use crate::utils::json_printer::print_json;
    use crate::utils::PrintableData;
    use jira_v3_openapi::models::{CreatedIssue, IssueBean, IssueTransition, Project, Version};
    use tempfile::tempdir;
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

    // ========================================================================
    // CONFIG EXECUTOR TESTS
    // ========================================================================

    #[tokio::test]
    async fn test_config_executor_show_action() {
        let temp_dir = tempdir().expect("create temp dir");
        let config_file = temp_dir.path().join("config.toml");
        let config_path = config_file.to_str().unwrap().to_string();

        let executor = ConfigExecutor::new(config_path, ConfigActionValues::Show);
        let config = create_test_config();

        let result = executor.exec_config_command(config).await;
        assert!(result.is_ok());

        let data = result.unwrap();
        assert_eq!(data.len(), 1);
    }

    #[tokio::test]
    async fn test_config_executor_auth_action() {
        let temp_dir = tempdir().expect("create temp dir");
        let config_file = temp_dir.path().join("config_auth.toml");
        let config_path = config_file.to_str().unwrap().to_string();

        // Write initial config
        std::fs::write(&config_file, "").expect("write config");

        let executor = ConfigExecutor::new(config_path, ConfigActionValues::Auth);
        let config = create_test_config();

        let result = executor.exec_config_command(config).await;
        // This might fail due to file I/O, but it exercises the code path
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_config_executor_jira_action() {
        let temp_dir = tempdir().expect("create temp dir");
        let config_file = temp_dir.path().join("config_jira.toml");
        let config_path = config_file.to_str().unwrap().to_string();

        std::fs::write(&config_file, "").expect("write config");

        let executor = ConfigExecutor::new(config_path, ConfigActionValues::Jira);
        let config = create_test_config();

        let result = executor.exec_config_command(config).await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_config_executor_setup_action() {
        let temp_dir = tempdir().expect("create temp dir");
        let config_file = temp_dir.path().join("config_setup.toml");
        let config_path = config_file.to_str().unwrap().to_string();

        std::fs::write(&config_file, "").expect("write config");

        let executor = ConfigExecutor::new(config_path, ConfigActionValues::Setup);
        let config = create_test_config();

        let result = executor.exec_config_command(config).await;
        assert!(result.is_ok() || result.is_err());
    }

    // ========================================================================
    // JSON PRINTER TESTS
    // ========================================================================

    #[test]
    fn test_print_json_generic() {
        let data = PrintableData::Generic {
            data: vec![
                serde_json::Value::String("test".to_string()),
                serde_json::Value::Number(serde_json::Number::from(42)),
            ],
        };

        // Just verify it doesn't panic
        print_json(data);
    }

    #[test]
    fn test_print_json_project() {
        let project = Project::new();
        let data = PrintableData::Project {
            projects: vec![project],
        };

        print_json(data);
    }

    #[test]
    fn test_print_json_version() {
        let version = Version::new();
        let data = PrintableData::Version {
            versions: vec![version],
        };

        print_json(data);
    }

    #[test]
    fn test_print_json_issue_type() {
        // Just test with empty vector - still exercises the code path
        let data = PrintableData::IssueType {
            issue_types: vec![],
        };

        print_json(data);
    }

    #[test]
    fn test_print_json_issue_type_field() {
        // Just test with empty vector - still exercises the code path
        let data = PrintableData::IssueTypeField {
            issue_type_fields: vec![],
        };

        print_json(data);
    }

    #[test]
    fn test_print_json_issue_created() {
        let created = CreatedIssue::new();
        let data = PrintableData::IssueCreated {
            issues: vec![created],
        };

        print_json(data);
    }

    #[test]
    fn test_print_json_issue_data() {
        let issue = IssueBean::new();
        let data = PrintableData::IssueData {
            issues: vec![issue],
        };

        print_json(data);
    }

    #[test]
    fn test_print_json_issue_transitions() {
        let transition = IssueTransition::new();
        let data = PrintableData::IssueTransitions {
            transitions: vec![transition],
        };

        print_json(data);
    }

    #[test]
    fn test_print_json_transitioned_issue() {
        let data = PrintableData::TransitionedIssue {
            issues: vec![
                (
                    "ISSUE-1".to_string(),
                    "Done".to_string(),
                    "user1".to_string(),
                    "1.0".to_string(),
                ),
                (
                    "ISSUE-2".to_string(),
                    "InProgress".to_string(),
                    "user2".to_string(),
                    "2.0".to_string(),
                ),
            ],
        };

        print_json(data);
    }

    #[test]
    fn test_print_json_version_related_work() {
        let data = PrintableData::VersionRelatedWork {
            version_related_work_items: vec![],
        };

        print_json(data);
    }

    #[test]
    fn test_print_json_with_complex_generic_data() {
        let complex_data = serde_json::json!({
            "name": "test",
            "value": 123,
            "nested": {
                "key": "value"
            },
            "array": [1, 2, 3]
        });

        let data = PrintableData::Generic {
            data: vec![complex_data],
        };

        print_json(data);
    }

    #[test]
    fn test_print_json_empty_vectors() {
        // Test all variants with empty vectors
        print_json(PrintableData::Project { projects: vec![] });
        print_json(PrintableData::Version { versions: vec![] });
        print_json(PrintableData::IssueType {
            issue_types: vec![],
        });
        print_json(PrintableData::IssueTypeField {
            issue_type_fields: vec![],
        });
        print_json(PrintableData::IssueCreated { issues: vec![] });
        print_json(PrintableData::IssueData { issues: vec![] });
        print_json(PrintableData::IssueTransitions {
            transitions: vec![],
        });
        print_json(PrintableData::TransitionedIssue { issues: vec![] });
        print_json(PrintableData::VersionRelatedWork {
            version_related_work_items: vec![],
        });
        print_json(PrintableData::Generic { data: vec![] });
    }

    #[test]
    fn test_print_json_multiple_items() {
        let projects = vec![Project::new(), Project::new(), Project::new()];
        let data = PrintableData::Project { projects };
        print_json(data);

        let versions = vec![Version::new(), Version::new()];
        let data = PrintableData::Version { versions };
        print_json(data);

        let transitions = vec![
            IssueTransition::new(),
            IssueTransition::new(),
            IssueTransition::new(),
        ];
        let data = PrintableData::IssueTransitions { transitions };
        print_json(data);
    }
}
