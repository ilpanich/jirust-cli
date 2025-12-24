#[cfg(test)]
mod tests {
    use crate::config::config_file::{AuthData, ConfigFile, YaraSection};
    use toml::{Table, Value};

    #[test]
    fn test_auth_data_creation() {
        let auth_data = AuthData::new("test_user".to_string(), "test_api_key".to_string());
        let base64_token = auth_data.to_base64();

        // Verify that the token was created
        assert!(!base64_token.is_empty());

        // Verify that we can decode the token back
        let (username, api_key) = AuthData::from_base64(&base64_token);
        assert_eq!(username, "test_user");
        assert_eq!(api_key, "test_api_key");
    }

    #[test]
    fn test_auth_data_setters() {
        let mut auth_data = AuthData::new("original_user".to_string(), "original_key".to_string());

        auth_data.set_username("new_user".to_string());
        auth_data.set_api_key("new_key".to_string());

        let base64_token = auth_data.to_base64();
        let (username, api_key) = AuthData::from_base64(&base64_token);
        assert_eq!(username, "new_user");
        assert_eq!(api_key, "new_key");
    }

    #[test]
    fn test_auth_data_base64_conversion() {
        let auth_data = AuthData::new("test_user".to_string(), "test_api_key".to_string());
        let base64_str = auth_data.to_base64();

        // Base64 of "test_user:test_api_key" should be a specific value
        assert!(!base64_str.is_empty());

        let (username, api_key) = AuthData::from_base64(&base64_str);
        assert_eq!(username, "test_user");
        assert_eq!(api_key, "test_api_key");
    }

    #[test]
    fn test_config_file_creation() {
        let mut transitions = Table::new();
        transitions.insert("done".to_string(), Value::String("Done".to_string()));
        transitions.insert(
            "progress".to_string(),
            Value::String("In Progress".to_string()),
        );

        let config = ConfigFile::new(
            "dGVzdF91c2VyOnRlc3RfYXBpX2tleQ==".to_string(),
            "https://test.atlassian.net".to_string(),
            "Done".to_string(),
            "Task completed".to_string(),
            transitions,
            YaraSection::default(),
        );

        assert_eq!(config.get_auth_key(), "dGVzdF91c2VyOnRlc3RfYXBpX2tleQ==");
        assert_eq!(config.get_jira_url(), "https://test.atlassian.net");
        assert_eq!(config.get_standard_resolution(), "Done");
        assert_eq!(config.get_standard_resolution_comment(), "Task completed");
    }

    #[test]
    fn test_config_file_default() {
        let config = ConfigFile::default();

        assert_eq!(config.get_auth_key(), "");
        assert_eq!(config.get_jira_url(), "");
        assert_eq!(config.get_standard_resolution(), "");
        assert_eq!(config.get_standard_resolution_comment(), "");
    }

    #[test]
    fn test_config_file_setters() {
        let mut config = ConfigFile::default();

        config.set_auth_key("new_auth_key".to_string());
        config.set_jira_url("https://new.atlassian.net".to_string());
        config.set_standard_resolution("Fixed".to_string());
        config.set_standard_resolution_comment("Issue resolved".to_string());

        assert_eq!(config.get_auth_key(), "new_auth_key");
        assert_eq!(config.get_jira_url(), "https://new.atlassian.net");
        assert_eq!(config.get_standard_resolution(), "Fixed");
        assert_eq!(config.get_standard_resolution_comment(), "Issue resolved");
    }

    #[test]
    fn test_transition_names_management() {
        let mut config = ConfigFile::default();

        config.add_transition_name("done".to_string(), "Done".to_string());
        config.add_transition_name("cancelled".to_string(), "Cancelled".to_string());

        let done_transitions = config.get_transition_name("done");
        let cancelled_transitions = config.get_transition_name("cancelled");

        assert!(done_transitions.is_some());
        assert!(cancelled_transitions.is_some());

        if let Some(transitions) = done_transitions {
            assert!(transitions.contains(&"Done".to_string()));
        }

        if let Some(transitions) = cancelled_transitions {
            assert!(transitions.contains(&"Cancelled".to_string()));
        }
    }

    #[test]
    fn test_transition_name_by_key() {
        let mut config = ConfigFile::default();

        config.add_transition_name("done".to_string(), "Done".to_string());
        config.add_transition_name("progress".to_string(), "In Progress".to_string());

        let done_transitions = config.get_transition_name("done");
        let progress_transitions = config.get_transition_name("progress");
        let nonexistent_transitions = config.get_transition_name("nonexistent");

        assert!(done_transitions.is_some());
        assert!(progress_transitions.is_some());
        // get_transition_name always returns Some, even for nonexistent keys
        assert!(nonexistent_transitions.is_some());
        if let Some(transitions) = nonexistent_transitions {
            assert!(transitions.is_empty());
        }
    }

    #[test]
    fn test_config_file_validation() {
        let mut config = ConfigFile::default();

        // Empty config should be invalid
        assert!(config.get_auth_key().is_empty());
        assert!(config.get_jira_url().is_empty());

        // Set valid values
        config.set_auth_key("valid_auth_key".to_string());
        config.set_jira_url("https://valid.atlassian.net".to_string());

        assert!(!config.get_auth_key().is_empty());
        assert!(!config.get_jira_url().is_empty());
    }

    #[test]
    fn test_config_serialization() {
        let mut transitions = Table::new();
        transitions.insert("done".to_string(), Value::String("Done".to_string()));

        let config = ConfigFile::new(
            "dGVzdF91c2VyOnRlc3RfYXBpX2tleQ==".to_string(),
            "https://test.atlassian.net".to_string(),
            "Done".to_string(),
            "Task completed".to_string(),
            transitions,
            YaraSection::default(),
        );

        // Test serialization
        let serialized = serde_json::to_string(&config).expect("Failed to serialize config");
        assert!(serialized.contains("dGVzdF91c2VyOnRlc3RfYXBpX2tleQ=="));
        assert!(serialized.contains("https://test.atlassian.net"));

        // Test deserialization
        let deserialized: ConfigFile =
            serde_json::from_str(&serialized).expect("Failed to deserialize config");
        assert_eq!(deserialized.get_auth_key(), config.get_auth_key());
        assert_eq!(deserialized.get_jira_url(), config.get_jira_url());
    }

    #[test]
    fn test_auth_section_clone() {
        let mut config = ConfigFile::default();
        config.set_auth_key("test_token".to_string());

        let cloned = config.clone();
        assert_eq!(config.get_auth_key(), cloned.get_auth_key());
    }

    #[test]
    fn test_jira_section_clone() {
        let mut config = ConfigFile::default();
        config.set_jira_url("https://test.atlassian.net".to_string());
        config.set_standard_resolution("Done".to_string());
        config.set_standard_resolution_comment("Task completed".to_string());

        let cloned = config.clone();
        assert_eq!(config.get_jira_url(), cloned.get_jira_url());
        assert_eq!(
            config.get_standard_resolution(),
            cloned.get_standard_resolution()
        );
        assert_eq!(
            config.get_standard_resolution_comment(),
            cloned.get_standard_resolution_comment()
        );
    }

    #[test]
    fn test_config_file_debug_display() {
        let config = ConfigFile::default();
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("ConfigFile"));
    }
}
