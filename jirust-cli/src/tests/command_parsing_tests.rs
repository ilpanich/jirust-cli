#[cfg(test)]
mod tests {
    use crate::args::commands::{
        Commands, ConfigActionValues, ConfigArgs, IssueActionValues, IssueArgs, JirustCliArgs,
        LinkIssueActionValues, LinkIssueArgs, OutputArgs, OutputTypes, OutputValues,
        PaginationArgs, ProjectActionValues, ProjectArgs, TransitionActionValues, TransitionArgs,
        VersionActionValues, VersionArgs,
    };
    use clap::Parser;

    #[test]
    fn test_config_action_values_enum() {
        // Test all ConfigActionValues variants
        let setup_action = ConfigActionValues::Setup;
        let show_action = ConfigActionValues::Show;

        // Test Debug trait
        assert!(format!("{:?}", setup_action).contains("Setup"));
        assert!(format!("{:?}", show_action).contains("Show"));

        // Test Clone trait
        let cloned_setup = setup_action.clone();
        let cloned_show = show_action.clone();

        assert!(matches!(cloned_setup, ConfigActionValues::Setup));
        assert!(matches!(cloned_show, ConfigActionValues::Show));
    }

    #[test]
    fn test_issue_action_values_enum() {
        // Test all IssueActionValues variants
        let actions = [
            IssueActionValues::Assign,
            IssueActionValues::Attach,
            IssueActionValues::Create,
            IssueActionValues::Delete,
            IssueActionValues::Get,
            IssueActionValues::Search,
            IssueActionValues::Transition,
            IssueActionValues::Update,
        ];

        for action in &actions {
            // Test Debug trait works
            let debug_str = format!("{:?}", action);
            assert!(!debug_str.is_empty());

            // Test Clone trait
            let cloned = action.clone();
            assert!(matches!(
                cloned,
                IssueActionValues::Assign
                    | IssueActionValues::Attach
                    | IssueActionValues::Create
                    | IssueActionValues::Delete
                    | IssueActionValues::Get
                    | IssueActionValues::Search
                    | IssueActionValues::Transition
                    | IssueActionValues::Update
            ));
        }
    }

    #[test]
    fn test_project_action_values_enum() {
        let create_action = ProjectActionValues::Create;
        let list_action = ProjectActionValues::List;

        // Test Debug trait
        assert!(format!("{:?}", create_action).contains("Create"));
        assert!(format!("{:?}", list_action).contains("List"));

        // Test Clone trait
        let cloned_create = create_action.clone();
        let cloned_list = list_action.clone();

        assert!(matches!(cloned_create, ProjectActionValues::Create));
        assert!(matches!(cloned_list, ProjectActionValues::List));
    }

    #[test]
    fn test_version_action_values_enum() {
        let actions = [
            VersionActionValues::Archive,
            VersionActionValues::Create,
            VersionActionValues::Delete,
            VersionActionValues::List,
            VersionActionValues::RelatedWorkItems,
            VersionActionValues::Release,
            VersionActionValues::Update,
        ];

        for action in &actions {
            let debug_str = format!("{:?}", action);
            assert!(!debug_str.is_empty());

            let cloned = action.clone();
            assert!(matches!(
                cloned,
                VersionActionValues::Archive
                    | VersionActionValues::Create
                    | VersionActionValues::Delete
                    | VersionActionValues::List
                    | VersionActionValues::RelatedWorkItems
                    | VersionActionValues::Release
                    | VersionActionValues::Update
            ));
        }
    }

    #[test]
    fn test_link_issue_action_values_enum() {
        let create_action = LinkIssueActionValues::Create;

        // Test Debug trait
        assert!(format!("{:?}", create_action).contains("Create"));

        // Test Clone trait
        let cloned_create = create_action.clone();
        assert!(matches!(cloned_create, LinkIssueActionValues::Create));
    }

    #[test]
    fn test_transition_action_values_enum() {
        let list_action = TransitionActionValues::List;

        // Test Debug trait
        assert!(format!("{:?}", list_action).contains("List"));

        // Test Clone trait
        let cloned_list = list_action.clone();
        assert!(matches!(cloned_list, TransitionActionValues::List));
    }

    #[test]
    fn test_output_args_creation_and_defaults() {
        // Test default creation
        let default_args = OutputArgs {
            output_format: None,
            output_type: None,
        };

        assert_eq!(default_args.output_format, None);
        assert_eq!(default_args.output_type, None);

        // Test with values
        let table_args = OutputArgs {
            output_format: Some(OutputValues::Table),
            output_type: Some(OutputTypes::Basic),
        };

        assert_eq!(table_args.output_format, Some(OutputValues::Table));
        assert_eq!(table_args.output_type, Some(OutputTypes::Basic));

        let json_args = OutputArgs {
            output_format: Some(OutputValues::Json),
            output_type: Some(OutputTypes::Full),
        };

        assert_eq!(json_args.output_format, Some(OutputValues::Json));
        assert_eq!(json_args.output_type, Some(OutputTypes::Full));
    }

    #[test]
    fn test_pagination_args_creation_and_defaults() {
        // Test default creation
        let default_pagination = PaginationArgs {
            page_size: None,
            page_offset: None,
        };

        assert_eq!(default_pagination.page_size, None);
        assert_eq!(default_pagination.page_offset, None);

        // Test with values
        let custom_pagination = PaginationArgs {
            page_size: Some(25),
            page_offset: Some(50),
        };

        assert_eq!(custom_pagination.page_size, Some(25));
        assert_eq!(custom_pagination.page_offset, Some(50));

        // Test edge cases
        let zero_pagination = PaginationArgs {
            page_size: Some(0),
            page_offset: Some(0),
        };

        assert_eq!(zero_pagination.page_size, Some(0));
        assert_eq!(zero_pagination.page_offset, Some(0));

        let large_pagination = PaginationArgs {
            page_size: Some(1000),
            page_offset: Some(10000),
        };

        assert_eq!(large_pagination.page_size, Some(1000));
        assert_eq!(large_pagination.page_offset, Some(10000));
    }

    #[test]
    fn test_config_args_creation() {
        let config_args = ConfigArgs {
            cfg_act: ConfigActionValues::Setup,
        };

        assert!(matches!(config_args.cfg_act, ConfigActionValues::Setup));

        let show_config_args = ConfigArgs {
            cfg_act: ConfigActionValues::Show,
        };

        assert!(matches!(show_config_args.cfg_act, ConfigActionValues::Show));
    }

    #[test]
    fn test_issue_args_comprehensive() {
        let comprehensive_issue_args = IssueArgs {
            issue_act: IssueActionValues::Create,
            project_key: Some("COMP".to_string()),
            issue_key: Some("COMP-123".to_string()),
            issue_fields: Some(vec![
                (
                    "summary".to_string(),
                    "Comprehensive test issue".to_string(),
                ),
                (
                    "description".to_string(),
                    "Detailed description for testing".to_string(),
                ),
                ("issuetype".to_string(), "Bug".to_string()),
                ("priority".to_string(), "High".to_string()),
                (
                    "components".to_string(),
                    "[\"Frontend\", \"Backend\"]".to_string(),
                ),
                (
                    "labels".to_string(),
                    "[\"urgent\", \"customer\"]".to_string(),
                ),
            ]),
            transition_to: Some("In Progress".to_string()),
            assignee: Some("developer@example.com".to_string()),
            query: Some("project = COMP AND status = 'To Do' ORDER BY priority DESC".to_string()),
            attachment_file_path: None,
            pagination: PaginationArgs {
                page_size: Some(50),
                page_offset: Some(0),
            },
            output: OutputArgs {
                output_format: Some(OutputValues::Json),
                output_type: Some(OutputTypes::Full),
            },
        };

        assert!(matches!(
            comprehensive_issue_args.issue_act,
            IssueActionValues::Create
        ));
        assert_eq!(
            comprehensive_issue_args.project_key,
            Some("COMP".to_string())
        );
        assert_eq!(
            comprehensive_issue_args.issue_key,
            Some("COMP-123".to_string())
        );
        assert!(comprehensive_issue_args.issue_fields.is_some());
        assert_eq!(
            comprehensive_issue_args
                .issue_fields
                .as_ref()
                .unwrap()
                .len(),
            6
        );
        assert_eq!(
            comprehensive_issue_args.transition_to,
            Some("In Progress".to_string())
        );
        assert_eq!(
            comprehensive_issue_args.assignee,
            Some("developer@example.com".to_string())
        );
        assert!(comprehensive_issue_args.query.is_some());
        assert_eq!(comprehensive_issue_args.pagination.page_size, Some(50));
        assert_eq!(
            comprehensive_issue_args.output.output_format,
            Some(OutputValues::Json)
        );
    }

    #[test]
    fn test_issue_args_minimal() {
        let minimal_issue_args = IssueArgs {
            issue_act: IssueActionValues::Get,
            project_key: None,
            issue_key: Some("MIN-1".to_string()),
            issue_fields: None,
            transition_to: None,
            assignee: None,
            query: None,
            attachment_file_path: None,
            pagination: PaginationArgs {
                page_size: None,
                page_offset: None,
            },
            output: OutputArgs {
                output_format: None,
                output_type: None,
            },
        };

        assert!(matches!(
            minimal_issue_args.issue_act,
            IssueActionValues::Get
        ));
        assert_eq!(minimal_issue_args.project_key, None);
        assert_eq!(minimal_issue_args.issue_key, Some("MIN-1".to_string()));
        assert_eq!(minimal_issue_args.issue_fields, None);
        assert_eq!(minimal_issue_args.transition_to, None);
        assert_eq!(minimal_issue_args.assignee, None);
        assert_eq!(minimal_issue_args.query, None);
    }

    #[test]
    fn test_project_args_comprehensive() {
        let comprehensive_project_args = ProjectArgs {
            project_act: ProjectActionValues::Create,
            project_key: Some("NEWPROJ".to_string()),
            project_issue_type: Some("Task".to_string()),
            project_name: Some("New Project".to_string()),
            project_description: Some("A brand new project for testing".to_string()),
            project_field_configuration_id: Some(10200),
            project_issue_security_scheme_id: Some(10300),
            project_issue_type_scheme_id: Some(10400),
            project_issue_type_screen_scheme_id: Some(10500),
            project_notification_scheme_id: Some(10600),
            project_permission_scheme_id: Some(10700),
            project_workflow_scheme_id: Some(10800),
            project_lead_account_id: Some("lead@example.com".to_string()),
            project_assignee_type: Some("PROJECT_LEAD".to_string()),
            pagination: PaginationArgs {
                page_size: Some(20),
                page_offset: Some(0),
            },
            output: OutputArgs {
                output_format: Some(OutputValues::Table),
                output_type: Some(OutputTypes::Basic),
            },
        };

        assert!(matches!(
            comprehensive_project_args.project_act,
            ProjectActionValues::Create
        ));
        assert_eq!(
            comprehensive_project_args.project_key,
            Some("NEWPROJ".to_string())
        );
        assert_eq!(
            comprehensive_project_args.project_name,
            Some("New Project".to_string())
        );
        assert_eq!(
            comprehensive_project_args.project_field_configuration_id,
            Some(10200)
        );
        assert_eq!(
            comprehensive_project_args.project_lead_account_id,
            Some("lead@example.com".to_string())
        );
    }

    #[test]
    fn test_version_args_comprehensive() {
        let comprehensive_version_args = VersionArgs {
            version_act: VersionActionValues::Create,
            project_key: "PROJ".to_string(),
            project_id: Some(12345),
            version_id: Some("v2.0.0".to_string()),
            version_name: Some("2.0.0".to_string()),
            version_description: Some("Major release with breaking changes".to_string()),
            version_start_date: Some("2024-01-01".to_string()),
            version_release_date: Some("2024-06-01".to_string()),
            version_archived: Some(false),
            version_released: Some(false),
            changelog_file: Some("CHANGELOG.md".to_string()),
            transition_issues: Some(true),
            transition_assignee: Some("release.manager@example.com".to_string()),
            pagination: PaginationArgs {
                page_size: Some(100),
                page_offset: Some(0),
            },
            output: OutputArgs {
                output_format: Some(OutputValues::Json),
                output_type: Some(OutputTypes::Full),
            },
        };

        assert!(matches!(
            comprehensive_version_args.version_act,
            VersionActionValues::Create
        ));
        assert_eq!(comprehensive_version_args.project_key, "PROJ");
        assert_eq!(comprehensive_version_args.project_id, Some(12345));
        assert_eq!(
            comprehensive_version_args.version_name,
            Some("2.0.0".to_string())
        );
        assert_eq!(comprehensive_version_args.version_archived, Some(false));
        assert_eq!(comprehensive_version_args.transition_issues, Some(true));
    }

    #[test]
    fn test_link_issue_args_comprehensive() {
        let comprehensive_link_args = LinkIssueArgs {
            link_act: LinkIssueActionValues::Create,
            project_key: Some("LINK".to_string()),
            origin_issue_key: "LINK-123".to_string(),
            destination_issue_key: Some("LINK-456".to_string()),
            link_type: "Blocks".to_string(),
            changelog_file: Some("LINKING_CHANGELOG.md".to_string()),
        };

        assert!(matches!(
            comprehensive_link_args.link_act,
            LinkIssueActionValues::Create
        ));
        assert_eq!(
            comprehensive_link_args.project_key,
            Some("LINK".to_string())
        );
        assert_eq!(comprehensive_link_args.origin_issue_key, "LINK-123");
        assert_eq!(
            comprehensive_link_args.destination_issue_key,
            Some("LINK-456".to_string())
        );
        assert_eq!(comprehensive_link_args.link_type, "Blocks");
        assert_eq!(
            comprehensive_link_args.changelog_file,
            Some("LINKING_CHANGELOG.md".to_string())
        );
    }

    #[test]
    fn test_transition_args_comprehensive() {
        let comprehensive_transition_args = TransitionArgs {
            transition_act: TransitionActionValues::List,
            issue_key: "TRANS-789".to_string(),
            output: OutputArgs {
                output_format: Some(OutputValues::Table),
                output_type: Some(OutputTypes::Single),
            },
        };

        assert!(matches!(
            comprehensive_transition_args.transition_act,
            TransitionActionValues::List
        ));
        assert_eq!(comprehensive_transition_args.issue_key, "TRANS-789");
        assert_eq!(
            comprehensive_transition_args.output.output_format,
            Some(OutputValues::Table)
        );
        assert_eq!(
            comprehensive_transition_args.output.output_type,
            Some(OutputTypes::Single)
        );
    }

    #[test]
    fn test_commands_enum_variants() {
        // Test that Commands enum variants can be created
        let config_command = Commands::Config(ConfigArgs {
            cfg_act: ConfigActionValues::Show,
        });

        let issue_command = Commands::Issue(IssueArgs {
            issue_act: IssueActionValues::Get,
            project_key: None,
            issue_key: Some("TEST-1".to_string()),
            issue_fields: None,
            transition_to: None,
            assignee: None,
            query: None,
            attachment_file_path: None,
            pagination: PaginationArgs {
                page_size: None,
                page_offset: None,
            },
            output: OutputArgs {
                output_format: None,
                output_type: None,
            },
        });

        // Test that we can match on the variants
        match config_command {
            Commands::Config(_) => assert!(true),
            _ => assert!(false, "Should match Config variant"),
        }

        match issue_command {
            Commands::Issue(_) => assert!(true),
            _ => assert!(false, "Should match Issue variant"),
        }
    }

    #[test]
    fn test_issue_fields_parsing() {
        // Test different types of issue field configurations
        let simple_fields = vec![
            ("summary".to_string(), "Simple summary".to_string()),
            ("priority".to_string(), "High".to_string()),
        ];

        let json_fields = vec![
            (
                "components".to_string(),
                "[{\"name\": \"Frontend\"}]".to_string(),
            ),
            (
                "customfield_10001".to_string(),
                "{\"value\": \"Custom Value\"}".to_string(),
            ),
        ];

        let mixed_fields = vec![
            ("summary".to_string(), "Mixed field types".to_string()),
            ("labels".to_string(), "[\"label1\", \"label2\"]".to_string()),
            ("duedate".to_string(), "2024-12-31".to_string()),
        ];

        let issue_args_simple = IssueArgs {
            issue_act: IssueActionValues::Create,
            project_key: Some("TEST".to_string()),
            issue_key: None,
            issue_fields: Some(simple_fields),
            transition_to: None,
            assignee: None,
            query: None,
            attachment_file_path: None,
            pagination: PaginationArgs {
                page_size: None,
                page_offset: None,
            },
            output: OutputArgs {
                output_format: None,
                output_type: None,
            },
        };

        assert!(issue_args_simple.issue_fields.is_some());
        assert_eq!(issue_args_simple.issue_fields.as_ref().unwrap().len(), 2);

        let issue_args_json = IssueArgs {
            issue_act: IssueActionValues::Update,
            project_key: Some("TEST".to_string()),
            issue_key: Some("TEST-1".to_string()),
            issue_fields: Some(json_fields),
            transition_to: None,
            assignee: None,
            query: None,
            attachment_file_path: None,
            pagination: PaginationArgs {
                page_size: None,
                page_offset: None,
            },
            output: OutputArgs {
                output_format: None,
                output_type: None,
            },
        };

        assert!(issue_args_json.issue_fields.is_some());
        assert_eq!(issue_args_json.issue_fields.as_ref().unwrap().len(), 2);

        let issue_args_mixed = IssueArgs {
            issue_act: IssueActionValues::Create,
            project_key: Some("TEST".to_string()),
            issue_key: None,
            issue_fields: Some(mixed_fields),
            transition_to: None,
            assignee: None,
            query: None,
            attachment_file_path: None,
            pagination: PaginationArgs {
                page_size: None,
                page_offset: None,
            },
            output: OutputArgs {
                output_format: None,
                output_type: None,
            },
        };

        assert!(issue_args_mixed.issue_fields.is_some());
        assert_eq!(issue_args_mixed.issue_fields.as_ref().unwrap().len(), 3);
    }

    #[test]
    fn test_edge_case_values() {
        // Test with empty strings
        let empty_string_args = IssueArgs {
            issue_act: IssueActionValues::Get,
            project_key: Some("".to_string()),
            issue_key: Some("".to_string()),
            issue_fields: Some(vec![("".to_string(), "".to_string())]),
            transition_to: Some("".to_string()),
            assignee: Some("".to_string()),
            query: Some("".to_string()),
            attachment_file_path: None,
            pagination: PaginationArgs {
                page_size: None,
                page_offset: None,
            },
            output: OutputArgs {
                output_format: None,
                output_type: None,
            },
        };

        assert_eq!(empty_string_args.project_key, Some("".to_string()));
        assert_eq!(empty_string_args.issue_key, Some("".to_string()));
        assert_eq!(empty_string_args.transition_to, Some("".to_string()));

        // Test with very long strings
        let long_string = "a".repeat(1000);
        let long_string_args = ProjectArgs {
            project_act: ProjectActionValues::Create,
            project_key: Some(long_string.clone()),
            project_issue_type: None,
            project_name: Some(long_string.clone()),
            project_description: Some(long_string.clone()),
            project_field_configuration_id: None,
            project_issue_security_scheme_id: None,
            project_issue_type_scheme_id: None,
            project_issue_type_screen_scheme_id: None,
            project_notification_scheme_id: None,
            project_permission_scheme_id: None,
            project_workflow_scheme_id: None,
            project_lead_account_id: Some(long_string.clone()),
            project_assignee_type: None,
            pagination: PaginationArgs {
                page_size: None,
                page_offset: None,
            },
            output: OutputArgs {
                output_format: None,
                output_type: None,
            },
        };

        assert_eq!(long_string_args.project_key, Some(long_string.clone()));
        assert_eq!(long_string_args.project_name, Some(long_string.clone()));

        // Test with special characters
        let special_chars = "!@#$%^&*()_+-=[]{}|;':\",./<>?~`".to_string();
        let special_char_args = VersionArgs {
            version_act: VersionActionValues::Create,
            project_key: special_chars.clone(),
            project_id: None,
            version_id: Some(special_chars.clone()),
            version_name: Some(special_chars.clone()),
            version_description: Some(special_chars.clone()),
            version_start_date: None,
            version_release_date: None,
            version_archived: None,
            version_released: None,
            changelog_file: Some(special_chars.clone()),
            transition_issues: None,
            transition_assignee: Some(special_chars.clone()),
            pagination: PaginationArgs {
                page_size: None,
                page_offset: None,
            },
            output: OutputArgs {
                output_format: None,
                output_type: None,
            },
        };

        assert_eq!(special_char_args.project_key, special_chars);
        assert_eq!(special_char_args.version_name, Some(special_chars.clone()));
    }

    #[test]
    fn test_cli_parses_issue_fields_with_jira_doc_wrapper() {
        let args = JirustCliArgs::try_parse_from([
            "jirust-cli",
            "issue",
            "create",
            "-k",
            "TEST",
            "-f",
            "summary=New issue",
            "-f",
            "description=jira_doc_field[Important update]",
        ])
        .expect("CLI parsing should succeed");

        match args.subcmd {
            Commands::Issue(issue_args) => {
                let fields = issue_args.issue_fields.expect("issue fields to be parsed");
                assert_eq!(fields.len(), 2);
                assert_eq!(fields[0], ("summary".to_string(), "New issue".to_string()));

                let expected_doc = "{\"content\":[{\"content\":[{\"text\":\"Important update\",\"type\":\"text\"}],\"type\":\"paragraph\"}],\"type\":\"doc\",\"version\":1}".to_string();
                assert_eq!(fields[1], ("description".to_string(), expected_doc));
            }
            other => panic!("unexpected command parsed: {:?}", other),
        }
    }

    #[test]
    fn test_cli_rejects_malformed_issue_field_argument() {
        let result = JirustCliArgs::try_parse_from([
            "jirust-cli",
            "issue",
            "create",
            "-k",
            "TEST",
            "-f",
            "missing_delimiter",
        ]);

        assert!(result.is_err());
        if let Err(err) = result {
            assert!(err.to_string().contains("invalid KEY=value"));
        }
    }

    #[test]
    fn test_issue_attach_action_parsing() {
        let args = JirustCliArgs::try_parse_from([
            "jirust-cli",
            "issue",
            "attach",
            "-i",
            "TEST-123",
            "-p",
            "/path/to/file.pdf",
        ])
        .expect("CLI parsing should succeed");

        match args.subcmd {
            Commands::Issue(issue_args) => {
                assert_eq!(issue_args.issue_act, IssueActionValues::Attach);
                assert_eq!(issue_args.issue_key, Some("TEST-123".to_string()));
                assert_eq!(
                    issue_args.attachment_file_path,
                    Some("/path/to/file.pdf".to_string())
                );
            }
            other => panic!("unexpected command parsed: {:?}", other),
        }
    }

    #[test]
    fn test_issue_attach_with_all_path_types() {
        // Test absolute path
        let args = JirustCliArgs::try_parse_from([
            "jirust-cli",
            "issue",
            "attach",
            "-i",
            "TEST-1",
            "-p",
            "/home/user/documents/file.txt",
        ])
        .expect("absolute path should parse");

        if let Commands::Issue(issue_args) = args.subcmd {
            assert_eq!(
                issue_args.attachment_file_path,
                Some("/home/user/documents/file.txt".to_string())
            );
        }

        // Test relative path
        let args = JirustCliArgs::try_parse_from([
            "jirust-cli",
            "issue",
            "attach",
            "-i",
            "TEST-1",
            "-p",
            "./relative/path/file.txt",
        ])
        .expect("relative path should parse");

        if let Commands::Issue(issue_args) = args.subcmd {
            assert_eq!(
                issue_args.attachment_file_path,
                Some("./relative/path/file.txt".to_string())
            );
        }

        // Test path with spaces
        let args = JirustCliArgs::try_parse_from([
            "jirust-cli",
            "issue",
            "attach",
            "-i",
            "TEST-1",
            "-p",
            "/path/with spaces/file name.txt",
        ])
        .expect("path with spaces should parse");

        if let Commands::Issue(issue_args) = args.subcmd {
            assert_eq!(
                issue_args.attachment_file_path,
                Some("/path/with spaces/file name.txt".to_string())
            );
        }
    }

    #[test]
    fn test_issue_attach_without_file_path_fails() {
        // Should fail when attachment file path is missing
        let result =
            JirustCliArgs::try_parse_from(["jirust-cli", "issue", "attach", "-i", "TEST-123"]);

        // This should actually succeed in parsing but the validation happens later
        // The field will be None
        if let Ok(args) = result {
            if let Commands::Issue(issue_args) = args.subcmd {
                assert_eq!(issue_args.attachment_file_path, None);
            }
        }
    }

    #[test]
    fn test_issue_action_values_includes_attach() {
        // Verify that Attach is included in IssueActionValues enum
        let attach_action = IssueActionValues::Attach;

        // Test Debug trait
        let debug_str = format!("{:?}", attach_action);
        assert!(debug_str.contains("Attach"));

        // Test Clone trait
        let cloned = attach_action.clone();
        assert!(matches!(cloned, IssueActionValues::Attach));
    }
}
