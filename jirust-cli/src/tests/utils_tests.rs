#[cfg(test)]
mod tests {
    use crate::args::commands::{OutputArgs, OutputTypes, OutputValues};
    use crate::utils::PrintableData;
    use jira_v3_openapi::models::{
        CreatedIssue, FieldCreateMetadata, IssueBean, IssueTransition,
        IssueTypeIssueCreateMetadata, Project, Version,
    };

    #[test]
    fn test_printable_data_generic() {
        let mut test_data = Vec::new();
        test_data.push(serde_json::json!({"key": "value"}));
        test_data.push(serde_json::json!({"number": 42}));

        let printable = PrintableData::Generic { data: test_data };

        match printable {
            PrintableData::Generic { data } => {
                assert_eq!(data.len(), 2);
                assert!(data[0].get("key").is_some());
                assert!(data[1].get("number").is_some());
            }
            _ => panic!("Expected Generic variant"),
        }
    }

    #[test]
    fn test_printable_data_issue_created() {
        let created_issue = CreatedIssue {
            id: Some("10001".to_string()),
            key: Some("TEST-1".to_string()),
            ..Default::default()
        };

        let printable = PrintableData::IssueCreated {
            issues: vec![created_issue],
        };

        match printable {
            PrintableData::IssueCreated { issues } => {
                assert_eq!(issues.len(), 1);
                assert_eq!(issues[0].id, Some("10001".to_string()));
                assert_eq!(issues[0].key, Some("TEST-1".to_string()));
            }
            _ => panic!("Expected IssueCreated variant"),
        }
    }

    #[test]
    fn test_printable_data_issue_data() {
        let issue = IssueBean {
            id: Some("10001".to_string()),
            key: Some("TEST-1".to_string()),
            ..Default::default()
        };

        let printable = PrintableData::IssueData {
            issues: vec![issue],
        };

        match printable {
            PrintableData::IssueData { issues } => {
                assert_eq!(issues.len(), 1);
                assert_eq!(issues[0].id, Some("10001".to_string()));
                assert_eq!(issues[0].key, Some("TEST-1".to_string()));
            }
            _ => panic!("Expected IssueData variant"),
        }
    }

    #[test]
    fn test_printable_data_project() {
        let project = Project {
            id: Some("10000".to_string()),
            key: Some("TEST".to_string()),
            name: Some("Test Project".to_string()),
            ..Default::default()
        };

        let printable = PrintableData::Project {
            projects: vec![project],
        };

        match printable {
            PrintableData::Project { projects } => {
                assert_eq!(projects.len(), 1);
                assert_eq!(projects[0].id, Some("10000".to_string()));
                assert_eq!(projects[0].key, Some("TEST".to_string()));
                assert_eq!(projects[0].name, Some("Test Project".to_string()));
            }
            _ => panic!("Expected Project variant"),
        }
    }

    #[test]
    fn test_printable_data_version() {
        let version = Version {
            id: Some("10000".to_string()),
            name: Some("1.0.0".to_string()),
            description: Some("Initial release".to_string()),
            ..Default::default()
        };

        let printable = PrintableData::Version {
            versions: vec![version],
        };

        match printable {
            PrintableData::Version { versions } => {
                assert_eq!(versions.len(), 1);
                assert_eq!(versions[0].id, Some("10000".to_string()));
                assert_eq!(versions[0].name, Some("1.0.0".to_string()));
                assert_eq!(versions[0].description, Some("Initial release".to_string()));
            }
            _ => panic!("Expected Version variant"),
        }
    }

    #[test]
    fn test_printable_data_issue_transitions() {
        let transition = IssueTransition {
            id: Some("1".to_string()),
            name: Some("Done".to_string()),
            ..Default::default()
        };

        let printable = PrintableData::IssueTransitions {
            transitions: vec![transition],
        };

        match printable {
            PrintableData::IssueTransitions { transitions } => {
                assert_eq!(transitions.len(), 1);
                assert_eq!(transitions[0].id, Some("1".to_string()));
                assert_eq!(transitions[0].name, Some("Done".to_string()));
            }
            _ => panic!("Expected IssueTransitions variant"),
        }
    }

    #[test]
    fn test_printable_data_issue_type() {
        let issue_type = IssueTypeIssueCreateMetadata {
            id: Some("10001".to_string()),
            name: Some("Bug".to_string()),
            description: Some("A bug report".to_string()),
            ..Default::default()
        };

        let printable = PrintableData::IssueType {
            issue_types: vec![issue_type],
        };

        match printable {
            PrintableData::IssueType { issue_types } => {
                assert_eq!(issue_types.len(), 1);
                assert_eq!(issue_types[0].id, Some("10001".to_string()));
                assert_eq!(issue_types[0].name, Some("Bug".to_string()));
            }
            _ => panic!("Expected IssueType variant"),
        }
    }

    #[test]
    fn test_printable_data_issue_type_field() {
        let field = FieldCreateMetadata {
            key: "summary".to_string(),
            name: "Summary".to_string(),
            required: true,
            field_id: "summary".to_string(),
            operations: vec!["set".to_string()],
            schema: Default::default(),
            ..Default::default()
        };

        let printable = PrintableData::IssueTypeField {
            issue_type_fields: vec![field],
        };

        match printable {
            PrintableData::IssueTypeField { issue_type_fields } => {
                assert_eq!(issue_type_fields.len(), 1);
                assert_eq!(issue_type_fields[0].key, "summary".to_string());
                assert_eq!(issue_type_fields[0].name, "Summary".to_string());
                assert!(issue_type_fields[0].required);
            }
            _ => panic!("Expected IssueTypeField variant"),
        }
    }

    #[test]
    fn test_printable_data_empty_collections() {
        let printable_generic = PrintableData::Generic { data: Vec::new() };
        let printable_projects = PrintableData::Project {
            projects: Vec::new(),
        };
        let printable_versions = PrintableData::Version {
            versions: Vec::new(),
        };

        match printable_generic {
            PrintableData::Generic { data } => assert_eq!(data.len(), 0),
            _ => panic!("Expected Generic variant"),
        }

        match printable_projects {
            PrintableData::Project { projects } => assert_eq!(projects.len(), 0),
            _ => panic!("Expected Project variant"),
        }

        match printable_versions {
            PrintableData::Version { versions } => assert_eq!(versions.len(), 0),
            _ => panic!("Expected Version variant"),
        }
    }

    #[test]
    fn test_printable_data_serialization() {
        let project = Project {
            id: Some("10000".to_string()),
            key: Some("TEST".to_string()),
            name: Some("Test Project".to_string()),
            ..Default::default()
        };

        let printable = PrintableData::Project {
            projects: vec![project],
        };

        let serialized =
            serde_json::to_string(&printable).expect("Failed to serialize PrintableData");
        assert!(serialized.contains("\"id\":\"10000\""));
        assert!(serialized.contains("\"key\":\"TEST\""));
        assert!(serialized.contains("\"name\":\"Test Project\""));
    }

    #[test]
    fn test_multiple_items_in_collections() {
        let project1 = Project {
            id: Some("10000".to_string()),
            key: Some("PROJ1".to_string()),
            name: Some("Project 1".to_string()),
            ..Default::default()
        };

        let project2 = Project {
            id: Some("10001".to_string()),
            key: Some("PROJ2".to_string()),
            name: Some("Project 2".to_string()),
            ..Default::default()
        };

        let printable = PrintableData::Project {
            projects: vec![project1, project2],
        };

        match printable {
            PrintableData::Project { projects } => {
                assert_eq!(projects.len(), 2);
                assert_eq!(projects[0].key, Some("PROJ1".to_string()));
                assert_eq!(projects[1].key, Some("PROJ2".to_string()));
            }
            _ => panic!("Expected Project variant"),
        }
    }

    #[test]
    fn test_output_args_creation() {
        let output_args = OutputArgs {
            output_format: Some(OutputValues::Json),
            output_type: Some(OutputTypes::Basic),
        };

        assert!(matches!(
            output_args.output_format,
            Some(OutputValues::Json)
        ));
        assert!(matches!(output_args.output_type, Some(OutputTypes::Basic)));
    }

    #[test]
    fn test_output_args_default_values() {
        let output_args = OutputArgs {
            output_format: None,
            output_type: None,
        };

        assert!(output_args.output_format.is_none());
        assert!(output_args.output_type.is_none());
    }

    #[test]
    fn test_complex_json_data() {
        let complex_json = serde_json::json!({
            "project": {
                "id": "10000",
                "key": "TEST"
            },
            "issues": [
                {"id": "1", "key": "TEST-1"},
                {"id": "2", "key": "TEST-2"}
            ],
            "metadata": {
                "total": 2,
                "maxResults": 50
            }
        });

        let printable = PrintableData::Generic {
            data: vec![complex_json],
        };

        match printable {
            PrintableData::Generic { data } => {
                assert_eq!(data.len(), 1);
                let json_obj = &data[0];
                assert!(json_obj.get("project").is_some());
                assert!(json_obj.get("issues").is_some());
                assert!(json_obj.get("metadata").is_some());
            }
            _ => panic!("Expected Generic variant"),
        }
    }
}
