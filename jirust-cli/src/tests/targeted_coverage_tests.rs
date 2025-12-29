#[cfg(test)]
mod targeted_coverage_tests {
    use crate::config::config_file::{ConfigFile, YaraSection};
    use crate::runners::jira_cmd_runners::{
        project_cmd_runner::{ProjectCmdParams, ProjectCmdRunner},
        version_cmd_runner::{VersionCmdParams, VersionCmdRunner},
    };
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
    // AREA 1: CONFIG FILE YARA GETTER/SETTER METHODS (config_file.rs:472-530)
    // ========================================================================

    #[test]
    fn test_config_yara_rules_source_getter_setter() {
        let mut config = create_test_config();

        // Test getter default value
        let default_source = config.get_yara_rules_source();
        assert!(!default_source.is_empty());

        // Test setter
        config.set_yara_rules_source("https://example.com/custom-rules.git".to_string());
        assert_eq!(config.get_yara_rules_source(), "https://example.com/custom-rules.git");
    }

    #[test]
    fn test_config_yara_rules_directory_getter_setter() {
        let mut config = create_test_config();

        // Test getter default value
        let default_dir = config.get_yara_rules_directory();
        assert!(!default_dir.is_empty());

        // Test setter
        config.set_yara_rules_directory("custom-yara-rules".to_string());
        assert_eq!(config.get_yara_rules_directory(), "custom-yara-rules");
    }

    #[test]
    fn test_config_yara_cache_file_getter_setter() {
        let mut config = create_test_config();

        // Test getter default value
        let default_cache = config.get_yara_cache_file();
        assert!(!default_cache.is_empty());

        // Test setter
        config.set_yara_cache_file("custom_cache.bin".to_string());
        assert_eq!(config.get_yara_cache_file(), "custom_cache.bin");
    }

    #[test]
    fn test_config_yara_cache_version_file_getter_setter() {
        let mut config = create_test_config();

        // Test getter default value
        let default_version = config.get_yara_cache_version_file();
        assert!(!default_version.is_empty());

        // Test setter
        config.set_yara_cache_version_file("custom_version.txt".to_string());
        assert_eq!(config.get_yara_cache_version_file(), "custom_version.txt");
    }

    #[test]
    fn test_config_yara_all_setters_together() {
        let mut config = create_test_config();

        // Set all YARA config values
        config.set_yara_rules_source("https://example.com/rules.zip".to_string());
        config.set_yara_rules_directory("my-rules".to_string());
        config.set_yara_cache_file("my-cache.bin".to_string());
        config.set_yara_cache_version_file("my-version.txt".to_string());

        // Verify all were set correctly
        assert_eq!(config.get_yara_rules_source(), "https://example.com/rules.zip");
        assert_eq!(config.get_yara_rules_directory(), "my-rules");
        assert_eq!(config.get_yara_cache_file(), "my-cache.bin");
        assert_eq!(config.get_yara_cache_version_file(), "my-version.txt");
    }

    #[test]
    fn test_config_yara_setters_with_empty_strings() {
        let mut config = create_test_config();

        // Test setters with empty strings
        config.set_yara_rules_source("".to_string());
        config.set_yara_rules_directory("".to_string());
        config.set_yara_cache_file("".to_string());
        config.set_yara_cache_version_file("".to_string());

        // Verify they accept empty strings
        assert_eq!(config.get_yara_rules_source(), "");
        assert_eq!(config.get_yara_rules_directory(), "");
        assert_eq!(config.get_yara_cache_file(), "");
        assert_eq!(config.get_yara_cache_version_file(), "");
    }

    #[test]
    fn test_config_yara_setters_with_special_characters() {
        let mut config = create_test_config();

        // Test with special characters and paths
        config.set_yara_rules_source("https://example.com/rules-v1.2.3.git".to_string());
        config.set_yara_rules_directory("rules_2024/malware".to_string());
        config.set_yara_cache_file("cache.v2.bin".to_string());
        config.set_yara_cache_version_file("version-2024.txt".to_string());

        assert_eq!(config.get_yara_rules_source(), "https://example.com/rules-v1.2.3.git");
        assert_eq!(config.get_yara_rules_directory(), "rules_2024/malware");
        assert_eq!(config.get_yara_cache_file(), "cache.v2.bin");
        assert_eq!(config.get_yara_cache_version_file(), "version-2024.txt");
    }

    // ========================================================================
    // AREA 2: VERSION CMD RUNNER OPERATIONS
    // ========================================================================

    #[tokio::test]
    async fn test_version_cmd_runner_archive_version() {
        let config = create_test_config();
        let runner = VersionCmdRunner::new(&config);

        let params = VersionCmdParams {
            project: "TEST".to_string(),
            version_id: Some("10000".to_string()),
            version_archived: Some(true),
            ..Default::default()
        };

        let result = runner.update_jira_version(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_version_cmd_runner_delete_version() {
        let config = create_test_config();
        let runner = VersionCmdRunner::new(&config);

        let params = VersionCmdParams {
            project: "TEST".to_string(),
            version_id: Some("10000".to_string()),
            ..Default::default()
        };

        let result = runner.delete_jira_version(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_version_cmd_runner_get_related_work() {
        let config = create_test_config();
        let runner = VersionCmdRunner::new(&config);

        let params = VersionCmdParams {
            project: "TEST".to_string(),
            version_id: Some("10000".to_string()),
            ..Default::default()
        };

        let result = runner.get_jira_version_related_work(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_version_cmd_runner_unarchive_version() {
        let config = create_test_config();
        let runner = VersionCmdRunner::new(&config);

        let params = VersionCmdParams {
            project: "TEST".to_string(),
            version_id: Some("10000".to_string()),
            version_archived: Some(false), // Unarchive
            ..Default::default()
        };

        let result = runner.update_jira_version(params).await;
        assert!(result.is_err());
    }

    // ========================================================================
    // AREA 3: PROJECT CMD RUNNER OPERATIONS
    // ========================================================================

    #[tokio::test]
    async fn test_project_cmd_runner_get_issue_types() {
        let config = create_test_config();
        let runner = ProjectCmdRunner::new(&config);

        let params = ProjectCmdParams {
            project_key: Some("TEST".to_string()),
            ..Default::default()
        };

        let result = runner.get_jira_project_issue_types(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_project_cmd_runner_get_issue_type_fields() {
        let config = create_test_config();
        let runner = ProjectCmdRunner::new(&config);

        let params = ProjectCmdParams {
            project_key: Some("TEST".to_string()),
            project_issue_type: Some("10001".to_string()),
            ..Default::default()
        };

        let result = runner.get_jira_project_issue_type_id(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_project_cmd_runner_get_issue_types_multiple_types() {
        let config = create_test_config();
        let runner = ProjectCmdRunner::new(&config);

        // Test with different project keys
        for project in ["TEST", "DEMO", "PROD"] {
            let params = ProjectCmdParams {
                project_key: Some(project.to_string()),
                ..Default::default()
            };

            let result = runner.get_jira_project_issue_types(params).await;
            assert!(result.is_err());
        }
    }

    #[tokio::test]
    async fn test_project_cmd_runner_get_issue_type_fields_multiple_types() {
        let config = create_test_config();
        let runner = ProjectCmdRunner::new(&config);

        // Test with different issue type IDs
        for type_id in ["1", "10001", "10002"] {
            let params = ProjectCmdParams {
                project_key: Some("TEST".to_string()),
                project_issue_type: Some(type_id.to_string()),
                ..Default::default()
            };

            let result = runner.get_jira_project_issue_type_id(params).await;
            assert!(result.is_err());
        }
    }
}
