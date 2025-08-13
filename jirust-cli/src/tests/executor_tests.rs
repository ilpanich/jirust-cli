#[cfg(test)]
mod tests {
    use crate::args::commands::ConfigActionValues;
    use crate::config::config_file::ConfigFile;
    use crate::executors::config_executor::ConfigExecutor;
    use crate::utils::PrintableData;
    use toml::Table;

    fn create_test_config() -> ConfigFile {
        ConfigFile::new(
            "dGVzdF91c2VyOnRlc3RfYXBpX2tleQ==".to_string(),
            "https://test.atlassian.net".to_string(),
            "Done".to_string(),
            "Task completed".to_string(),
            Table::new(),
        )
    }

    #[test]
    fn test_config_executor_creation() {
        let _executor = ConfigExecutor::new(
            "/tmp/test_config.toml".to_string(),
            ConfigActionValues::Show,
        );

        // We can't directly access private fields, but we can test that creation doesn't panic
        // and that we can use the executor
        assert!(true); // This test mainly ensures the constructor works
    }

    #[tokio::test]
    async fn test_config_executor_show_action() {
        let executor = ConfigExecutor::new(
            "/tmp/test_config.toml".to_string(),
            ConfigActionValues::Show,
        );

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

    #[test]
    fn test_config_executor_with_different_actions() {
        // Test different ConfigActionValues can be used to create executors
        let _auth_executor =
            ConfigExecutor::new("/tmp/config.toml".to_string(), ConfigActionValues::Auth);

        let _jira_executor =
            ConfigExecutor::new("/tmp/config.toml".to_string(), ConfigActionValues::Jira);

        let _setup_executor =
            ConfigExecutor::new("/tmp/config.toml".to_string(), ConfigActionValues::Setup);

        let _show_executor =
            ConfigExecutor::new("/tmp/config.toml".to_string(), ConfigActionValues::Show);

        // All constructors should work without panicking
        assert!(true);
    }

    #[tokio::test]
    async fn test_config_executor_show_always_succeeds() {
        // Test that Show action always succeeds regardless of path
        // NOTE: We use Show action here because Auth, Jira, and Setup actions
        // call stdin().read_line() which would cause tests to hang waiting for input
        let executor = ConfigExecutor::new(
            "/any/path/config.toml".to_string(),
            ConfigActionValues::Show,
        );

        let config = create_test_config();
        let result = executor.exec_config_command(config).await;

        // Show action should always succeed since it doesn't write files or read stdin
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

    #[test]
    fn test_config_action_values_enum() {
        // Test that all ConfigActionValues variants exist and can be used
        let auth_action = ConfigActionValues::Auth;
        let jira_action = ConfigActionValues::Jira;
        let setup_action = ConfigActionValues::Setup;
        let show_action = ConfigActionValues::Show;

        // Test equality
        assert_eq!(auth_action, ConfigActionValues::Auth);
        assert_eq!(jira_action, ConfigActionValues::Jira);
        assert_eq!(setup_action, ConfigActionValues::Setup);
        assert_eq!(show_action, ConfigActionValues::Show);

        // Test inequality
        assert_ne!(auth_action, jira_action);
        assert_ne!(setup_action, show_action);
    }

    #[test]
    fn test_config_file_with_executor() {
        let mut config = ConfigFile::default();
        config.set_auth_key("test_auth".to_string());
        config.set_jira_url("https://test.atlassian.net".to_string());

        // Test that config can be cloned for use with executor
        let config_clone = config.clone();
        assert_eq!(config.get_auth_key(), config_clone.get_auth_key());
        assert_eq!(config.get_jira_url(), config_clone.get_jira_url());
    }

    #[test]
    fn test_multiple_executor_instances() {
        // Test creating multiple executors with different configurations
        let executors = vec![
            ConfigExecutor::new("/path1/config.toml".to_string(), ConfigActionValues::Auth),
            ConfigExecutor::new("/path2/config.toml".to_string(), ConfigActionValues::Jira),
            ConfigExecutor::new("/path3/config.toml".to_string(), ConfigActionValues::Setup),
            ConfigExecutor::new("/path4/config.toml".to_string(), ConfigActionValues::Show),
        ];

        assert_eq!(executors.len(), 4);
    }
}
