#[cfg(test)]
mod tests {
    use crate::config::config_file::{AuthData, ConfigFile};
    use crate::utils::PrintableData;
    use jira_v3_openapi::models::{CreatedIssue, IssueBean, IssueTransition};
    use serde_json::{Value, json};
    use std::collections::HashMap;
    use tempfile::tempdir;

    #[test]
    fn test_printable_data_generic_serialization() {
        let data = vec![
            json!({"id": "1", "name": "test", "active": true}),
            json!({"id": "2", "name": "another", "active": false}),
            json!({"complex": {"nested": {"array": [1, 2, 3]}}}),
        ];

        let printable = PrintableData::Generic { data: data.clone() };

        // Test serialization
        let serialized = serde_json::to_string(&printable).expect("Should serialize");
        assert!(serialized.contains("test"));
        assert!(serialized.contains("another"));
        assert!(serialized.contains("complex"));

        // Test that serialization contains expected data (deserialization not available for PrintableData)
        assert!(
            serialized.len() > 50,
            "Serialized data should be substantial"
        );
    }

    #[test]
    fn test_printable_data_issue_created_serialization() {
        let issues = vec![
            CreatedIssue {
                id: Some("10001".to_string()),
                key: Some("TEST-1".to_string()),
                param_self: Some("https://test.atlassian.net/rest/api/3/issue/10001".to_string()),
                transition: None,
                watchers: None,
            },
            CreatedIssue {
                id: Some("10002".to_string()),
                key: Some("TEST-2".to_string()),
                param_self: Some("https://test.atlassian.net/rest/api/3/issue/10002".to_string()),
                transition: None,
                watchers: None,
            },
        ];

        let printable = PrintableData::IssueCreated {
            issues: issues.clone(),
        };

        // Test serialization
        let serialized = serde_json::to_string(&printable).expect("Should serialize");
        assert!(serialized.contains("TEST-1"));
        assert!(serialized.contains("TEST-2"));
        assert!(serialized.contains("10001"));

        // Verify serialized content contains expected data
        assert!(
            serialized.len() > 100,
            "Serialized data should be substantial"
        );
    }

    #[test]
    fn test_printable_data_issue_transitions_serialization() {
        let transitions = vec![
            IssueTransition {
                id: Some("1".to_string()),
                name: Some("To Do".to_string()),
                to: None,
                has_screen: Some(false),
                is_global: Some(true),
                is_initial: Some(true),
                is_available: Some(true),
                is_conditional: Some(false),
                looped: Some(false),
                fields: None,
                expand: None,
            },
            IssueTransition {
                id: Some("2".to_string()),
                name: Some("In Progress".to_string()),
                to: None,
                has_screen: Some(true),
                is_global: Some(false),
                is_initial: Some(false),
                is_available: Some(true),
                is_conditional: Some(true),
                looped: Some(false),
                fields: None,
                expand: None,
            },
        ];

        let printable = PrintableData::IssueTransitions {
            transitions: transitions.clone(),
        };

        // Test serialization
        let serialized = serde_json::to_string(&printable).expect("Should serialize");
        assert!(serialized.contains("To Do"));
        assert!(serialized.contains("In Progress"));

        // Verify serialized content contains expected data
        assert!(
            serialized.len() > 100,
            "Serialized data should be substantial"
        );
    }

    #[test]
    fn test_printable_data_transitioned_issue_serialization() {
        let issues = vec![
            (
                "TEST-1".to_string(),
                "In Progress".to_string(),
                "user@example.com".to_string(),
                "1.0.0".to_string(),
            ),
            (
                "TEST-2".to_string(),
                "Done".to_string(),
                "admin@example.com".to_string(),
                "2.0.0".to_string(),
            ),
        ];

        let printable = PrintableData::TransitionedIssue {
            issues: issues.clone(),
        };

        // Test serialization
        let serialized = serde_json::to_string(&printable).expect("Should serialize");
        assert!(serialized.contains("TEST-1"));
        assert!(serialized.contains("TEST-2"));
        assert!(serialized.contains("user@example.com"));

        // Verify serialized content contains expected data
        assert!(
            serialized.len() > 50,
            "Serialized data should be substantial"
        );
    }

    #[test]
    fn test_config_file_toml_serialization() {
        let mut config = ConfigFile::new(
            "dGVzdF91c2VyOnRlc3RfYXBpX2tleQ==".to_string(),
            "https://test.atlassian.net".to_string(),
            "Done".to_string(),
            "Task completed".to_string(),
            toml::Table::new(),
        );

        // Add transitions using the proper method
        config.add_transition_name("start".to_string(), "In Progress".to_string());
        config.add_transition_name("finish".to_string(), "Done".to_string());

        let temp_dir = tempdir().expect("Failed to create temp dir");
        let config_path = temp_dir.path().join("test_config.toml");

        // Test writing to file (TOML serialization)
        let write_result = config.write_to_file(config_path.to_str().unwrap());
        assert!(write_result.is_ok(), "Should write config to file");

        // Test reading from file (TOML deserialization)
        let read_result = ConfigFile::read_from_file(config_path.to_str().unwrap());
        assert!(read_result.is_ok(), "Should read config from file");

        let read_config = read_result.unwrap();
        assert_eq!(read_config.get_auth_key(), config.get_auth_key());
        assert_eq!(read_config.get_jira_url(), config.get_jira_url());
        assert_eq!(
            read_config.get_standard_resolution(),
            config.get_standard_resolution()
        );
        assert_eq!(
            read_config.get_transition_name("start"),
            Some(vec!["In Progress".to_string()])
        );
        assert_eq!(
            read_config.get_transition_name("finish"),
            Some(vec!["Done".to_string()])
        );
    }

    #[test]
    fn test_config_file_round_trip_serialization() {
        // Test multiple round trips to ensure data integrity
        let original_config = ConfigFile::new(
            "dGVzdF91c2VyOnRlc3RfYXBpX2tleQ==".to_string(),
            "https://roundtrip.atlassian.net".to_string(),
            "Resolved".to_string(),
            "Issue resolved".to_string(),
            toml::Table::new(),
        );

        let temp_dir = tempdir().expect("Failed to create temp dir");

        for i in 0..5 {
            let config_path = temp_dir.path().join(format!("roundtrip_{}.toml", i));

            // Write config
            let write_result = original_config.write_to_file(config_path.to_str().unwrap());
            assert!(
                write_result.is_ok(),
                "Round trip {} write should succeed",
                i
            );

            // Read config back
            let read_result = ConfigFile::read_from_file(config_path.to_str().unwrap());
            assert!(read_result.is_ok(), "Round trip {} read should succeed", i);

            let read_config = read_result.unwrap();

            // Verify data integrity
            assert_eq!(read_config.get_auth_key(), original_config.get_auth_key());
            assert_eq!(read_config.get_jira_url(), original_config.get_jira_url());
            assert_eq!(
                read_config.get_standard_resolution(),
                original_config.get_standard_resolution()
            );
            assert_eq!(
                read_config.get_standard_resolution_comment(),
                original_config.get_standard_resolution_comment()
            );
        }
    }

    #[test]
    fn test_auth_data_base64_serialization() {
        let auth_data = AuthData::new("test_user".to_string(), "test_api_key".to_string());

        // Test base64 encoding
        let base64_encoded = auth_data.to_base64();
        assert!(!base64_encoded.is_empty(), "Base64 should not be empty");

        // Test base64 decoding
        let (decoded_username, decoded_api_key) = AuthData::from_base64(&base64_encoded);
        assert_eq!(decoded_username, "test_user");
        assert_eq!(decoded_api_key, "test_api_key");

        // Test multiple round trips
        for _ in 0..10 {
            let encoded = auth_data.to_base64();
            let (redecoded_username, redecoded_api_key) = AuthData::from_base64(&encoded);
            assert_eq!(redecoded_username, "test_user");
            assert_eq!(redecoded_api_key, "test_api_key");
        }
    }

    #[test]
    fn test_complex_printable_data_serialization() {
        // Create complex IssueBean with many fields
        let mut fields = HashMap::new();
        fields.insert("summary".to_string(), json!("Complex test issue"));
        fields.insert(
            "description".to_string(),
            json!("Very detailed description"),
        );
        fields.insert(
            "priority".to_string(),
            json!({"name": "High", "id": "2", "iconUrl": "icon.png"}),
        );
        fields.insert(
            "status".to_string(),
            json!({"name": "To Do", "category": {"key": "new"}}),
        );
        fields.insert(
            "assignee".to_string(),
            json!({"displayName": "Test User", "emailAddress": "test@example.com"}),
        );
        fields.insert(
            "components".to_string(),
            json!([{"name": "Frontend"}, {"name": "Backend"}]),
        );
        fields.insert("labels".to_string(), json!(["urgent", "customer", "bug"]));
        fields.insert(
            "customfield_10001".to_string(),
            json!({"value": "Custom Value"}),
        );

        let complex_issue = IssueBean {
            expand: Some("renderedFields,names,schema,operations,editmeta,changelog".to_string()),
            id: Some("10001".to_string()),
            param_self: Some("https://test.atlassian.net/rest/api/3/issue/10001".to_string()),
            key: Some("COMPLEX-1".to_string()),
            rendered_fields: None,
            properties: None,
            names: None,
            schema: None,
            transitions: Some(vec![
                IssueTransition {
                    id: Some("11".to_string()),
                    name: Some("In Progress".to_string()),
                    to: None,
                    has_screen: Some(false),
                    is_global: Some(false),
                    is_initial: Some(false),
                    is_available: Some(true),
                    is_conditional: Some(false),
                    looped: Some(false),
                    fields: None,
                    expand: None,
                },
                IssueTransition {
                    id: Some("21".to_string()),
                    name: Some("Done".to_string()),
                    to: None,
                    has_screen: Some(true),
                    is_global: Some(false),
                    is_initial: Some(false),
                    is_available: Some(true),
                    is_conditional: Some(true),
                    looped: Some(false),
                    fields: None,
                    expand: None,
                },
            ]),
            operations: None,
            editmeta: None,
            changelog: None,
            versioned_representations: None,
            fields_to_include: None,
            fields: Some(fields),
        };

        let printable = PrintableData::IssueData {
            issues: vec![complex_issue],
        };

        // Test serialization
        let serialized = serde_json::to_string(&printable).expect("Should serialize complex data");
        assert!(serialized.contains("COMPLEX-1"));
        assert!(serialized.contains("Complex test issue"));
        assert!(serialized.contains("Frontend"));
        assert!(serialized.contains("urgent"));

        // Verify complex serialization worked
        assert!(
            serialized.len() > 500,
            "Complex serialized data should be substantial"
        );
    }

    #[test]
    fn test_serialization_with_unicode_and_special_chars() {
        let unicode_data = vec![
            json!({"name": "测试项目", "description": "这是一个测试项目"}),
            json!({"emoji": "🚀🎉⭐", "special": "!@#$%^&*()_+-=[]{}|;':\"./<>?"}),
            json!({"mixed": "English and 中文 and русский"}),
        ];

        let printable = PrintableData::Generic { data: unicode_data };

        // Test serialization with unicode
        let serialized = serde_json::to_string(&printable).expect("Should serialize unicode data");
        assert!(serialized.contains("测试项目"));
        assert!(serialized.contains("🚀"));
        assert!(serialized.contains("русский"));

        // Verify unicode serialization worked
        assert!(
            serialized.len() > 100,
            "Unicode serialized data should be substantial"
        );
    }

    #[test]
    fn test_serialization_edge_cases() {
        // Test with null values
        let null_data = vec![
            json!(null),
            json!({"field": null}),
            json!({"array": [null, "value", null]}),
        ];

        let printable = PrintableData::Generic { data: null_data };

        let serialized = serde_json::to_string(&printable).expect("Should serialize null values");
        // Verify null values can be serialized
        assert!(serialized.contains("null"), "Should contain null values");

        // Test with empty structures
        let empty_data = vec![
            json!({}),
            json!([]),
            json!({"empty_object": {}, "empty_array": []}),
        ];

        let printable = PrintableData::Generic { data: empty_data };

        let serialized =
            serde_json::to_string(&printable).expect("Should serialize empty structures");
        // Verify empty structures can be serialized
        assert!(serialized.contains("{}"), "Should contain empty object");
        assert!(serialized.contains("[]"), "Should contain empty array");
    }

    #[test]
    fn test_large_data_serialization() {
        // Test serialization performance with large data sets
        let large_data: Vec<Value> = (0..1000).map(|i| {
            json!({
                "id": i,
                "name": format!("Item {}", i),
                "description": format!("Description for item {} with some additional text to make it longer", i),
                "metadata": {
                    "created": format!("2024-01-{:02}", (i % 28) + 1),
                    "tags": [format!("tag{}", i), format!("category{}", i % 10)],
                    "properties": {
                        "active": i % 2 == 0,
                        "priority": i % 5,
                        "score": i as f64 / 10.0
                    }
                }
            })
        }).collect();

        let printable = PrintableData::Generic { data: large_data };

        // Test serialization
        let serialized = serde_json::to_string(&printable).expect("Should serialize large data");
        assert!(
            serialized.len() > 10000,
            "Serialized data should be substantial"
        );

        // Verify large data serialization performance
        assert!(
            serialized.len() > 50000,
            "Large serialized data should be very substantial"
        );
        assert!(serialized.contains("Item 0"), "Should contain first item");
        assert!(serialized.contains("Item 999"), "Should contain last item");
    }

    #[test]
    fn test_config_serialization_with_complex_transitions() {
        let mut config = ConfigFile::new(
            "dGVzdF91c2VyOnRlc3RfYXBpX2tleQ==".to_string(),
            "https://complex-transitions.atlassian.net".to_string(),
            "Resolved".to_string(),
            "Complex resolution comment with unicode 测试 and emoji 🎉".to_string(),
            toml::Table::new(),
        );

        // Add various types of transition names using the proper method
        config.add_transition_name("simple".to_string(), "Simple Transition".to_string());
        config.add_transition_name(
            "with-spaces".to_string(),
            "Transition With Spaces".to_string(),
        );
        config.add_transition_name(
            "with_underscores".to_string(),
            "Transition_With_Underscores".to_string(),
        );
        config.add_transition_name("with.dots".to_string(), "Transition.With.Dots".to_string());
        config.add_transition_name(
            "unicode测试".to_string(),
            "Unicode Transition 测试".to_string(),
        );
        config.add_transition_name("emoji🚀".to_string(), "Emoji Transition 🚀".to_string());
        config.add_transition_name("special!@#".to_string(), "Special Chars !@#".to_string());

        let temp_dir = tempdir().expect("Failed to create temp dir");
        let config_path = temp_dir.path().join("complex_transitions.toml");

        // Test serialization
        let write_result = config.write_to_file(config_path.to_str().unwrap());
        assert!(write_result.is_ok(), "Should write complex config");

        // Test deserialization
        let read_result = ConfigFile::read_from_file(config_path.to_str().unwrap());
        assert!(read_result.is_ok(), "Should read complex config");

        let read_config = read_result.unwrap();

        // Verify all complex transitions are preserved
        assert_eq!(
            read_config.get_transition_name("simple"),
            Some(vec!["Simple Transition".to_string()])
        );
        assert_eq!(
            read_config.get_transition_name("with-spaces"),
            Some(vec!["Transition With Spaces".to_string()])
        );
        assert_eq!(
            read_config.get_transition_name("unicode测试"),
            Some(vec!["Unicode Transition 测试".to_string()])
        );
        assert_eq!(
            read_config.get_transition_name("emoji🚀"),
            Some(vec!["Emoji Transition 🚀".to_string()])
        );
        assert_eq!(
            read_config.get_transition_name("special!@#"),
            Some(vec!["Special Chars !@#".to_string()])
        );

        // Verify other fields
        assert_eq!(
            read_config.get_jira_url(),
            "https://complex-transitions.atlassian.net"
        );
        assert!(
            read_config
                .get_standard_resolution_comment()
                .contains("测试")
        );
        assert!(read_config.get_standard_resolution_comment().contains("🎉"));
    }
}
