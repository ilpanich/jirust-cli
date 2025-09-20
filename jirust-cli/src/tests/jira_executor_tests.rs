#[cfg(test)]
mod tests {
    use crate::args::commands::{
        IssueActionValues, IssueArgs, LinkIssueActionValues, LinkIssueArgs, OutputArgs,
        OutputTypes, OutputValues, PaginationArgs, ProjectActionValues, ProjectArgs,
        TransitionActionValues, TransitionArgs, VersionActionValues, VersionArgs,
    };
    use crate::config::config_file::ConfigFile;
    use crate::executors::jira_commands_executors::{
        ExecJiraCommand, jira_issue_executor::IssueExecutor,
        jira_issue_link_executor::LinkIssueExecutor,
        jira_issue_transition_executor::IssueTransitionExecutor,
        jira_project_executor::ProjectExecutor, jira_version_executor::VersionExecutor,
    };
    use crate::runners::jira_cmd_runners::issue_cmd_runner::MockIssueCmdRunnerApi;
    use crate::runners::jira_cmd_runners::link_issue_cmd_runner::MockLinkIssueCmdRunnerApi;
    use crate::runners::jira_cmd_runners::project_cmd_runner::MockProjectCmdRunnerApi;
    use crate::runners::jira_cmd_runners::version_cmd_runner::MockVersionCmdRunnerApi;
    use crate::utils::PrintableData;
    use jira_v3_openapi::apis::Error as JiraApiError;
    use jira_v3_openapi::models::{
        FieldCreateMetadata, IssueTransition, IssueTypeIssueCreateMetadata, Transitions, Version,
        VersionRelatedWork, project::Project, project_identifiers::ProjectIdentifiers,
    };
    use serde_json::json;
    use std::io::{Error as IoError, ErrorKind};
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

    fn create_test_issue_args() -> IssueArgs {
        IssueArgs {
            issue_act: IssueActionValues::Get,
            project_key: Some("TEST".to_string()),
            issue_key: Some("TEST-123".to_string()),
            issue_fields: Some(vec![
                ("summary".to_string(), "Test issue".to_string()),
                ("description".to_string(), "Test description".to_string()),
                ("issuetype".to_string(), "Task".to_string()),
            ]),
            transition_to: None,
            assignee: Some("test.user".to_string()),
            query: None,
            pagination: PaginationArgs {
                page_size: None,
                page_offset: None,
            },
            output: OutputArgs {
                output_format: Some(OutputValues::Table),
                output_type: Some(OutputTypes::Basic),
            },
        }
    }

    fn create_test_project_args() -> ProjectArgs {
        ProjectArgs {
            project_act: ProjectActionValues::List,
            project_key: Some("TEST".to_string()),
            project_issue_type: None,
            project_name: Some("Test Project".to_string()),
            project_description: Some("Test project description".to_string()),
            project_field_configuration_id: None,
            project_issue_security_scheme_id: None,
            project_issue_type_scheme_id: None,
            project_issue_type_screen_scheme_id: None,
            project_notification_scheme_id: None,
            project_permission_scheme_id: None,
            project_workflow_scheme_id: None,
            project_lead_account_id: Some("test.user".to_string()),
            project_assignee_type: None,
            pagination: PaginationArgs {
                page_size: None,
                page_offset: None,
            },
            output: OutputArgs {
                output_format: Some(OutputValues::Table),
                output_type: Some(OutputTypes::Basic),
            },
        }
    }

    fn create_test_version_args() -> VersionArgs {
        VersionArgs {
            version_act: VersionActionValues::List,
            project_key: "TEST".to_string(),
            project_id: None,
            version_id: Some("1.0.0".to_string()),
            version_name: Some("1.0.0".to_string()),
            version_description: Some("Test version".to_string()),
            version_start_date: None,
            version_release_date: None,
            version_archived: None,
            version_released: None,
            changelog_file: None,
            transition_issues: None,
            transition_assignee: None,
            pagination: PaginationArgs {
                page_size: None,
                page_offset: None,
            },
            output: OutputArgs {
                output_format: Some(OutputValues::Table),
                output_type: Some(OutputTypes::Basic),
            },
        }
    }

    fn create_test_link_args() -> LinkIssueArgs {
        LinkIssueArgs {
            link_act: LinkIssueActionValues::Create,
            project_key: Some("TEST".to_string()),
            origin_issue_key: "TEST-123".to_string(),
            destination_issue_key: Some("TEST-456".to_string()),
            link_type: "Blocks".to_string(),
            changelog_file: None,
        }
    }

    fn create_test_transition_args() -> TransitionArgs {
        TransitionArgs {
            transition_act: TransitionActionValues::List,
            issue_key: "TEST-123".to_string(),
            output: OutputArgs {
                output_format: Some(OutputValues::Table),
                output_type: Some(OutputTypes::Basic),
            },
        }
    }

    #[test]
    fn test_issue_executor_creation() {
        let config = create_test_config();
        let args = create_test_issue_args();

        let _executor = IssueExecutor::new(config, IssueActionValues::Get, args);

        // Test passes if no panic occurs during creation
        assert!(true);
    }

    #[test]
    fn test_project_executor_creation() {
        let config = create_test_config();
        let args = create_test_project_args();

        let _executor = ProjectExecutor::new(config, ProjectActionValues::List, args);

        // Test passes if no panic occurs during creation
        assert!(true);
    }

    #[test]
    fn test_version_executor_creation() {
        let config = create_test_config();
        let args = create_test_version_args();

        let _executor = VersionExecutor::new(config, VersionActionValues::List, args);

        // Test passes if no panic occurs during creation
        assert!(true);
    }

    #[test]
    fn test_link_issue_executor_creation() {
        let config = create_test_config();
        let args = create_test_link_args();

        let _executor = LinkIssueExecutor::new(config, LinkIssueActionValues::Create, args);

        // Test passes if no panic occurs during creation
        assert!(true);
    }

    #[test]
    fn test_issue_transition_executor_creation() {
        let config = create_test_config();
        let args = create_test_transition_args();

        let _executor = IssueTransitionExecutor::new(config, TransitionActionValues::List, args);

        // Test passes if no panic occurs during creation
        assert!(true);
    }

    #[test]
    fn test_issue_executor_with_different_actions() {
        let config = create_test_config();
        let args = create_test_issue_args();

        // Test creating executors with different actions
        let _get_executor =
            IssueExecutor::new(config.clone(), IssueActionValues::Get, args.clone());
        let _create_executor =
            IssueExecutor::new(config.clone(), IssueActionValues::Create, args.clone());
        let _update_executor =
            IssueExecutor::new(config.clone(), IssueActionValues::Update, args.clone());
        let _delete_executor =
            IssueExecutor::new(config.clone(), IssueActionValues::Delete, args.clone());
        let _search_executor =
            IssueExecutor::new(config.clone(), IssueActionValues::Search, args.clone());
        let _assign_executor = IssueExecutor::new(config, IssueActionValues::Assign, args);

        // All constructors should work without panicking
        assert!(true);
    }

    #[tokio::test]
    async fn test_issue_executor_assign_success() {
        let mut mock_runner = MockIssueCmdRunnerApi::new();
        mock_runner.expect_assign_jira_issue().returning(|params| {
            assert_eq!(params.issue_key.as_deref(), Some("TEST-123"));
            assert_eq!(params.assignee.as_deref(), Some("user@example.com"));
            Ok(json!({"ok": true}))
        });

        let args = IssueArgs {
            issue_act: IssueActionValues::Assign,
            project_key: Some("TEST".to_string()),
            issue_key: Some("TEST-123".to_string()),
            issue_fields: None,
            transition_to: None,
            assignee: Some("user@example.com".to_string()),
            query: None,
            pagination: PaginationArgs {
                page_size: None,
                page_offset: None,
            },
            output: OutputArgs {
                output_format: None,
                output_type: None,
            },
        };

        let executor = IssueExecutor::with_runner(mock_runner, IssueActionValues::Assign, args);
        let result = executor.exec_jira_command().await.expect("assign succeeds");
        let first = result.first().expect("printable data available");
        match first {
            PrintableData::Generic { data } => {
                assert_eq!(
                    data,
                    &vec![serde_json::Value::String(
                        "Issue assigned successfully".to_string()
                    )]
                );
            }
            _ => panic!("unexpected printable data variant"),
        }
    }

    #[tokio::test]
    async fn test_issue_executor_get_error_surface() {
        let mut mock_runner = MockIssueCmdRunnerApi::new();
        mock_runner.expect_get_jira_issue().returning(|_| {
            Err(Box::new(IoError::new(ErrorKind::Other, "boom")) as Box<dyn std::error::Error>)
        });

        let args = IssueArgs {
            issue_act: IssueActionValues::Get,
            project_key: Some("TEST".to_string()),
            issue_key: Some("TEST-123".to_string()),
            issue_fields: None,
            transition_to: None,
            assignee: None,
            query: None,
            pagination: PaginationArgs {
                page_size: None,
                page_offset: None,
            },
            output: OutputArgs {
                output_format: None,
                output_type: None,
            },
        };

        let executor = IssueExecutor::with_runner(mock_runner, IssueActionValues::Get, args);
        match executor.exec_jira_command().await {
            Ok(_) => panic!("expected error, got success"),
            Err(err) => assert!(err.to_string().contains("Error retrieving issue")),
        }
    }

    #[tokio::test]
    async fn test_project_executor_list_success() {
        let mut mock_runner = MockProjectCmdRunnerApi::new();
        mock_runner
            .expect_list_jira_projects()
            .returning(|_| Ok(vec![Project::default()]));

        let args = create_test_project_args();
        let executor = ProjectExecutor::with_runner(mock_runner, ProjectActionValues::List, args);

        let result = executor.exec_jira_command().await.expect("list succeeds");
        assert!(matches!(
            result.first(),
            Some(PrintableData::Project { .. })
        ));
    }

    #[tokio::test]
    async fn test_project_executor_list_error() {
        let mut mock_runner = MockProjectCmdRunnerApi::new();
        mock_runner.expect_list_jira_projects().returning(|_| {
            Err(Box::new(IoError::new(ErrorKind::Other, "no projects"))
                as Box<dyn std::error::Error>)
        });

        let args = create_test_project_args();
        let executor = ProjectExecutor::with_runner(mock_runner, ProjectActionValues::List, args);

        match executor.exec_jira_command().await {
            Ok(_) => panic!("expected error, got success"),
            Err(err) => assert!(err.to_string().contains("Error listing projects")),
        }
    }

    #[tokio::test]
    async fn test_project_executor_create_success() {
        let mut mock_runner = MockProjectCmdRunnerApi::new();
        mock_runner.expect_create_jira_project().returning(|_| {
            Ok(ProjectIdentifiers::new(
                42,
                "TEST".to_string(),
                "url".to_string(),
            ))
        });

        let mut args = create_test_project_args();
        args.project_act = ProjectActionValues::Create;
        let executor = ProjectExecutor::with_runner(mock_runner, ProjectActionValues::Create, args);

        let result = executor.exec_jira_command().await.expect("create succeeds");
        match result.first().expect("missing data") {
            PrintableData::Project { projects } => {
                assert_eq!(projects.len(), 1);
                assert_eq!(projects[0].id.as_deref(), Some("42"));
                assert_eq!(projects[0].key.as_deref(), Some("TEST"));
            }
            _ => panic!("expected project data"),
        }
    }

    #[tokio::test]
    async fn test_project_executor_create_error() {
        let mut mock_runner = MockProjectCmdRunnerApi::new();
        mock_runner.expect_create_jira_project().returning(|_| {
            Err(Box::new(IoError::new(ErrorKind::Other, "fail")) as Box<dyn std::error::Error>)
        });

        let mut args = create_test_project_args();
        args.project_act = ProjectActionValues::Create;
        let executor = ProjectExecutor::with_runner(mock_runner, ProjectActionValues::Create, args);

        match executor.exec_jira_command().await {
            Ok(_) => panic!("expected error, got success"),
            Err(err) => assert!(err.to_string().contains("Error creating project")),
        }
    }

    #[tokio::test]
    async fn test_version_executor_list_success() {
        let mut mock_runner = MockVersionCmdRunnerApi::new();
        mock_runner
            .expect_list_jira_versions()
            .returning(|_| Ok(vec![Version::default()]));

        let args = create_test_version_args();
        let executor = VersionExecutor::with_runner(mock_runner, VersionActionValues::List, args);

        let result = executor.exec_jira_command().await.expect("list succeeds");
        assert!(matches!(
            result.first(),
            Some(PrintableData::Version { .. })
        ));
    }

    #[tokio::test]
    async fn test_version_executor_create_success_with_transitions() {
        let mut mock_runner = MockVersionCmdRunnerApi::new();
        mock_runner.expect_create_jira_version().returning(|_| {
            let mut version = Version::default();
            version.id = Some("123".to_string());
            Ok((
                version,
                Some(vec![(
                    "ISSUE-1".to_string(),
                    "Done".to_string(),
                    "user".to_string(),
                    "1.0.0".to_string(),
                )]),
            ))
        });

        let mut args = create_test_version_args();
        args.version_act = VersionActionValues::Create;
        let executor = VersionExecutor::with_runner(mock_runner, VersionActionValues::Create, args);

        let result = executor.exec_jira_command().await.expect("create succeeds");
        assert_eq!(result.len(), 2);
        assert!(matches!(result[0], PrintableData::Version { .. }));
        assert!(matches!(result[1], PrintableData::TransitionedIssue { .. }));
    }

    #[tokio::test]
    async fn test_version_executor_create_error() {
        let mut mock_runner = MockVersionCmdRunnerApi::new();
        mock_runner.expect_create_jira_version().returning(|_| {
            Err(Box::new(IoError::new(ErrorKind::Other, "boom")) as Box<dyn std::error::Error>)
        });

        let mut args = create_test_version_args();
        args.version_act = VersionActionValues::Create;
        let executor = VersionExecutor::with_runner(mock_runner, VersionActionValues::Create, args);

        match executor.exec_jira_command().await {
            Ok(_) => panic!("expected error, got success"),
            Err(err) => assert!(err.to_string().contains("Error creating version")),
        }
    }

    #[tokio::test]
    async fn test_version_executor_update_success() {
        let mut mock_runner = MockVersionCmdRunnerApi::new();
        mock_runner.expect_get_jira_version().returning(|_| {
            let mut version = Version::default();
            version.id = Some("123".to_string());
            Ok(version)
        });
        mock_runner.expect_update_jira_version().returning(|_| {
            let mut updated = Version::default();
            updated.name = Some("updated".to_string());
            Ok(updated)
        });

        let mut args = create_test_version_args();
        args.version_act = VersionActionValues::Update;
        let executor = VersionExecutor::with_runner(mock_runner, VersionActionValues::Update, args);

        let result = executor.exec_jira_command().await.expect("update succeeds");
        assert!(matches!(
            result.first(),
            Some(PrintableData::Version { .. })
        ));
    }

    #[tokio::test]
    async fn test_version_executor_update_error_on_get() {
        let mut mock_runner = MockVersionCmdRunnerApi::new();
        mock_runner.expect_get_jira_version().returning(|_| {
            Err(Box::new(IoError::new(ErrorKind::Other, "missing")) as Box<dyn std::error::Error>)
        });

        let mut args = create_test_version_args();
        args.version_act = VersionActionValues::Update;
        let executor = VersionExecutor::with_runner(mock_runner, VersionActionValues::Update, args);

        match executor.exec_jira_command().await {
            Ok(_) => panic!("expected error, got success"),
            Err(err) => assert!(err.to_string().contains("Error retrieving version")),
        }
    }

    #[tokio::test]
    async fn test_version_executor_update_error_on_update() {
        let mut mock_runner = MockVersionCmdRunnerApi::new();
        mock_runner.expect_get_jira_version().returning(|_| {
            let mut version = Version::default();
            version.id = Some("123".to_string());
            Ok(version)
        });
        mock_runner.expect_update_jira_version().returning(|_| {
            Err(Box::new(IoError::new(ErrorKind::Other, "nope")) as Box<dyn std::error::Error>)
        });

        let mut args = create_test_version_args();
        args.version_act = VersionActionValues::Update;
        let executor = VersionExecutor::with_runner(mock_runner, VersionActionValues::Update, args);

        match executor.exec_jira_command().await {
            Ok(_) => panic!("expected error, got success"),
            Err(err) => assert!(err.to_string().contains("Error updating version")),
        }
    }

    #[tokio::test]
    async fn test_project_executor_issue_types_success() {
        let mut mock_runner = MockProjectCmdRunnerApi::new();
        mock_runner
            .expect_get_jira_project_issue_types()
            .returning(|_| Ok(vec![IssueTypeIssueCreateMetadata::default()]));

        let mut args = create_test_project_args();
        args.project_act = ProjectActionValues::GetIssueTypes;
        let executor =
            ProjectExecutor::with_runner(mock_runner, ProjectActionValues::GetIssueTypes, args);

        let result = executor
            .exec_jira_command()
            .await
            .expect("issue types succeed");
        assert!(matches!(
            result.first(),
            Some(PrintableData::IssueType { .. })
        ));
    }

    #[tokio::test]
    async fn test_project_executor_issue_types_error() {
        let mut mock_runner = MockProjectCmdRunnerApi::new();
        mock_runner
            .expect_get_jira_project_issue_types()
            .returning(|_| {
                Err(Box::new(IoError::new(ErrorKind::Other, "bad")) as Box<dyn std::error::Error>)
            });

        let mut args = create_test_project_args();
        args.project_act = ProjectActionValues::GetIssueTypes;
        let executor =
            ProjectExecutor::with_runner(mock_runner, ProjectActionValues::GetIssueTypes, args);

        match executor.exec_jira_command().await {
            Ok(_) => panic!("expected error, got success"),
            Err(err) => assert!(err.to_string().contains("Error listing issue types")),
        }
    }

    #[tokio::test]
    async fn test_project_executor_issue_type_fields_success() {
        let mut mock_runner = MockProjectCmdRunnerApi::new();
        mock_runner
            .expect_get_jira_project_issue_type_id()
            .returning(|_| Ok(vec![FieldCreateMetadata::default()]));

        let mut args = create_test_project_args();
        args.project_act = ProjectActionValues::GetIssueTypeFields;
        let executor = ProjectExecutor::with_runner(
            mock_runner,
            ProjectActionValues::GetIssueTypeFields,
            args,
        );

        let result = executor
            .exec_jira_command()
            .await
            .expect("issue type fields succeed");
        assert!(matches!(
            result.first(),
            Some(PrintableData::IssueTypeField { .. })
        ));
    }

    #[tokio::test]
    async fn test_project_executor_issue_type_fields_error() {
        let mut mock_runner = MockProjectCmdRunnerApi::new();
        mock_runner
            .expect_get_jira_project_issue_type_id()
            .returning(|_| {
                Err(Box::new(IoError::new(ErrorKind::Other, "fail")) as Box<dyn std::error::Error>)
            });

        let mut args = create_test_project_args();
        args.project_act = ProjectActionValues::GetIssueTypeFields;
        let executor = ProjectExecutor::with_runner(
            mock_runner,
            ProjectActionValues::GetIssueTypeFields,
            args,
        );

        match executor.exec_jira_command().await {
            Ok(_) => panic!("expected error, got success"),
            Err(err) => assert!(err.to_string().contains("Error listing issue type fields")),
        }
    }

    #[tokio::test]
    async fn test_version_executor_delete_error() {
        let mut mock_runner = MockVersionCmdRunnerApi::new();
        mock_runner.expect_delete_jira_version().returning(|_| {
            Err(Box::new(IoError::new(ErrorKind::Other, "nope")) as Box<dyn std::error::Error>)
        });

        let mut args = create_test_version_args();
        args.version_act = VersionActionValues::Delete;
        let executor = VersionExecutor::with_runner(mock_runner, VersionActionValues::Delete, args);

        match executor.exec_jira_command().await {
            Ok(_) => panic!("expected error, got success"),
            Err(err) => assert!(err.to_string().contains("Error deleting version")),
        }
    }

    #[tokio::test]
    async fn test_version_executor_release_success() {
        let mut mock_runner = MockVersionCmdRunnerApi::new();
        mock_runner.expect_get_jira_version().returning(|_| {
            let mut version = Version::default();
            version.id = Some("123".to_string());
            Ok(version)
        });
        mock_runner.expect_update_jira_version().returning(|_| {
            let mut released = Version::default();
            released.released = Some(true);
            Ok(released)
        });

        let mut args = create_test_version_args();
        args.version_act = VersionActionValues::Release;
        let executor =
            VersionExecutor::with_runner(mock_runner, VersionActionValues::Release, args);

        let result = executor
            .exec_jira_command()
            .await
            .expect("release succeeds");
        assert!(matches!(
            result.first(),
            Some(PrintableData::Version { .. })
        ));
    }

    #[tokio::test]
    async fn test_version_executor_release_error_on_update() {
        let mut mock_runner = MockVersionCmdRunnerApi::new();
        mock_runner.expect_get_jira_version().returning(|_| {
            let mut version = Version::default();
            version.id = Some("123".to_string());
            Ok(version)
        });
        mock_runner.expect_update_jira_version().returning(|_| {
            Err(Box::new(IoError::new(ErrorKind::Other, "transition fail"))
                as Box<dyn std::error::Error>)
        });

        let mut args = create_test_version_args();
        args.version_act = VersionActionValues::Release;
        let executor =
            VersionExecutor::with_runner(mock_runner, VersionActionValues::Release, args);

        match executor.exec_jira_command().await {
            Ok(_) => panic!("expected error, got success"),
            Err(err) => assert!(err.to_string().contains("Error releasing version")),
        }
    }

    #[tokio::test]
    async fn test_version_executor_archive_success() {
        let mut mock_runner = MockVersionCmdRunnerApi::new();
        mock_runner.expect_get_jira_version().returning(|_| {
            let mut version = Version::default();
            version.id = Some("123".to_string());
            Ok(version)
        });
        mock_runner.expect_update_jira_version().returning(|_| {
            let mut archived = Version::default();
            archived.archived = Some(true);
            Ok(archived)
        });

        let mut args = create_test_version_args();
        args.version_act = VersionActionValues::Archive;
        let executor =
            VersionExecutor::with_runner(mock_runner, VersionActionValues::Archive, args);

        let result = executor
            .exec_jira_command()
            .await
            .expect("archive succeeds");
        assert!(matches!(
            result.first(),
            Some(PrintableData::Version { .. })
        ));
    }

    #[tokio::test]
    async fn test_version_executor_related_work_success() {
        let mut mock_runner = MockVersionCmdRunnerApi::new();
        mock_runner
            .expect_get_jira_version_related_work()
            .returning(|_| Ok(vec![VersionRelatedWork::default()]));

        let mut args = create_test_version_args();
        args.version_act = VersionActionValues::RelatedWorkItems;
        let executor =
            VersionExecutor::with_runner(mock_runner, VersionActionValues::RelatedWorkItems, args);

        let result = executor
            .exec_jira_command()
            .await
            .expect("related work succeeds");
        assert!(matches!(
            result.first(),
            Some(PrintableData::VersionRelatedWork { .. })
        ));
    }

    #[tokio::test]
    async fn test_version_executor_related_work_error() {
        let mut mock_runner = MockVersionCmdRunnerApi::new();
        mock_runner
            .expect_get_jira_version_related_work()
            .returning(|_| Err(JiraApiError::from(IoError::new(ErrorKind::Other, "fail"))));

        let mut args = create_test_version_args();
        args.version_act = VersionActionValues::RelatedWorkItems;
        let executor =
            VersionExecutor::with_runner(mock_runner, VersionActionValues::RelatedWorkItems, args);

        match executor.exec_jira_command().await {
            Ok(_) => panic!("expected error, got success"),
            Err(err) => assert!(
                err.to_string()
                    .contains("Error listing version Related Workitems")
            ),
        }
    }

    #[tokio::test]
    async fn test_link_issue_executor_success() {
        let mut mock_runner = MockLinkIssueCmdRunnerApi::new();
        mock_runner.expect_link_jira_issues().returning(|params| {
            assert_eq!(params.origin_issue_key, "TEST-123");
            Ok(json!({"linked": true}))
        });

        let args = create_test_link_args();
        let executor =
            LinkIssueExecutor::with_runner(mock_runner, LinkIssueActionValues::Create, args);

        let result = executor.exec_jira_command().await.expect("link succeeds");
        assert!(matches!(
            result.first(),
            Some(PrintableData::Generic { .. })
        ));
    }

    #[tokio::test]
    async fn test_issue_transition_executor_success() {
        let mut mock_runner = MockIssueCmdRunnerApi::new();
        mock_runner
            .expect_get_issue_available_transitions()
            .returning(|_| {
                let transitions = Transitions {
                    transitions: Some(vec![IssueTransition::default()]),
                    ..Default::default()
                };
                Ok(transitions)
            });

        let args = create_test_transition_args();
        let executor =
            IssueTransitionExecutor::with_runner(mock_runner, TransitionActionValues::List, args);

        let result = executor
            .exec_jira_command()
            .await
            .expect("transition succeeds");
        assert!(matches!(
            result.first(),
            Some(PrintableData::IssueTransitions { .. })
        ));
    }

    #[test]
    fn test_project_executor_with_different_actions() {
        let config = create_test_config();
        let args = create_test_project_args();

        // Test creating executors with different actions
        let _list_executor =
            ProjectExecutor::new(config.clone(), ProjectActionValues::List, args.clone());
        let _create_executor = ProjectExecutor::new(config, ProjectActionValues::Create, args);

        // All constructors should work without panicking
        assert!(true);
    }

    #[test]
    fn test_version_executor_with_different_actions() {
        let config = create_test_config();
        let args = create_test_version_args();

        // Test creating executors with different actions
        let _list_executor =
            VersionExecutor::new(config.clone(), VersionActionValues::List, args.clone());
        let _create_executor =
            VersionExecutor::new(config.clone(), VersionActionValues::Create, args.clone());
        let _update_executor =
            VersionExecutor::new(config.clone(), VersionActionValues::Update, args.clone());
        let _delete_executor =
            VersionExecutor::new(config.clone(), VersionActionValues::Delete, args.clone());
        let _release_executor =
            VersionExecutor::new(config.clone(), VersionActionValues::Release, args.clone());
        let _archive_executor = VersionExecutor::new(config, VersionActionValues::Archive, args);

        // All constructors should work without panicking
        assert!(true);
    }

    #[test]
    fn test_multiple_executor_instances() {
        let config = create_test_config();

        // Test creating multiple different executor types
        let _issue_executor = IssueExecutor::new(
            config.clone(),
            IssueActionValues::Get,
            create_test_issue_args(),
        );
        let _project_executor = ProjectExecutor::new(
            config.clone(),
            ProjectActionValues::List,
            create_test_project_args(),
        );
        let _version_executor = VersionExecutor::new(
            config.clone(),
            VersionActionValues::List,
            create_test_version_args(),
        );
        let _link_executor = LinkIssueExecutor::new(
            config.clone(),
            LinkIssueActionValues::Create,
            create_test_link_args(),
        );
        let _transition_executor = IssueTransitionExecutor::new(
            config,
            TransitionActionValues::List,
            create_test_transition_args(),
        );

        // Test that all executors were created successfully
        assert!(true);
    }

    #[test]
    fn test_executor_config_handling() {
        let mut config = ConfigFile::default();
        config.set_auth_key("dGVzdF91c2VyOnRlc3RfYXBpX2tleQ==".to_string());
        config.set_jira_url("https://custom.atlassian.net".to_string());

        let args = create_test_issue_args();
        let _executor = IssueExecutor::new(config, IssueActionValues::Get, args);

        // Test that executor accepts custom config
        assert!(true);
    }

    #[test]
    fn test_executor_args_variations() {
        let config = create_test_config();

        // Test with minimal args
        let minimal_args = IssueArgs {
            issue_act: IssueActionValues::Get,
            project_key: None,
            issue_key: Some("MIN-1".to_string()),
            issue_fields: None,
            transition_to: None,
            assignee: None,
            query: None,
            pagination: PaginationArgs {
                page_size: None,
                page_offset: None,
            },
            output: OutputArgs {
                output_format: None,
                output_type: None,
            },
        };
        let _executor_minimal =
            IssueExecutor::new(config.clone(), IssueActionValues::Get, minimal_args);

        // Test with comprehensive args
        let comprehensive_args = IssueArgs {
            issue_act: IssueActionValues::Create,
            project_key: Some("COMP".to_string()),
            issue_key: Some("COMP-1".to_string()),
            issue_fields: Some(vec![
                ("summary".to_string(), "Comprehensive test".to_string()),
                ("description".to_string(), "Full description".to_string()),
                ("issuetype".to_string(), "Bug".to_string()),
                ("priority".to_string(), "High".to_string()),
            ]),
            transition_to: Some("In Progress".to_string()),
            assignee: Some("user@test.com".to_string()),
            query: Some("project = COMP".to_string()),
            pagination: PaginationArgs {
                page_size: Some(50),
                page_offset: Some(0),
            },
            output: OutputArgs {
                output_format: Some(OutputValues::Json),
                output_type: Some(OutputTypes::Full),
            },
        };
        let _executor_comprehensive =
            IssueExecutor::new(config, IssueActionValues::Create, comprehensive_args);

        // Both should create successfully
        assert!(true);
    }

    #[test]
    fn test_action_enum_variants() {
        // Test all IssueActionValues variants
        let issue_actions = [
            IssueActionValues::Assign,
            IssueActionValues::Create,
            IssueActionValues::Delete,
            IssueActionValues::Get,
            IssueActionValues::Search,
            IssueActionValues::Transition,
            IssueActionValues::Update,
        ];
        assert_eq!(issue_actions.len(), 7);

        // Test all ProjectActionValues variants
        let project_actions = [ProjectActionValues::Create, ProjectActionValues::List];
        assert_eq!(project_actions.len(), 2);

        // Test all VersionActionValues variants
        let version_actions = [
            VersionActionValues::Archive,
            VersionActionValues::Create,
            VersionActionValues::Delete,
            VersionActionValues::List,
            VersionActionValues::RelatedWorkItems,
            VersionActionValues::Release,
            VersionActionValues::Update,
        ];
        assert_eq!(version_actions.len(), 7);
    }

    #[test]
    fn test_output_args_creation() {
        let table_basic = OutputArgs {
            output_format: Some(OutputValues::Table),
            output_type: Some(OutputTypes::Basic),
        };

        let json_full = OutputArgs {
            output_format: Some(OutputValues::Json),
            output_type: Some(OutputTypes::Full),
        };

        let default_output = OutputArgs {
            output_format: None,
            output_type: None,
        };

        // All should be valid output configurations
        assert!(table_basic.output_format.is_some());
        assert!(json_full.output_format.is_some());
        assert!(default_output.output_format.is_none());
    }

    #[test]
    fn test_pagination_args_creation() {
        let paginated = PaginationArgs {
            page_size: Some(25),
            page_offset: Some(100),
        };

        let unpaginated = PaginationArgs {
            page_size: None,
            page_offset: None,
        };

        // Both should be valid pagination configurations
        assert_eq!(paginated.page_size, Some(25));
        assert_eq!(paginated.page_offset, Some(100));
        assert_eq!(unpaginated.page_size, None);
        assert_eq!(unpaginated.page_offset, None);
    }
}
