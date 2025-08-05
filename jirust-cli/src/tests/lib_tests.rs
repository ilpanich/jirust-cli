#[cfg(test)]
mod tests {
    use crate::process_command;
    use crate::config::config_file::ConfigFile;
    use crate::args::commands::{Commands, ConfigArgs, ConfigActionValues};
    use std::io::{Error, ErrorKind};
    use toml::Table;

    #[test]
    fn test_manage_config_with_valid_config() {
        let valid_config = ConfigFile::new(
            "valid_auth_token".to_string(),
            "https://valid.atlassian.net".to_string(),
            "Done".to_string(),
            "Task completed".to_string(),
            Table::new(),
        );
        
        // Test valid config validation logic
        assert!(!valid_config.get_auth_key().is_empty());
        assert!(!valid_config.get_jira_url().is_empty());
    }

    #[test]
    fn test_manage_config_with_empty_auth() {
        let invalid_config = ConfigFile::new(
            "".to_string(),
            "https://valid.atlassian.net".to_string(),
            "Done".to_string(),
            "Task completed".to_string(),
            Table::new(),
        );
        
        // This should be considered invalid due to empty auth key
        assert!(invalid_config.get_auth_key().is_empty());
        assert!(!invalid_config.get_jira_url().is_empty());
    }

    #[test]
    fn test_manage_config_with_empty_jira_url() {
        let invalid_config = ConfigFile::new(
            "valid_auth_token".to_string(),
            "".to_string(),
            "Done".to_string(),
            "Task completed".to_string(),
            Table::new(),
        );
        
        // This should be considered invalid due to empty Jira URL
        assert!(!invalid_config.get_auth_key().is_empty());
        assert!(invalid_config.get_jira_url().is_empty());
    }

    #[test]
    fn test_config_validation_logic() {
        let valid_config = create_valid_config();
        let invalid_config_empty_auth = create_config_with_empty_auth();
        let invalid_config_empty_url = create_config_with_empty_url();
        
        // Valid config should pass validation
        assert!(!valid_config.get_auth_key().is_empty() && !valid_config.get_jira_url().is_empty());
        
        // Invalid configs should fail validation
        assert!(invalid_config_empty_auth.get_auth_key().is_empty() || invalid_config_empty_auth.get_jira_url().is_empty());
        assert!(invalid_config_empty_url.get_auth_key().is_empty() || invalid_config_empty_url.get_jira_url().is_empty());
    }

    #[tokio::test]
    async fn test_process_command_config_with_path() {
        let config_args = ConfigArgs {
            cfg_act: ConfigActionValues::Setup,
        };
        
        let command = Commands::Config(config_args);
        let config_file_path = Some("/tmp/test_config.toml".to_string());
        let cfg_data = create_valid_config();
        
        // This should return an error because the file doesn't exist
        let result = process_command(command, config_file_path, cfg_data).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_process_command_config_without_path() {
        let config_args = ConfigArgs {
            cfg_act: ConfigActionValues::Setup,
        };
        
        let command = Commands::Config(config_args);
        let config_file_path = None;
        let cfg_data = create_valid_config();
        
        // This should return an error due to missing config file path
        let result = process_command(command, config_file_path, cfg_data).await;
        assert!(result.is_err());
        
        if let Err(err) = result {
            let error_message = err.to_string();
            assert!(error_message.contains("Missing config file path"));
        }
    }

    #[test]
    fn test_error_creation() {
        let error = Error::new(ErrorKind::NotFound, "Test error message");
        assert_eq!(error.kind(), ErrorKind::NotFound);
        assert!(error.to_string().contains("Test error message"));
    }

    #[test]
    fn test_config_file_clone() {
        let original_config = create_valid_config();
        let cloned_config = original_config.clone();
        
        assert_eq!(original_config.get_auth_key(), cloned_config.get_auth_key());
        assert_eq!(original_config.get_jira_url(), cloned_config.get_jira_url());
        assert_eq!(original_config.get_standard_resolution(), cloned_config.get_standard_resolution());
    }

    #[test]
    fn test_config_file_debug() {
        let config = create_valid_config();
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("ConfigFile"));
    }

    #[test]
    fn test_config_equality() {
        let config1 = create_valid_config();
        let mut config2 = create_valid_config();
        
        // Initially they should be equal
        assert_eq!(config1.get_auth_key(), config2.get_auth_key());
        assert_eq!(config1.get_jira_url(), config2.get_jira_url());
        
        // After modification, they should be different
        config2.set_auth_key("different_auth_key".to_string());
        assert_ne!(config1.get_auth_key(), config2.get_auth_key());
    }

    #[test]
    fn test_config_serialization_roundtrip() {
        let original_config = create_valid_config();
        
        // Serialize to JSON
        let json_string = serde_json::to_string(&original_config).expect("Failed to serialize config");
        
        // Deserialize back from JSON
        let deserialized_config: ConfigFile = serde_json::from_str(&json_string).expect("Failed to deserialize config");
        
        // Check that values are preserved
        assert_eq!(original_config.get_auth_key(), deserialized_config.get_auth_key());
        assert_eq!(original_config.get_jira_url(), deserialized_config.get_jira_url());
        assert_eq!(original_config.get_standard_resolution(), deserialized_config.get_standard_resolution());
        assert_eq!(original_config.get_standard_resolution_comment(), deserialized_config.get_standard_resolution_comment());
    }

    // Helper functions
    fn create_valid_config() -> ConfigFile {
        ConfigFile::new(
            "valid_auth_token".to_string(),
            "https://valid.atlassian.net".to_string(),
            "Done".to_string(),
            "Task completed".to_string(),
            Table::new(),
        )
    }

    fn create_config_with_empty_auth() -> ConfigFile {
        ConfigFile::new(
            "".to_string(),
            "https://valid.atlassian.net".to_string(),
            "Done".to_string(),
            "Task completed".to_string(),
            Table::new(),
        )
    }

    fn create_config_with_empty_url() -> ConfigFile {
        ConfigFile::new(
            "valid_auth_token".to_string(),
            "".to_string(),
            "Done".to_string(),
            "Task completed".to_string(),
            Table::new(),
        )
    }
}