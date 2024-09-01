pub mod changelog_extractor;
pub mod table_printer;

use jira_v3_openapi::models::{
    CreatedIssue, FieldCreateMetadata, IssueBean, IssueTypeIssueCreateMetadata, Project, Version,
};
use serde_json::Value;

/// Enum to hold the different types of data that can be printed in a table
///
/// # Variants
///
/// * `IssueType` - Jira Issue types available in a project data
/// * `IssueTypeField` - Fields available for a specific issue type in a project data
/// * `Project` - Projects available in Jira data
/// * `Version` - Versions available in a project data
pub enum TablePrintable {
    Generic {
        data: Vec<Value>,
    },
    IssueCreated {
        issues: Vec<CreatedIssue>,
    },
    IssueData {
        issues: Vec<IssueBean>,
    },
    IssueType {
        issue_types: Vec<IssueTypeIssueCreateMetadata>,
    },
    IssueTypeField {
        issue_type_fields: Vec<FieldCreateMetadata>,
    },
    Project {
        projects: Vec<Project>,
    },
    Version {
        versions: Vec<Version>,
    },
}
