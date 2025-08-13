#[cfg(test)]
mod tests {
    use crate::args::commands::{
        Commands, ConfigArgs, ConfigActionValues, IssueArgs, IssueActionValues,
        LinkIssueActionValues, ProjectActionValues,
        TransitionActionValues, VersionArgs, VersionActionValues,
        OutputArgs, OutputValues, OutputTypes, PaginationArgs
    };

    #[test]
    fn test_output_values_enum() {
        // Test that OutputValues variants exist and can be used
        let table_value = OutputValues::Table;
        let json_value = OutputValues::Json;
        
        // Test Debug trait
        assert!(format!("{:?}", table_value).contains("Table"));
        assert!(format!("{:?}", json_value).contains("Json"));
        
        // Test Clone trait
        let cloned_table = table_value.clone();
        let cloned_json = json_value.clone();
        
        assert!(matches!(cloned_table, OutputValues::Table));
        assert!(matches!(cloned_json, OutputValues::Json));
    }

    #[test]
    fn test_output_types_enum() {
        // Test that OutputTypes variants exist and can be used
        let basic_type = OutputTypes::Basic;
        let single_type = OutputTypes::Single;
        let full_type = OutputTypes::Full;
        
        // Test Debug trait
        assert!(format!("{:?}", basic_type).contains("Basic"));
        assert!(format!("{:?}", single_type).contains("Single"));
        assert!(format!("{:?}", full_type).contains("Full"));
        
        // Test Clone trait
        let cloned_basic = basic_type.clone();
        let cloned_single = single_type.clone();
        let cloned_full = full_type.clone();
        
        assert!(matches!(cloned_basic, OutputTypes::Basic));
        assert!(matches!(cloned_single, OutputTypes::Single));
        assert!(matches!(cloned_full, OutputTypes::Full));
    }

    #[test]
    fn test_pagination_args_creation() {
        let pagination = PaginationArgs {
            page_size: Some(50),
            page_offset: Some(100),
        };
        
        assert_eq!(pagination.page_size, Some(50));
        assert_eq!(pagination.page_offset, Some(100));
    }

    #[test]
    fn test_pagination_args_none_values() {
        let pagination = PaginationArgs {
            page_size: None,
            page_offset: None,
        };
        
        assert!(pagination.page_size.is_none());
        assert!(pagination.page_offset.is_none());
    }

    #[test]
    fn test_output_args_creation() {
        let output = OutputArgs {
            output_format: Some(OutputValues::Json),
            output_type: Some(OutputTypes::Full),
        };
        
        assert!(matches!(output.output_format, Some(OutputValues::Json)));
        assert!(matches!(output.output_type, Some(OutputTypes::Full)));
    }

    #[test]
    fn test_output_args_none_values() {
        let output = OutputArgs {
            output_format: None,
            output_type: None,
        };
        
        assert!(output.output_format.is_none());
        assert!(output.output_type.is_none());
    }

    #[test]
    fn test_config_action_values_enum() {
        // Test all ConfigActionValues variants
        let auth_action = ConfigActionValues::Auth;
        let jira_action = ConfigActionValues::Jira;
        let setup_action = ConfigActionValues::Setup;
        let show_action = ConfigActionValues::Show;
        
        // Test equality
        assert_eq!(auth_action, ConfigActionValues::Auth);
        assert_eq!(jira_action, ConfigActionValues::Jira);
        assert_eq!(setup_action, ConfigActionValues::Setup);
        assert_eq!(show_action, ConfigActionValues::Show);
        
        // Test Debug trait
        assert!(format!("{:?}", auth_action).contains("Auth"));
        assert!(format!("{:?}", jira_action).contains("Jira"));
        assert!(format!("{:?}", setup_action).contains("Setup"));
        assert!(format!("{:?}", show_action).contains("Show"));
    }

    #[test]
    fn test_config_args_creation() {
        let config_args = ConfigArgs {
            cfg_act: ConfigActionValues::Show,
        };
        
        assert_eq!(config_args.cfg_act, ConfigActionValues::Show);
    }

    #[test]
    fn test_commands_enum_variants() {
        // Test all Commands enum variants can be created
        let config_cmd = Commands::Config(ConfigArgs {
            cfg_act: ConfigActionValues::Show,
        });
        
        let issue_cmd = Commands::Issue(IssueArgs {
            issue_act: IssueActionValues::Search,
            project_key: Some("TEST".to_string()),
            issue_key: None,
            issue_fields: None,
            transition_to: None,
            assignee: None,
            pagination: PaginationArgs { page_size: None, page_offset: None },
            output: OutputArgs { output_format: None, output_type: None },
            query: None,
        });
        
        // Test pattern matching works
        match config_cmd {
            Commands::Config(args) => {
                assert_eq!(args.cfg_act, ConfigActionValues::Show);
            }
            _ => panic!("Expected Config command"),
        }
        
        match issue_cmd {
            Commands::Issue(args) => {
                assert_eq!(args.issue_act, IssueActionValues::Search);
                assert_eq!(args.project_key, Some("TEST".to_string()));
            }
            _ => panic!("Expected Issue command"),
        }
    }

    #[test]
    fn test_args_clone_and_debug_traits() {
        let pagination = PaginationArgs {
            page_size: Some(25),
            page_offset: Some(50),
        };
        
        // Test Clone trait
        let cloned_pagination = pagination.clone();
        assert_eq!(cloned_pagination.page_size, Some(25));
        assert_eq!(cloned_pagination.page_offset, Some(50));
        
        // Test Debug trait
        let debug_str = format!("{:?}", pagination);
        assert!(debug_str.contains("PaginationArgs"));
        assert!(debug_str.contains("25"));
        assert!(debug_str.contains("50"));
    }

    #[test]
    fn test_serialization_traits() {
        let output = OutputArgs {
            output_format: Some(OutputValues::Table),
            output_type: Some(OutputTypes::Basic),
        };
        
        // Test serialization
        let serialized = serde_json::to_string(&output).expect("Should serialize");
        assert!(serialized.contains("Table") || serialized.contains("table"));
        assert!(serialized.contains("Basic") || serialized.contains("basic"));
        
        // Test deserialization
        let deserialized: OutputArgs = serde_json::from_str(&serialized).expect("Should deserialize");
        assert!(matches!(deserialized.output_format, Some(OutputValues::Table)));
        assert!(matches!(deserialized.output_type, Some(OutputTypes::Basic)));
    }

    #[test]
    fn test_complex_command_structures() {
        let version_args = VersionArgs {
            version_act: VersionActionValues::List,
            project_key: "COMPLEX".to_string(),
            project_id: Some("10000".to_string()),
            version_id: Some("1000".to_string()),
            version_name: Some("v1.0.0".to_string()),
            version_description: Some("Complex version".to_string()),
            version_start_date: None,
            version_release_date: None,
            version_archived: Some(false),
            version_released: Some(true),
            changelog_file: Some("/path/to/changelog.md".to_string()),
            pagination: PaginationArgs {
                page_size: Some(100),
                page_offset: Some(200),
            },
            output: OutputArgs {
                output_format: Some(OutputValues::Json),
                output_type: Some(OutputTypes::Full),
            },
            transition_assignee: Some("test-user".to_string()),
            transition_issues: Some(true),
        };
        
        assert_eq!(version_args.project_key, "COMPLEX");
        assert_eq!(version_args.version_name, Some("v1.0.0".to_string()));
        assert_eq!(version_args.version_archived, Some(false));
        assert_eq!(version_args.version_released, Some(true));
        assert_eq!(version_args.pagination.page_size, Some(100));
        assert!(matches!(version_args.output.output_format, Some(OutputValues::Json)));
    }

    #[test]
    fn test_command_cloning() {
        let config_cmd = Commands::Config(ConfigArgs {
            cfg_act: ConfigActionValues::Auth,
        });
        
        let cloned_cmd = config_cmd.clone();
        
        match cloned_cmd {
            Commands::Config(args) => {
                assert_eq!(args.cfg_act, ConfigActionValues::Auth);
            }
            _ => panic!("Cloned command should be Config"),
        }
    }

    #[test]
    fn test_all_action_value_enums() {
        // Test that all action enum variants work
        let _config_actions = vec![
            ConfigActionValues::Auth,
            ConfigActionValues::Jira,
            ConfigActionValues::Setup,
            ConfigActionValues::Show,
        ];
        
        let _issue_actions = vec![
            IssueActionValues::Assign,
            IssueActionValues::Create,
            IssueActionValues::Delete,
            IssueActionValues::Get,
            IssueActionValues::Search,
            IssueActionValues::Transition,
            IssueActionValues::Update,
        ];
        
        let _project_actions = vec![
            ProjectActionValues::List,
            ProjectActionValues::Show,
            ProjectActionValues::Create,
        ];
        
        let _version_actions = vec![
            VersionActionValues::List,
            VersionActionValues::Create,
            VersionActionValues::Update,
            VersionActionValues::Release,
            VersionActionValues::Archive,
        ];
        
        let _transition_actions = vec![
            TransitionActionValues::List,
            TransitionActionValues::Apply,
        ];
        
        let _link_actions = vec![
            LinkIssueActionValues::Add,
            LinkIssueActionValues::Remove,
        ];
        
        // Just ensure all enums can be created without panicking
        assert!(true);
    }

    #[test]
    fn test_edge_case_values() {
        // Test with edge case values
        let pagination_zero = PaginationArgs {
            page_size: Some(0),
            page_offset: Some(0),
        };
        
        assert_eq!(pagination_zero.page_size, Some(0));
        assert_eq!(pagination_zero.page_offset, Some(0));
        
        let pagination_large = PaginationArgs {
            page_size: Some(1000000),
            page_offset: Some(9223372036854775807), // i64::MAX
        };
        
        assert_eq!(pagination_large.page_size, Some(1000000));
        assert_eq!(pagination_large.page_offset, Some(9223372036854775807));
    }

    #[test]
    fn test_empty_string_values() {
        // Test args that accept string values work with empty strings
        let issue_args = IssueArgs {
            issue_act: IssueActionValues::Create,
            project_key: Some("".to_string()),
            issue_key: Some("".to_string()),
            issue_fields: None,
            transition_to: None,
            assignee: None,
            pagination: PaginationArgs { page_size: None, page_offset: None },
            output: OutputArgs { output_format: None, output_type: None },
            query: None,
        };
        
        assert_eq!(issue_args.project_key, Some("".to_string()));
        assert_eq!(issue_args.issue_key, Some("".to_string()));
    }
}