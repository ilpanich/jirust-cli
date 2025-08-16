#[cfg(test)]
mod tests {
    use crate::utils::{PrintableData, json_printer::print_json, table_printer::{print_table_full, print_table_basic, print_table_single}};
    use jira_v3_openapi::models::{
        CreatedIssue, IssueBean, IssueTransition
    };
    use serde_json::{json};
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
        
        let issues = vec![
            IssueBean {
                expand: None,
                id: Some("10001".to_string()),
                param_self: Some("https://test.atlassian.net/rest/api/3/issue/10001".to_string()),
                key: Some("TEST-1".to_string()),
                rendered_fields: None,
                properties: None,
                names: None,
                schema: None,
                transitions: Some(vec![
                    IssueTransition {
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
                    }
                ]),
                operations: None,
                editmeta: None,
                changelog: None,
                versioned_representations: None,
                fields_to_include: None,
                fields: Some(fields),
            },
        ];
        
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
        ];
        
        let printable = PrintableData::IssueTransitions { transitions: transitions.clone() };
        print_table_full(printable);

        let printable2 = PrintableData::IssueTransitions { transitions: transitions.clone() };
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
        let printable2 = PrintableData::IssueCreated { issues: empty_issues };
        print_json(printable2);
        
        assert!(true);
    }

    #[test] 
    fn test_json_printer_with_null_values() {
        let data_with_nulls = vec![
            json!({"id": null, "name": "test", "value": null}),
            json!({"id": "1", "name": null, "value": 42}),
        ];
        
        let printable = PrintableData::Generic { data: data_with_nulls };
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


}