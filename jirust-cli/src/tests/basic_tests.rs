#[cfg(test)]
mod tests {
    use crate::{
        args::commands::{Commands, ConfigArgs, ConfigActionValues},
        config::config_file::{AuthData, ConfigFile},
        utils::PrintableData,
    };
    use serde_json::json;
    use toml::{Table, Value};

    fn create_test_config() -> ConfigFile {
        let auth_data = AuthData::new("testuser".to_string(), "testtoken".to_string());
        let auth_token = auth_data.to_base64();

        let mut transitions = Table::new();
        transitions.insert("done".to_string(), Value::String("Done".to_string()));
        
        ConfigFile::new(
            auth_token,
            "https://test-jira.atlassian.net".to_string(),
            "Done".to_string(),
            "Won't Do".to_string(),
            transitions,
        )
    }

    #[test]
    fn test_auth_data_basic() {
        let auth_data = AuthData::new("testuser".to_string(), "testtoken".to_string());
        let base64_token = auth_data.to_base64();
        
        assert!(!base64_token.is_empty());
        
        let (username, api_key) = AuthData::from_base64(&base64_token);
        assert_eq!(username, "testuser");
        assert_eq!(api_key, "testtoken");
    }

    #[test]
    fn test_config_file_basic() {
        let config = create_test_config();
        
        assert_eq!(config.get_jira_url(), "https://test-jira.atlassian.net");
        assert_eq!(config.get_standard_resolution(), "Done");
        assert_eq!(config.get_standard_resolution_comment(), "Won't Do");
        assert!(!config.get_auth_key().is_empty());
    }

    #[test]
    fn test_config_file_setters() {
        let mut config = create_test_config();
        
        config.set_jira_url("https://new-url.atlassian.net".to_string());
        assert_eq!(config.get_jira_url(), "https://new-url.atlassian.net");
        
        config.set_standard_resolution("Resolved".to_string());
        assert_eq!(config.get_standard_resolution(), "Resolved");
    }

    #[test]
    fn test_printable_data_variants() {
        let generic_data = PrintableData::Generic { 
            data: vec![json!({"test": "value"})] 
        };
        
        match generic_data {
            PrintableData::Generic { data } => {
                assert_eq!(data.len(), 1);
            }
            _ => panic!("Expected Generic variant"),
        }

        let project_data = PrintableData::Project { projects: vec![] };
        match project_data {
            PrintableData::Project { projects } => {
                assert_eq!(projects.len(), 0);
            }
            _ => panic!("Expected Project variant"),
        }
    }

    #[test]
    fn test_config_args() {
        let config_args = ConfigArgs {
            cfg_act: ConfigActionValues::Show,
        };

        assert_eq!(config_args.cfg_act, ConfigActionValues::Show);
    }

    #[test]
    fn test_commands_enum() {
        let command = Commands::Config(ConfigArgs {
            cfg_act: ConfigActionValues::Setup,
        });

        match command {
            Commands::Config(args) => {
                assert_eq!(args.cfg_act, ConfigActionValues::Setup);
            }
            _ => panic!("Expected Config command"),
        }
    }

    #[test]
    fn test_config_serialization() {
        let config = create_test_config();
        
        let toml_result = toml::to_string(&config);
        assert!(toml_result.is_ok());
        
        if let Ok(toml_string) = toml_result {
            assert!(toml_string.contains("[auth]"));
            assert!(toml_string.contains("[jira]"));
        }
    }

    #[test]
    fn test_auth_data_setters() {
        let mut auth_data = AuthData::new("original".to_string(), "original".to_string());
        
        auth_data.set_username("updated_user".to_string());
        auth_data.set_api_key("updated_key".to_string());

        let base64_token = auth_data.to_base64();
        let (username, api_key) = AuthData::from_base64(&base64_token);
        assert_eq!(username, "updated_user");
        assert_eq!(api_key, "updated_key");
    }

    #[test]
    fn test_config_transitions() {
        let mut config = create_test_config();
        
        config.add_transition_name("review".to_string(), "In Review".to_string());
        let transitions = config.get_transition_name("review");
        
        assert!(transitions.is_some());
    }
}