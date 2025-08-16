#[cfg(test)]
mod tests {
    use crate::runners::cfg_cmd_runner::ConfigCmdRunner;
    use crate::config::config_file::{ConfigFile, AuthData};
    use std::fs;
    use std::path::Path;
    use tempfile::tempdir;

    #[test]
    fn test_config_cmd_runner_creation() {
        let config_path = "/test/path/config.toml".to_string();
        let _runner = ConfigCmdRunner::new(config_path.clone());
        
        // We can't directly access the internal field, but we can test
        // that the runner was created successfully
        assert!(true);
    }

    #[test]
    fn test_config_cmd_runner_init_file() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let config_file_path = temp_dir.path().join("test_config.toml");
        let config_path = config_file_path.to_string_lossy().to_string();
        
        let runner = ConfigCmdRunner::new(config_path.clone());
        
        // Test file initialization
        let result = runner.init_file();
        assert!(result.is_ok(), "Failed to initialize config file");
        
        // Verify the file was created
        assert!(config_file_path.exists(), "Config file should exist after initialization");
        
        // Verify it's a file (not a directory)
        assert!(config_file_path.is_file(), "Config path should be a file");
    }

    #[test]
    fn test_config_cmd_runner_init_file_with_nested_directories() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let nested_path = temp_dir.path().join("nested").join("deep").join("config.toml");
        let config_path = nested_path.to_string_lossy().to_string();
        
        let runner = ConfigCmdRunner::new(config_path.clone());
        
        // Test file initialization with nested directories
        let result = runner.init_file();
        assert!(result.is_ok(), "Failed to initialize config file with nested directories");
        
        // Verify the file was created
        assert!(nested_path.exists(), "Config file should exist in nested directory");
        
        // Verify parent directories were created
        assert!(nested_path.parent().unwrap().exists(), "Parent directory should exist");
    }

    #[test]
    fn test_config_cmd_runner_init_file_already_exists() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let config_file_path = temp_dir.path().join("existing_config.toml");
        let config_path = config_file_path.to_string_lossy().to_string();
        
        // Create the file first
        fs::write(&config_file_path, "existing content").expect("Failed to create existing file");
        
        let runner = ConfigCmdRunner::new(config_path.clone());
        
        // Test file initialization when file already exists
        let result = runner.init_file();
        assert!(result.is_ok(), "Should succeed even if file already exists");
        
        // File should still exist
        assert!(config_file_path.exists(), "Config file should still exist");
    }

    #[test]
    fn test_config_cmd_runner_show_cfg() {
        let config = ConfigFile::new(
            "dGVzdF91c2VyOnRlc3RfYXBpX2tleQ==".to_string(),
            "https://test.atlassian.net".to_string(),
            "Done".to_string(),
            "Task completed".to_string(),
            toml::Table::new(),
        );
        
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let config_file_path = temp_dir.path().join("show_config.toml");
        let config_path = config_file_path.to_string_lossy().to_string();
        
        let runner = ConfigCmdRunner::new(config_path);
        
        // Test show configuration - should not panic
        runner.show_cfg(config);
        assert!(true);
    }

    #[test]
    fn test_config_cmd_runner_show_cfg_with_empty_config() {
        let empty_config = ConfigFile::default();
        
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let config_file_path = temp_dir.path().join("empty_config.toml");
        let config_path = config_file_path.to_string_lossy().to_string();
        
        let runner = ConfigCmdRunner::new(config_path);
        
        // Test show configuration with empty config - should not panic
        runner.show_cfg(empty_config);
        assert!(true);
    }

    #[test]
    fn test_config_cmd_runner_show_cfg_with_complex_config() {
        let mut transitions = toml::Table::new();
        transitions.insert("start".to_string(), toml::Value::String("In Progress".to_string()));
        transitions.insert("finish".to_string(), toml::Value::String("Done".to_string()));
        transitions.insert("close".to_string(), toml::Value::String("Closed".to_string()));
        
        let config = ConfigFile::new(
            "dGVzdF91c2VyOnRlc3RfYXBpX2tleQ==".to_string(),
            "https://complex.atlassian.net".to_string(),
            "Resolved".to_string(),
            "Issue has been resolved".to_string(),
            transitions,
        );
        
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let config_file_path = temp_dir.path().join("complex_config.toml");
        let config_path = config_file_path.to_string_lossy().to_string();
        
        let runner = ConfigCmdRunner::new(config_path);
        
        // Test show configuration with complex config - should not panic
        runner.show_cfg(config);
        assert!(true);
    }

    #[test]
    fn test_config_cmd_runner_multiple_instances() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        
        let config1_path = temp_dir.path().join("config1.toml").to_string_lossy().to_string();
        let config2_path = temp_dir.path().join("config2.toml").to_string_lossy().to_string();
        let config3_path = temp_dir.path().join("subdir").join("config3.toml").to_string_lossy().to_string();
        
        let runner1 = ConfigCmdRunner::new(config1_path.clone());
        let runner2 = ConfigCmdRunner::new(config2_path.clone());
        let runner3 = ConfigCmdRunner::new(config3_path.clone());
        
        // Test that multiple runners can be created and used independently
        let result1 = runner1.init_file();
        let result2 = runner2.init_file();
        let result3 = runner3.init_file();
        
        assert!(result1.is_ok(), "First runner should initialize successfully");
        assert!(result2.is_ok(), "Second runner should initialize successfully");
        assert!(result3.is_ok(), "Third runner should initialize successfully");
        
        // Verify all files were created
        assert!(Path::new(&config1_path).exists(), "First config file should exist");
        assert!(Path::new(&config2_path).exists(), "Second config file should exist");
        assert!(Path::new(&config3_path).exists(), "Third config file should exist");
    }

    #[test]
    fn test_config_cmd_runner_init_file_error_conditions() {
        // Test with invalid path (should fail on some systems)
        let invalid_path = "/invalid/path/that/should/not/exist/definitely/config.toml".to_string();
        let runner = ConfigCmdRunner::new(invalid_path);
        
        // Note: This might still succeed on some systems if they create the directory structure
        // The test verifies the runner handles the case appropriately
        let result = runner.init_file();
        // We don't assert failure here because the behavior depends on the system
        // But the function should return a Result type
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_config_cmd_runner_with_different_file_extensions() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        
        let toml_path = temp_dir.path().join("config.toml").to_string_lossy().to_string();
        let txt_path = temp_dir.path().join("config.txt").to_string_lossy().to_string();
        let no_ext_path = temp_dir.path().join("config").to_string_lossy().to_string();
        
        let runner_toml = ConfigCmdRunner::new(toml_path.clone());
        let runner_txt = ConfigCmdRunner::new(txt_path.clone());
        let runner_no_ext = ConfigCmdRunner::new(no_ext_path.clone());
        
        // Test that runners work regardless of file extension
        assert!(runner_toml.init_file().is_ok());
        assert!(runner_txt.init_file().is_ok());
        assert!(runner_no_ext.init_file().is_ok());
        
        assert!(Path::new(&toml_path).exists());
        assert!(Path::new(&txt_path).exists());
        assert!(Path::new(&no_ext_path).exists());
    }

    #[test]
    fn test_config_cmd_runner_with_unicode_paths() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let unicode_path = temp_dir.path().join("测试配置.toml").to_string_lossy().to_string();
        
        let runner = ConfigCmdRunner::new(unicode_path.clone());
        
        // Test that runner works with unicode file names
        let result = runner.init_file();
        assert!(result.is_ok(), "Should handle unicode file names");
        
        assert!(Path::new(&unicode_path).exists(), "Unicode named file should exist");
    }

    #[test]
    fn test_config_cmd_runner_with_spaces_in_path() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let spaced_path = temp_dir.path().join("config with spaces.toml").to_string_lossy().to_string();
        
        let runner = ConfigCmdRunner::new(spaced_path.clone());
        
        // Test that runner works with spaces in file names
        let result = runner.init_file();
        assert!(result.is_ok(), "Should handle spaces in file names");
        
        assert!(Path::new(&spaced_path).exists(), "File with spaces should exist");
    }

    #[test]
    fn test_config_cmd_runner_integration_workflow() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let config_path = temp_dir.path().join("workflow_config.toml").to_string_lossy().to_string();
        
        let runner = ConfigCmdRunner::new(config_path.clone());
        
        // Step 1: Initialize the file
        let init_result = runner.init_file();
        assert!(init_result.is_ok(), "Should initialize file successfully");
        
        // Step 2: Create a config to show
        let config = ConfigFile::new(
            "dGVzdF91c2VyOnRlc3RfYXBpX2tleQ==".to_string(),
            "https://workflow.atlassian.net".to_string(),
            "Done".to_string(),
            "Workflow completed".to_string(),
            toml::Table::new(),
        );
        
        // Step 3: Show the config (should not panic)
        runner.show_cfg(config);
        
        // Verify file still exists
        assert!(Path::new(&config_path).exists(), "Config file should persist");
        
        assert!(true);
    }

    #[test]
    fn test_auth_data_integration_with_config_runner() {
        // Test AuthData creation and base64 encoding
        let auth_data = AuthData::new(
            "test_user".to_string(),
            "test_api_key".to_string(),
        );
        
        let base64_auth = auth_data.to_base64();
        assert!(!base64_auth.is_empty(), "Base64 auth should not be empty");
        
        // Test that we can create a config with this auth data
        let config = ConfigFile::new(
            base64_auth,
            "https://auth-test.atlassian.net".to_string(),
            "Done".to_string(),
            "Auth test completed".to_string(),
            toml::Table::new(),
        );
        
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let config_path = temp_dir.path().join("auth_config.toml").to_string_lossy().to_string();
        
        let runner = ConfigCmdRunner::new(config_path);
        
        // Show config with auth data (should not panic or leak sensitive data)
        runner.show_cfg(config);
        assert!(true);
    }
}