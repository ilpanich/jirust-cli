pub mod table_printer;

use jira_v3_openapi::models::{
    FieldCreateMetadata, IssueTypeIssueCreateMetadata, Project, Version,
};

pub enum TablePrintable {
    IssueType {
        issue_types: Vec<IssueTypeIssueCreateMetadata>,
    },
    IssueTypeFields {
        issue_type_fields: Vec<FieldCreateMetadata>,
    },
    Project {
        projects: Vec<Project>,
    },
    Version {
        versions: Vec<Version>,
    },
}
