#[cfg(test)]
mod tests {
    use crate::utils::{PrintableData, OutputType, print_data};
    use crate::utils::json_printer::print_json;
    use crate::utils::table_printer::{print_table_full, print_table_basic, print_table_single};
    use crate::args::commands::{OutputValues, OutputTypes};
    use jira_v3_openapi::models::{
        CreatedIssue, FieldCreateMetadata, IssueBean, IssueTransition, 
        IssueTypeIssueCreateMetadata, Project, Version, VersionRelatedWork
    };
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn test_output_type_from_conversion() {
        assert!(matches!(OutputType::from(OutputTypes::Full), OutputType::Full));
        assert!(matches!(OutputType::from(OutputTypes::Basic), OutputType::Basic));
        assert!(matches!(OutputType::from(OutputTypes::Single), OutputType::Single));
    }

    #[test]
    fn test_output_type_clone() {
        let original = OutputType::Full;
        let cloned = original.clone();
        
        // Since OutputType doesn't implement PartialEq, we'll test by using them
        match cloned {
            OutputType::Full => assert!(true),
            _ => assert!(false, "Clone should preserve the variant"),
        }
    }

    #[test]
    fn test_printable_data_generic_serialization() {
        let data = vec![
            json!({"id": "1", "name": "test"}),
            json!({"id": "2", "name": "another"}),
        ];
        let printable = PrintableData::Generic { data };
        
        let serialized = serde_json::to_string(&printable).expect("Should serialize");
        assert!(serialized.contains("test"));
        assert!(serialized.contains("another"));
    }

    #[test]
    fn test_printable_data_issue_created_serialization() {
        let issues = vec![
            CreatedIssue {
                id: Some("10001".to_string()),
                key: Some("TEST-1".to_string()),
                param_self: Some("https://test.com/issue/10001".to_string()),
                ..Default::default()
            },
        ];
        let printable = PrintableData::IssueCreated { issues };
        
        let serialized = serde_json::to_string(&printable).expect("Should serialize");
        assert!(serialized.contains("TEST-1"));
        assert!(serialized.contains("10001"));
    }

    #[test]
    fn test_printable_data_issue_data_serialization() {
        let mut fields = HashMap::new();
        fields.insert("summary".to_string(), json!("Test issue"));
        fields.insert("priority".to_string(), json!("High"));
        
        let issues = vec![
            IssueBean {
                id: Some("10001".to_string()),
                key: Some("TEST-1".to_string()),
                fields: Some(fields),
                transitions: Some(vec![
                    IssueTransition {
                        id: Some("1".to_string()),
                        name: Some("Done".to_string()),
                        is_available: Some(true),
                        ..Default::default()
                    }
                ]),
                ..Default::default()
            },
        ];
        let printable = PrintableData::IssueData { issues };
        
        let serialized = serde_json::to_string(&printable).expect("Should serialize");
        assert!(serialized.contains("TEST-1"));
        assert!(serialized.contains("10001"));
    }

    #[test]
    fn test_printable_data_project_serialization() {
        let projects = vec![
            Project {
                id: Some("10000".to_string()),
                key: Some("PROJ".to_string()),
                name: Some("Test Project".to_string()),
                description: Some("A test project".to_string()),
                ..Default::default()
            },
        ];
        let printable = PrintableData::Project { projects };
        
        let serialized = serde_json::to_string(&printable).expect("Should serialize");
        assert!(serialized.contains("PROJ"));
        assert!(serialized.contains("Test Project"));
    }

    #[test]
    fn test_printable_data_version_serialization() {
        let versions = vec![
            Version {
                id: Some("100".to_string()),
                name: Some("v1.0.0".to_string()),
                description: Some("Initial release".to_string()),
                project_id: Some(10000),
                released: Some(true),
                archived: Some(false),
                ..Default::default()
            },
        ];
        let printable = PrintableData::Version { versions };
        
        let serialized = serde_json::to_string(&printable).expect("Should serialize");
        assert!(serialized.contains("v1.0.0"));
        assert!(serialized.contains("Initial release"));
    }

    #[test]
    fn test_printable_data_version_related_work_serialization() {
        let work_items = vec![
            VersionRelatedWork {
                category: "Development".to_string(),
                issue_id: Some(10001),
                related_work_id: Some("WORK-1".to_string()),
                title: Some("Test work item".to_string()),
                url: Some("https://example.com".to_string()),
                ..Default::default()
            },
        ];
        let printable = PrintableData::VersionRelatedWork { 
            version_related_work_items: work_items 
        };
        
        let serialized = serde_json::to_string(&printable).expect("Should serialize");
        assert!(serialized.contains("Development"));
        assert!(serialized.contains("WORK-1"));
    }

    #[test]
    fn test_printable_data_issue_transitions_serialization() {
        let transitions = vec![
            IssueTransition {
                id: Some("1".to_string()),
                name: Some("Done".to_string()),
                is_available: Some(true),
                fields: None,
                ..Default::default()
            },
        ];
        let printable = PrintableData::IssueTransitions { transitions };
        
        let serialized = serde_json::to_string(&printable).expect("Should serialize");
        assert!(serialized.contains("Done"));
    }

    #[test]
    fn test_printable_data_issue_type_serialization() {
        let issue_types = vec![
            IssueTypeIssueCreateMetadata {
                id: Some("10001".to_string()),
                name: Some("Bug".to_string()),
                description: Some("A bug report".to_string()),
                hierarchy_level: Some(1),
                subtask: Some(false),
                ..Default::default()
            },
        ];
        let printable = PrintableData::IssueType { issue_types };
        
        let serialized = serde_json::to_string(&printable).expect("Should serialize");
        assert!(serialized.contains("Bug"));
        assert!(serialized.contains("bug report"));
    }

    #[test]
    fn test_printable_data_issue_type_field_serialization() {
        let fields = vec![
            FieldCreateMetadata {
                field_id: "summary".to_string(),
                key: "summary".to_string(),
                name: "Summary".to_string(),
                required: true,
                operations: vec!["set".to_string()],
                ..Default::default()
            },
        ];
        let printable = PrintableData::IssueTypeField { issue_type_fields: fields };
        
        let serialized = serde_json::to_string(&printable).expect("Should serialize");
        assert!(serialized.contains("summary"));
        assert!(serialized.contains("Summary"));
    }

    #[test]
    fn test_printable_data_transitioned_issue_serialization() {
        let issues = vec![
            ("TEST-1".to_string(), "OK".to_string(), "OK".to_string(), "v1.0.0".to_string()),
            ("TEST-2".to_string(), "FAILED".to_string(), "OK".to_string(), "NO fixVersion set".to_string()),
        ];
        let printable = PrintableData::TransitionedIssue { issues };
        
        let serialized = serde_json::to_string(&printable).expect("Should serialize");
        assert!(serialized.contains("TEST-1"));
        assert!(serialized.contains("TEST-2"));
    }

    #[test]
    fn test_print_data_json_format() {
        let data = PrintableData::Generic { 
            data: vec![json!({"test": "value"})] 
        };
        
        // This test mainly ensures the function doesn't panic
        // Since print_json outputs to stdout, we can't easily capture the output in tests
        // But we can ensure the code path is exercised
        print_data(data, OutputValues::Json, OutputType::Full);
    }

    #[test]
    fn test_print_data_table_full_format() {
        let projects = vec![
            Project {
                id: Some("10000".to_string()),
                key: Some("TEST".to_string()),
                name: Some("Test Project".to_string()),
                ..Default::default()
            },
        ];
        let data = PrintableData::Project { projects };
        
        // This test mainly ensures the function doesn't panic
        print_data(data, OutputValues::Table, OutputType::Full);
    }

    #[test]
    fn test_print_data_table_basic_format() {
        let projects = vec![
            Project {
                id: Some("10000".to_string()),
                key: Some("TEST".to_string()),
                name: Some("Test Project".to_string()),
                ..Default::default()
            },
        ];
        let data = PrintableData::Project { projects };
        
        // This test mainly ensures the function doesn't panic
        print_data(data, OutputValues::Table, OutputType::Basic);
    }

    #[test]
    fn test_print_data_table_single_format() {
        let projects = vec![
            Project {
                id: Some("10000".to_string()),
                key: Some("TEST".to_string()),
                name: Some("Test Project".to_string()),
                ..Default::default()
            },
        ];
        let data = PrintableData::Project { projects };
        
        // This test mainly ensures the function doesn't panic
        print_data(data, OutputValues::Table, OutputType::Single);
    }

    #[test]
    fn test_json_printer_with_empty_collections() {
        // Test with empty vectors
        let data = PrintableData::Project { projects: vec![] };
        print_json(data);
        
        let data = PrintableData::Version { versions: vec![] };
        print_json(data);
        
        let data = PrintableData::Generic { data: vec![] };
        print_json(data);
    }

    #[test]
    fn test_table_printer_with_empty_collections() {
        // Test table printers with empty data
        let data1 = PrintableData::Project { projects: vec![] };
        print_table_full(data1);
        
        let data2 = PrintableData::Project { projects: vec![] };
        print_table_basic(data2);
        
        let data3 = PrintableData::Project { projects: vec![] };
        print_table_single(data3);
        
        let data4 = PrintableData::Version { versions: vec![] };
        print_table_full(data4);
        
        let data5 = PrintableData::Version { versions: vec![] };
        print_table_basic(data5);
        
        let data6 = PrintableData::Version { versions: vec![] };
        print_table_single(data6);
    }

    #[test]
    fn test_table_printer_with_minimal_data() {
        // Test with minimal data that should work
        let projects = vec![
            Project {
                ..Default::default()
            },
        ];
        let data1 = PrintableData::Project { projects: projects.clone() };
        print_table_full(data1);
        
        let data2 = PrintableData::Project { projects: projects.clone() };
        print_table_basic(data2);
        
        let data3 = PrintableData::Project { projects };
        print_table_single(data3);
    }

    #[test]
    fn test_all_printable_data_variants_coverage() {
        // Ensure we can create and use all variants
        let _ = PrintableData::Generic { data: vec![] };
        let _ = PrintableData::IssueCreated { issues: vec![] };
        let _ = PrintableData::IssueData { issues: vec![] };
        let _ = PrintableData::IssueTransitions { transitions: vec![] };
        let _ = PrintableData::IssueType { issue_types: vec![] };
        let _ = PrintableData::IssueTypeField { issue_type_fields: vec![] };
        let _ = PrintableData::Project { projects: vec![] };
        let _ = PrintableData::TransitionedIssue { issues: vec![] };
        let _ = PrintableData::Version { versions: vec![] };
        let _ = PrintableData::VersionRelatedWork { version_related_work_items: vec![] };
    }

    #[test]
    fn test_complex_nested_data_structures() {
        // Test with complex nested structures
        let mut fields = HashMap::new();
        fields.insert("summary".to_string(), json!("Complex issue with nested data"));
        fields.insert("labels".to_string(), json!(["bug", "critical", "ui"]));
        fields.insert("custom_field".to_string(), json!({
            "id": "10001",
            "value": {
                "nested": "data",
                "array": [1, 2, 3]
            }
        }));
        
        let issues = vec![
            IssueBean {
                id: Some("10001".to_string()),
                key: Some("COMPLEX-1".to_string()),
                fields: Some(fields),
                transitions: Some(vec![
                    IssueTransition {
                        id: Some("1".to_string()),
                        name: Some("Done".to_string()),
                        is_available: Some(true),
                        fields: None,
                        ..Default::default()
                    }
                ]),
                ..Default::default()
            },
        ];
        
        let data = PrintableData::IssueData { issues };
        let serialized = serde_json::to_string(&data).expect("Should serialize complex data");
        assert!(serialized.contains("COMPLEX-1"));
        assert!(serialized.contains("nested"));
        assert!(serialized.contains("critical"));
    }
}