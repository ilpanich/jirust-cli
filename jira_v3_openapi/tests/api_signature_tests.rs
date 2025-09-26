#[cfg(feature = "issues_api")]
mod issues_api_signatures {
    use std::future::Future;

    use jira_v3_openapi::apis::{self, configuration, issues_api};
    use jira_v3_openapi::models;

    #[test]
    fn create_issue_signature_is_stable() {
        fn assert_future<Fut>(future: Fut)
        where
            Fut: Future<
                Output = std::result::Result<
                    models::CreatedIssue,
                    apis::Error<issues_api::CreateIssueError>,
                >,
            >,
        {
            let _ = future;
        }

        let cfg = configuration::Configuration::new();
        let details: models::IssueUpdateDetails = Default::default();
        assert_future(issues_api::create_issue(&cfg, details, None));
    }

    #[test]
    fn delete_issue_signature_is_stable() {
        fn assert_future<Fut>(future: Fut)
        where
            Fut:
                Future<Output = std::result::Result<(), apis::Error<issues_api::DeleteIssueError>>>,
        {
            let _ = future;
        }

        let cfg = configuration::Configuration::new();
        assert_future(issues_api::delete_issue(&cfg, "ISSUE-1", None));
    }

    #[test]
    fn get_issue_signature_is_stable() {
        fn assert_future<Fut>(future: Fut)
        where
            Fut: Future<
                Output = std::result::Result<
                    models::IssueBean,
                    apis::Error<issues_api::GetIssueError>,
                >,
            >,
        {
            let _ = future;
        }

        let cfg = configuration::Configuration::new();
        assert_future(issues_api::get_issue(
            &cfg, "ISSUE-1", None, None, None, None, None, None,
        ));
    }

    #[cfg(feature = "version_api")]
    #[test]
    fn assign_issue_signature_is_stable() {
        fn assert_future<Fut>(future: Fut)
        where
            Fut: Future<
                Output = std::result::Result<
                    serde_json::Value,
                    apis::Error<issues_api::AssignIssueError>,
                >,
            >,
        {
            let _ = future;
        }

        let cfg = configuration::Configuration::new();
        let user = models::User {
            account_id: Some(String::new()),
            ..Default::default()
        };
        assert_future(issues_api::assign_issue(&cfg, "ISSUE-1", user));
    }

    #[cfg(feature = "version_api")]
    #[test]
    fn do_transition_signature_is_stable() {
        fn assert_future<Fut>(future: Fut)
        where
            Fut: Future<
                Output = std::result::Result<
                    serde_json::Value,
                    apis::Error<issues_api::DoTransitionError>,
                >,
            >,
        {
            let _ = future;
        }

        let cfg = configuration::Configuration::new();
        let update: models::IssueUpdateDetails = Default::default();
        assert_future(issues_api::do_transition(&cfg, "ISSUE-1", update));
    }

    #[cfg(feature = "version_api")]
    #[test]
    fn edit_issue_signature_is_stable() {
        fn assert_future<Fut>(future: Fut)
        where
            Fut: Future<
                Output = std::result::Result<
                    serde_json::Value,
                    apis::Error<issues_api::EditIssueError>,
                >,
            >,
        {
            let _ = future;
        }

        let cfg = configuration::Configuration::new();
        let update: models::IssueUpdateDetails = Default::default();
        assert_future(issues_api::edit_issue(
            &cfg, "ISSUE-1", update, None, None, None, None, None,
        ));
    }

    #[cfg(feature = "version_api")]
    #[test]
    fn get_transitions_signature_is_stable() {
        fn assert_future<Fut>(future: Fut)
        where
            Fut: Future<
                Output = std::result::Result<
                    models::Transitions,
                    apis::Error<issues_api::GetTransitionsError>,
                >,
            >,
        {
            let _ = future;
        }

        let cfg = configuration::Configuration::new();
        assert_future(issues_api::get_transitions(
            &cfg, "ISSUE-1", None, None, None, None, None,
        ));
    }
}

#[cfg(feature = "issues_api")]
mod issue_search_api_signatures {
    use std::future::Future;

    use jira_v3_openapi::apis::{self, configuration, issue_search_api};
    use jira_v3_openapi::models;

    #[test]
    fn search_and_reconsile_issues_using_jql_post_signature_is_stable() {
        fn assert_future<Fut>(future: Fut)
        where
            Fut: Future<
                Output = std::result::Result<
                    models::SearchAndReconcileResults,
                    apis::Error<issue_search_api::SearchAndReconsileIssuesUsingJqlPostError>,
                >,
            >,
        {
            let _ = future;
        }

        let cfg = configuration::Configuration::new();
        let request: models::SearchAndReconcileRequestBean = Default::default();
        assert_future(issue_search_api::search_and_reconsile_issues_using_jql_post(&cfg, request));
    }
}

#[cfg(all(feature = "projects_api", feature = "issues_api"))]
mod issue_project_metadata_signatures {
    use std::future::Future;

    use jira_v3_openapi::apis::{self, configuration, issues_api};

    #[test]
    fn get_create_issue_meta_issue_type_id_signature_is_stable() {
        fn assert_future<Fut>(future: Fut)
        where
            Fut: Future<
                Output = std::result::Result<
                    jira_v3_openapi::models::PageOfCreateMetaIssueTypeWithField,
                    apis::Error<issues_api::GetCreateIssueMetaIssueTypeIdError>,
                >,
            >,
        {
            let _ = future;
        }

        let cfg = configuration::Configuration::new();
        assert_future(issues_api::get_create_issue_meta_issue_type_id(
            &cfg, "PROJ", "10000", None, None,
        ));
    }

    #[test]
    fn get_create_issue_meta_issue_types_signature_is_stable() {
        fn assert_future<Fut>(future: Fut)
        where
            Fut: Future<
                Output = std::result::Result<
                    jira_v3_openapi::models::PageOfCreateMetaIssueTypes,
                    apis::Error<issues_api::GetCreateIssueMetaIssueTypesError>,
                >,
            >,
        {
            let _ = future;
        }

        let cfg = configuration::Configuration::new();
        assert_future(issues_api::get_create_issue_meta_issue_types(
            &cfg, "PROJ", None, None,
        ));
    }
}

#[cfg(feature = "projects_api")]
mod projects_api_signatures {
    use std::future::Future;

    use jira_v3_openapi::apis::{self, configuration, projects_api};
    use jira_v3_openapi::models;

    #[test]
    fn create_project_signature_is_stable() {
        fn assert_future<Fut>(future: Fut)
        where
            Fut: Future<
                Output = std::result::Result<
                    models::ProjectIdentifiers,
                    apis::Error<projects_api::CreateProjectError>,
                >,
            >,
        {
            let _ = future;
        }

        let cfg = configuration::Configuration::new();
        let mut details =
            models::CreateProjectDetails::new(String::from("KEY"), String::from("Name"));
        details.project_type_key = Some(models::create_project_details::ProjectTypeKey::Software);
        assert_future(projects_api::create_project(&cfg, details));
    }

    #[test]
    fn search_projects_signature_is_stable() {
        fn assert_future<Fut>(future: Fut)
        where
            Fut: Future<
                Output = std::result::Result<
                    models::PageBeanProject,
                    apis::Error<projects_api::SearchProjectsError>,
                >,
            >,
        {
            let _ = future;
        }

        let cfg = configuration::Configuration::new();
        assert_future(projects_api::search_projects(
            &cfg, None, None, None, None, None, None, None, None, None, None, None, None, None,
        ));
    }
}

#[cfg(feature = "link_issues_api")]
mod issue_links_api_signatures {
    use std::future::Future;

    use jira_v3_openapi::apis::{self, configuration, issue_links_api};
    use jira_v3_openapi::models;

    #[test]
    fn link_issues_signature_is_stable() {
        fn assert_future<Fut>(future: Fut)
        where
            Fut: Future<
                Output = std::result::Result<
                    serde_json::Value,
                    apis::Error<issue_links_api::LinkIssuesError>,
                >,
            >,
        {
            let _ = future;
        }

        let cfg = configuration::Configuration::new();
        let request = models::LinkIssueRequestJsonBean::new(
            models::LinkedIssue::new(),
            models::LinkedIssue::new(),
            models::IssueLinkType::new(),
        );
        assert_future(issue_links_api::link_issues(&cfg, request));
    }
}

#[cfg(feature = "version_api")]
mod project_versions_api_signatures {
    use std::future::Future;

    use jira_v3_openapi::apis::{self, configuration, project_versions_api};
    use jira_v3_openapi::models;

    #[test]
    fn create_version_signature_is_stable() {
        fn assert_future<Fut>(future: Fut)
        where
            Fut: Future<
                Output = std::result::Result<
                    models::Version,
                    apis::Error<project_versions_api::CreateVersionError>,
                >,
            >,
        {
            let _ = future;
        }

        let cfg = configuration::Configuration::new();
        let version: models::Version = Default::default();
        assert_future(project_versions_api::create_version(&cfg, version));
    }

    #[test]
    fn get_project_versions_signature_is_stable() {
        fn assert_future<Fut>(future: Fut)
        where
            Fut: Future<
                Output = std::result::Result<
                    Vec<models::Version>,
                    apis::Error<project_versions_api::GetProjectVersionsError>,
                >,
            >,
        {
            let _ = future;
        }

        let cfg = configuration::Configuration::new();
        assert_future(project_versions_api::get_project_versions(
            &cfg, "PROJ", None,
        ));
    }

    #[test]
    fn get_project_versions_paginated_signature_is_stable() {
        fn assert_future<Fut>(future: Fut)
        where
            Fut: Future<
                Output = std::result::Result<
                    models::PageBeanVersion,
                    apis::Error<project_versions_api::GetProjectVersionsPaginatedError>,
                >,
            >,
        {
            let _ = future;
        }

        let cfg = configuration::Configuration::new();
        assert_future(project_versions_api::get_project_versions_paginated(
            &cfg, "PROJ", None, None, None, None, None, None,
        ));
    }

    #[test]
    fn get_version_signature_is_stable() {
        fn assert_future<Fut>(future: Fut)
        where
            Fut: Future<
                Output = std::result::Result<
                    models::Version,
                    apis::Error<project_versions_api::GetVersionError>,
                >,
            >,
        {
            let _ = future;
        }

        let cfg = configuration::Configuration::new();
        assert_future(project_versions_api::get_version(&cfg, "10000", None));
    }

    #[test]
    fn update_version_signature_is_stable() {
        fn assert_future<Fut>(future: Fut)
        where
            Fut: Future<
                Output = std::result::Result<
                    models::Version,
                    apis::Error<project_versions_api::UpdateVersionError>,
                >,
            >,
        {
            let _ = future;
        }

        let cfg = configuration::Configuration::new();
        let version: models::Version = Default::default();
        assert_future(project_versions_api::update_version(&cfg, "10000", version));
    }

    #[test]
    fn delete_and_replace_version_signature_is_stable() {
        fn assert_future<Fut>(future: Fut)
        where
            Fut: Future<
                Output = std::result::Result<
                    serde_json::Value,
                    apis::Error<project_versions_api::DeleteAndReplaceVersionError>,
                >,
            >,
        {
            let _ = future;
        }

        let cfg = configuration::Configuration::new();
        let request = models::DeleteAndReplaceVersionBean::new();
        assert_future(project_versions_api::delete_and_replace_version(
            &cfg, "10000", request,
        ));
    }

    #[test]
    fn get_related_work_signature_is_stable() {
        fn assert_future<Fut>(future: Fut)
        where
            Fut: Future<
                Output = std::result::Result<
                    Vec<models::VersionRelatedWork>,
                    apis::Error<project_versions_api::GetRelatedWorkError>,
                >,
            >,
        {
            let _ = future;
        }

        let cfg = configuration::Configuration::new();
        assert_future(project_versions_api::get_related_work(&cfg, "10000"));
    }
}
