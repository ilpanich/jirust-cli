pub mod changelog_extractor;
pub mod json_printer;
pub mod macros;
pub mod table_printer;

use jira_v3_openapi::models::{
    CreatedIssue, FieldCreateMetadata, IssueBean, IssueTransition, IssueTypeIssueCreateMetadata,
    Project, Version, VersionRelatedWork,
};
use serde::Serialize;
use serde_json::Value;

use crate::args::commands::{OutputTypes, OutputValues};

/// Enum to hold the different types of data that can be printed in a table
///
/// # Variants
///
/// * `IssueType` - Jira Issue types available in a project data
/// * `IssueTypeField` - Fields available for a specific issue type in a project data
/// * `Project` - Projects available in Jira data
/// * `Version` - Versions available in a project data
/// * `IssueCreated` - Issues created in a project data
/// * `IssueTransition` - Issues transitions in a project data
/// * `TransitionedIssue` - Issues transitioned data
/// * `VersionRelatedWork` - Version related work items data
#[derive(Serialize)]
pub enum PrintableData {
    Generic {
        data: Vec<Value>,
    },
    IssueCreated {
        issues: Vec<CreatedIssue>,
    },
    IssueData {
        issues: Vec<IssueBean>,
    },
    IssueTransitions {
        transitions: Vec<IssueTransition>,
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
    TransitionedIssue {
        issues: Vec<(String, String, String, String)>,
    },
    Version {
        versions: Vec<Version>,
    },
    VersionRelatedWork {
        version_related_work_items: Vec<VersionRelatedWork>,
    },
}

#[derive(Clone)]
pub enum OutputType {
    Full,
    Basic,
    Single,
}

impl From<OutputTypes> for OutputType {
    fn from(output_type: OutputTypes) -> Self {
        match output_type {
            OutputTypes::Full => OutputType::Full,
            OutputTypes::Basic => OutputType::Basic,
            OutputTypes::Single => OutputType::Single,
        }
    }
}

pub fn print_data(data: PrintableData, output_format: OutputValues, output_type: OutputType) {
    match output_format {
        OutputValues::Json => {
            json_printer::print_json(data);
        }
        OutputValues::Table => match output_type {
            OutputType::Full => {
                table_printer::print_table_full(data);
            }
            OutputType::Basic => {
                table_printer::print_table_basic(data);
            }
            OutputType::Single => {
                table_printer::print_table_single(data);
            }
        },
    }
}
