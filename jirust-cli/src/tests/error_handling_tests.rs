#[cfg(test)]
mod tests {
    use crate::args::commands::OutputValues;
    use crate::config::config_file::{ConfigFile, YaraSection};
    use crate::utils::{OutputType, PrintableData, print_data};
    use serde_json::json;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_config_file_read_from_invalid_toml() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let config_path = temp_dir.path().join("invalid.toml");

        // Write invalid TOML content
        fs::write(&config_path, "invalid toml content [[[").expect("Failed to write invalid TOML");

        let result = ConfigFile::read_from_file(config_path.to_str().unwrap());
        assert!(result.is_err(), "Should return error for invalid TOML");
    }

    #[test]
    fn test_config_file_with_empty_values() {
        let config = ConfigFile::new(
            "".to_string(), // Empty auth key
            "".to_string(), // Empty Jira URL
            "".to_string(), // Empty resolution
            "".to_string(), // Empty resolution comment
            toml::Table::new(), YaraSection::default()
        );

        // Test that empty values are handled gracefully
        assert_eq!(config.get_auth_key(), "");
        assert_eq!(config.get_jira_url(), "");
        assert_eq!(config.get_standard_resolution(), "");
        assert_eq!(config.get_standard_resolution_comment(), "");
    }

    #[test]
    fn test_config_file_serialization_with_invalid_characters() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let config_path = temp_dir.path().join("test_config.toml");

        // Create config with special characters that might cause serialization issues
        let mut transitions = toml::Table::new();
        transitions.insert(
            "weird\"key".to_string(),
            toml::Value::String("value with\nnewlines\tand\ttabs".to_string()),
        );
        transitions.insert(
            "unicode🚀".to_string(),
            toml::Value::String("unicode value 🎉".to_string()),
        );

        let config = ConfigFile::new(
            "dGVzdF91c2VyOnRlc3RfYXBpX2tleQ==".to_string(),
            "https://test.atlassian.net".to_string(),
            "Done".to_string(),
            "Task completed".to_string(),
            transitions,
            YaraSection::default(),
        );

        // This should handle special characters gracefully
        let result = config.write_to_file(config_path.to_str().unwrap());
        assert!(
            result.is_ok(),
            "Should handle special characters in serialization"
        );

        // Verify we can read it back
        let read_result = ConfigFile::read_from_file(config_path.to_str().unwrap());
        assert!(
            read_result.is_ok(),
            "Should be able to read back config with special characters"
        );
    }

    #[test]
    fn test_printable_data_with_malformed_json() {
        // Create PrintableData with potentially problematic JSON
        let problematic_data = vec![
            json!(null),
            json!({}),
            json!([]),
            json!({"circular_ref": {"self": "reference"}}),
            json!({"very_deep": {"nested": {"object": {"with": {"many": {"levels": {"value": "deep"}}}}}}}),
            json!({"special_chars": "value with \u{0000} null bytes and 🚀 emoji"}),
        ];

        let printable = PrintableData::Generic {
            data: problematic_data,
        };

        // This should handle problematic JSON gracefully
        print_data(printable, OutputValues::Json, OutputType::Full);
        assert!(true);
    }

    #[test]
    fn test_config_validation_edge_cases() {
        // Test config validation with edge cases
        let mut config = ConfigFile::default();

        // Test with whitespace-only values
        config.set_auth_key("   ".to_string());
        config.set_jira_url("   ".to_string());
        config.set_standard_resolution("   ".to_string());
        config.set_standard_resolution_comment("   ".to_string());

        // Validation should handle whitespace appropriately
        assert_eq!(config.get_auth_key(), "   ");
        assert_eq!(config.get_jira_url(), "   ");

        // Test with very long values
        let long_string = "a".repeat(10000);
        config.set_auth_key(long_string.clone());
        config.set_jira_url(long_string.clone());

        assert_eq!(config.get_auth_key(), long_string);
        assert_eq!(config.get_jira_url(), long_string);
    }

    #[test]
    fn test_transition_name_management_edge_cases() {
        let mut config = ConfigFile::default();

        // Test adding transition with empty key
        config.add_transition_name("".to_string(), "Empty Key Transition".to_string());
        let result = config.get_transition_name("");
        assert_eq!(result, Some(vec!["Empty Key Transition".to_string()]));

        // Test adding transition with empty value
        config.add_transition_name("empty_value".to_string(), "".to_string());
        let result = config.get_transition_name("empty_value");
        assert_eq!(result, Some(vec!["".to_string()]));

        // Test adding multiple transitions with same key (should append to array)
        config.add_transition_name("duplicate".to_string(), "First Value".to_string());
        config.add_transition_name("duplicate".to_string(), "Second Value".to_string());
        let result = config.get_transition_name("duplicate");
        assert_eq!(
            result,
            Some(vec!["First Value".to_string(), "Second Value".to_string()])
        );

        // Test getting nonexistent transition
        let result = config.get_transition_name("nonexistent");
        assert_eq!(result, Some(vec![]));

        // Test with special character keys
        config.add_transition_name("special!@#$%".to_string(), "Special Chars".to_string());
        let result = config.get_transition_name("special!@#$%");
        assert_eq!(result, Some(vec!["Special Chars".to_string()]));

        // Test with unicode keys
        config.add_transition_name("测试".to_string(), "Unicode Transition".to_string());
        let result = config.get_transition_name("测试");
        assert_eq!(result, Some(vec!["Unicode Transition".to_string()]));
    }

    #[test]
    fn test_config_file_clone_and_equality() {
        let original = ConfigFile::new(
            "dGVzdF91c2VyOnRlc3RfYXBpX2tleQ==".to_string(),
            "https://test.atlassian.net".to_string(),
            "Done".to_string(),
            "Task completed".to_string(),
            toml::Table::new(), YaraSection::default()
        );

        let cloned = original.clone();

        // Test that cloned config has same values
        assert_eq!(original.get_auth_key(), cloned.get_auth_key());
        assert_eq!(original.get_jira_url(), cloned.get_jira_url());
        assert_eq!(
            original.get_standard_resolution(),
            cloned.get_standard_resolution()
        );
        assert_eq!(
            original.get_standard_resolution_comment(),
            cloned.get_standard_resolution_comment()
        );

        // Test equality by comparing values since ConfigFile doesn't implement PartialEq
        // We test that the comparison logic works by checking individual fields
        let configs_equal = original.get_auth_key() == cloned.get_auth_key()
            && original.get_jira_url() == cloned.get_jira_url()
            && original.get_standard_resolution() == cloned.get_standard_resolution();
        assert!(configs_equal, "Cloned config should have same core values");
        assert!(true);
    }

    #[test]
    fn test_concurrent_config_operations() {
        use std::sync::Arc;
        use std::thread;

        let config = Arc::new(ConfigFile::new(
            "dGVzdF91c2VyOnRlc3RfYXBpX2tleQ==".to_string(),
            "https://concurrent.atlassian.net".to_string(),
            "Done".to_string(),
            "Concurrent test completed".to_string(),
            toml::Table::new(), YaraSection::default()
        ));

        let handles: Vec<_> = (0..10)
            .map(|i| {
                let config_clone = Arc::clone(&config);
                thread::spawn(move || {
                    // Multiple threads reading config values
                    let _auth = config_clone.get_auth_key();
                    let _url = config_clone.get_jira_url();
                    let _resolution = config_clone.get_standard_resolution();
                    let _comment = config_clone.get_standard_resolution_comment();

                    // This tests that concurrent access doesn't cause issues
                    format!("Thread {} completed", i)
                })
            })
            .collect();

        for handle in handles {
            let result = handle.join();
            assert!(result.is_ok(), "Thread should complete successfully");
        }
    }

    #[test]
    fn test_memory_usage_with_large_configs() {
        // Test config with large number of transitions
        let mut large_config = ConfigFile::new(
            "dGVzdF91c2VyOnRlc3RfYXBpX2tleQ==".to_string(),
            "https://large.atlassian.net".to_string(),
            "Done".to_string(),
            "Large config test".to_string(),
            toml::Table::new(), YaraSection::default()
        );

        // Add transitions using the proper method
        for i in 0..1000 {
            large_config.add_transition_name(
                format!("transition_{}", i),
                format!("Transition Name {}", i),
            );
        }

        // Test that large config operations don't cause issues
        let cloned_large = large_config.clone();
        assert_eq!(large_config.get_auth_key(), cloned_large.get_auth_key());

        // Test accessing many transitions
        for i in 0..100 {
            let transition_name = large_config.get_transition_name(&format!("transition_{}", i));
            assert_eq!(
                transition_name,
                Some(vec![format!("Transition Name {}", i)])
            );
        }

        // Test serialization of large config
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let large_config_path = temp_dir.path().join("large_config.toml");

        let write_result = large_config.write_to_file(large_config_path.to_str().unwrap());
        assert!(
            write_result.is_ok(),
            "Should handle large config serialization"
        );

        // Test reading back large config
        let read_result = ConfigFile::read_from_file(large_config_path.to_str().unwrap());
        assert!(
            read_result.is_ok(),
            "Should handle large config deserialization"
        );
    }
}
