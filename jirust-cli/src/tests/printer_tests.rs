#[cfg(test)]
mod tests {
    use crate::utils::{
        PrintableData,
        json_printer::print_json,
        table_printer::{print_table_basic, print_table_full, print_table_single},
    };
    use jira_v3_openapi::models::{
        CreatedIssue, FieldCreateMetadata, IssueBean, IssueTransition,
        IssueTypeIssueCreateMetadata, Project, ProjectCategory, Version, VersionRelatedWork,
    };
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn test_json_printer_generic_data() {
        let data = vec![
            json!({"id": "1", "name": "test item", "active": true}),
            json!({"id": "2", "name": "another item", "active": false}),
            json!({"complex": {"nested": {"value": 42}}}),
        ];

        let printable = PrintableData::Generic { data };

        // Test that print_json doesn't panic
        print_json(printable);

        // In a real test environment, you might want to capture stdout
        // For now, we just verify the function executes without error
        assert!(true);
    }

    #[test]
    fn test_json_printer_issue_transitions() {
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

        let printable = PrintableData::IssueTransitions { transitions };
        print_json(printable);
        assert!(true);
    }

    #[test]
    fn test_json_printer_issue_created() {
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

        let printable = PrintableData::IssueCreated { issues };
        print_json(printable);
        assert!(true);
    }

    #[test]
    fn test_json_printer_issue_data() {
        let mut fields = HashMap::new();
        fields.insert("summary".to_string(), json!("Test issue summary"));
        fields.insert("description".to_string(), json!("Test issue description"));
        fields.insert("priority".to_string(), json!({"name": "High", "id": "2"}));

        let issues = vec![IssueBean {
            expand: None,
            id: Some("10001".to_string()),
            param_self: Some("https://test.atlassian.net/rest/api/3/issue/10001".to_string()),
            key: Some("TEST-1".to_string()),
            rendered_fields: None,
            properties: None,
            names: None,
            schema: None,
            transitions: Some(vec![IssueTransition {
                id: Some("11".to_string()),
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
            }]),
            operations: None,
            editmeta: None,
            changelog: None,
            versioned_representations: None,
            fields_to_include: None,
            fields: Some(fields),
        }];

        let printable = PrintableData::IssueData { issues };
        print_json(printable);
        assert!(true);
    }

    #[test]
    fn test_json_printer_transitioned_issue() {
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

        let printable = PrintableData::TransitionedIssue { issues };
        print_json(printable);
        assert!(true);
    }

    #[test]
    fn test_table_printer_issue_transitions() {
        let transitions = vec![IssueTransition {
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
        }];

        let printable = PrintableData::IssueTransitions {
            transitions: transitions.clone(),
        };
        print_table_full(printable);

        let printable2 = PrintableData::IssueTransitions {
            transitions: transitions.clone(),
        };
        print_table_basic(printable2);

        let printable3 = PrintableData::IssueTransitions { transitions };
        print_table_single(printable3);

        assert!(true);
    }

    #[test]
    fn test_json_printer_with_empty_data() {
        // Test with empty data
        let empty_data: Vec<serde_json::Value> = vec![];
        let printable = PrintableData::Generic { data: empty_data };
        print_json(printable);

        // Test with empty issue list
        let empty_issues: Vec<CreatedIssue> = vec![];
        let printable2 = PrintableData::IssueCreated {
            issues: empty_issues,
        };
        print_json(printable2);

        assert!(true);
    }

    #[test]
    fn test_json_printer_with_null_values() {
        let data_with_nulls = vec![
            json!({"id": null, "name": "test", "value": null}),
            json!({"id": "1", "name": null, "value": 42}),
        ];

        let printable = PrintableData::Generic {
            data: data_with_nulls,
        };
        print_json(printable);
        assert!(true);
    }

    #[test]
    fn test_json_printer_with_unicode() {
        let unicode_data = vec![
            json!({"name": "测试项目", "description": "这是一个测试"}),
            json!({"emoji": "🚀🎉⭐", "special": "!@#$%^&*()"}),
        ];

        let printable = PrintableData::Generic { data: unicode_data };
        print_json(printable);
        assert!(true);
    }

    #[test]
    fn test_table_printer_project_and_version_variants() {
        let mut project_category = ProjectCategory::default();
        project_category.name = Some("Business".to_string());

        let mut first_project = Project::default();
        first_project.id = Some("10000".to_string());
        first_project.key = Some("PRJ".to_string());
        first_project.name = Some("Primary".to_string());
        first_project.description = Some("Main project".to_string());
        first_project.project_category = Some(Box::new(project_category.clone()));

        let mut second_project = Project::default();
        second_project.id = Some("20000".to_string());
        second_project.key = Some("LIB".to_string());
        second_project.name = Some("Library".to_string());
        second_project.description = None;
        second_project.project_category = None;

        let projects = vec![first_project.clone(), second_project.clone()];

        print_table_full(PrintableData::Project {
            projects: projects.clone(),
        });
        print_table_basic(PrintableData::Project {
            projects: projects.clone(),
        });
        print_table_single(PrintableData::Project { projects });

        let mut first_version = Version::default();
        first_version.project_id = Some(10);
        first_version.id = Some("1".to_string());
        first_version.name = Some("1.0.0".to_string());
        first_version.description = Some("Initial release".to_string());
        first_version.start_date = Some("2024-01-01".to_string());
        first_version.release_date = Some("2024-01-15".to_string());
        first_version.archived = Some(false);
        first_version.released = Some(true);

        let mut second_version = Version::default();
        second_version.project_id = Some(20);
        second_version.id = Some("2".to_string());
        second_version.name = Some("2.0.0".to_string());
        second_version.description = None;
        second_version.start_date = None;
        second_version.release_date = None;
        second_version.archived = Some(true);
        second_version.released = Some(false);

        let versions = vec![first_version.clone(), second_version.clone()];

        print_table_full(PrintableData::Version {
            versions: versions.clone(),
        });
        print_table_basic(PrintableData::Version {
            versions: versions.clone(),
        });
        print_table_single(PrintableData::Version { versions });

        let mut related_work_primary = VersionRelatedWork::new("ReleaseNotes".to_string());
        related_work_primary.issue_id = Some(1010);
        related_work_primary.related_work_id = Some("REL-1".to_string());
        related_work_primary.title = Some("Release notes".to_string());
        related_work_primary.url = Some("https://example.com/release-notes".to_string());

        let mut related_work_secondary = VersionRelatedWork::new("Blog".to_string());
        related_work_secondary.issue_id = None;
        related_work_secondary.related_work_id = Some("REL-2".to_string());
        related_work_secondary.title = Some("Launch blog".to_string());
        related_work_secondary.url = Some("https://example.com/blog".to_string());

        let related_items = vec![related_work_primary.clone(), related_work_secondary.clone()];

        print_table_full(PrintableData::VersionRelatedWork {
            version_related_work_items: related_items.clone(),
        });
        print_table_basic(PrintableData::VersionRelatedWork {
            version_related_work_items: related_items.clone(),
        });
        print_table_single(PrintableData::VersionRelatedWork {
            version_related_work_items: related_items,
        });

        assert!(true);
    }

    #[test]
    fn test_table_printer_issue_variants() {
        let mut fields = HashMap::new();
        fields.insert("summary".to_string(), json!("Issue summary"));
        fields.insert("labels".to_string(), json!(["one", "two"]));

        let transitions = vec![
            IssueTransition {
                id: Some("1".to_string()),
                name: Some("Start".to_string()),
                is_available: Some(true),
                is_initial: Some(true),
                fields: Some(HashMap::from([(
                    "resolution".to_string(),
                    Default::default(),
                )])),
                ..Default::default()
            },
            IssueTransition {
                id: Some("2".to_string()),
                name: Some("Finish".to_string()),
                is_available: Some(false),
                is_initial: Some(false),
                fields: None,
                ..Default::default()
            },
        ];

        let issue_first = IssueBean {
            id: Some("10001".to_string()),
            key: Some("TEST-1".to_string()),
            fields: Some(fields.clone()),
            transitions: Some(transitions.clone()),
            ..Default::default()
        };

        let issue_second = IssueBean {
            id: Some("10002".to_string()),
            key: Some("TEST-2".to_string()),
            fields: None,
            transitions: None,
            ..Default::default()
        };

        let issues = vec![issue_first.clone(), issue_second.clone()];

        print_table_full(PrintableData::IssueData {
            issues: issues.clone(),
        });
        print_table_basic(PrintableData::IssueData {
            issues: issues.clone(),
        });
        print_table_single(PrintableData::IssueData { issues });

        let created_issues = vec![
            CreatedIssue {
                id: Some("20001".to_string()),
                key: Some("CR-1".to_string()),
                param_self: Some("https://example.com/browse/CR-1".to_string()),
                ..Default::default()
            },
            CreatedIssue {
                id: None,
                key: None,
                param_self: None,
                ..Default::default()
            },
        ];

        print_table_full(PrintableData::IssueCreated {
            issues: created_issues.clone(),
        });
        print_table_basic(PrintableData::IssueCreated {
            issues: created_issues.clone(),
        });
        print_table_single(PrintableData::IssueCreated {
            issues: created_issues,
        });

        let issue_types = vec![
            IssueTypeIssueCreateMetadata {
                id: Some("10".to_string()),
                name: Some("Bug".to_string()),
                description: Some("Something is broken".to_string()),
                hierarchy_level: Some(1),
                subtask: Some(false),
                ..Default::default()
            },
            IssueTypeIssueCreateMetadata {
                id: Some("20".to_string()),
                name: Some("Sub-task".to_string()),
                description: None,
                hierarchy_level: None,
                subtask: Some(true),
                ..Default::default()
            },
        ];

        print_table_full(PrintableData::IssueType {
            issue_types: issue_types.clone(),
        });
        print_table_basic(PrintableData::IssueType {
            issue_types: issue_types.clone(),
        });
        print_table_single(PrintableData::IssueType { issue_types });

        let fields_metadata = vec![
            FieldCreateMetadata {
                field_id: "summary".to_string(),
                key: "summary".to_string(),
                name: "Summary".to_string(),
                required: true,
                operations: vec!["set".to_string()],
                schema: Default::default(),
                ..Default::default()
            },
            FieldCreateMetadata {
                field_id: "description".to_string(),
                key: "description".to_string(),
                name: "Description".to_string(),
                required: false,
                operations: vec!["set".to_string()],
                schema: Default::default(),
                ..Default::default()
            },
        ];

        print_table_full(PrintableData::IssueTypeField {
            issue_type_fields: fields_metadata.clone(),
        });
        print_table_basic(PrintableData::IssueTypeField {
            issue_type_fields: fields_metadata.clone(),
        });
        print_table_single(PrintableData::IssueTypeField {
            issue_type_fields: fields_metadata,
        });

        let transitioned = vec![
            (
                "TRANS-1".to_string(),
                "OK".to_string(),
                "FAILED".to_string(),
                "NO fixVersion set".to_string(),
            ),
            (
                "TRANS-2".to_string(),
                "FAILED".to_string(),
                "OK".to_string(),
                "1.0.0".to_string(),
            ),
        ];

        print_table_full(PrintableData::TransitionedIssue {
            issues: transitioned.clone(),
        });
        print_table_basic(PrintableData::TransitionedIssue {
            issues: transitioned.clone(),
        });
        print_table_single(PrintableData::TransitionedIssue {
            issues: transitioned,
        });

        assert!(true);
    }
}
