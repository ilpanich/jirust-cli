//! Shared utilities for printing, formatting, and changelog handling.
/// Utilities to extract release notes from changelog files.
pub mod changelog_extractor;
/// JSON output helpers.
pub mod json_printer;
/// Reusable macros.
pub mod macros;
/// Pretty-table renderers.
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
    /// Generic JSON data returned from Jira endpoints.
    Generic {
        /// Raw JSON payloads to print.
        data: Vec<Value>,
    },
    /// Issues created by a command.
    IssueCreated {
        /// Created issue payloads.
        issues: Vec<CreatedIssue>,
    },
    /// Issue data retrieved from Jira.
    IssueData {
        /// Issues to print.
        issues: Vec<IssueBean>,
    },
    /// Available transitions for an issue.
    IssueTransitions {
        /// Transition details.
        transitions: Vec<IssueTransition>,
    },
    /// Issue types associated to a project.
    IssueType {
        /// Project issue types.
        issue_types: Vec<IssueTypeIssueCreateMetadata>,
    },
    /// Fields for a given issue type.
    IssueTypeField {
        /// Field metadata grouped by issue type.
        issue_type_fields: Vec<FieldCreateMetadata>,
    },
    /// Jira projects.
    Project {
        /// Project list to print.
        projects: Vec<Project>,
    },
    /// Issues transitioned by a workflow automation.
    TransitionedIssue {
        /// Tuple of (issue key, transition, assignee, fixVersion).
        issues: Vec<(String, String, String, String)>,
    },
    /// Versions associated to a project.
    Version {
        /// Version list to print.
        versions: Vec<Version>,
    },
    /// Work items related to a version.
    VersionRelatedWork {
        /// Related work entries.
        version_related_work_items: Vec<VersionRelatedWork>,
    },
}

/// Output verbosity level used when printing table data.
#[derive(Clone)]
pub enum OutputType {
    /// Print all available columns.
    Full,
    /// Print a subset of useful columns.
    Basic,
    /// Print only a single row payload.
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

/// Prints the given data using the requested format and verbosity.
///
/// # Arguments
/// * `data` - Data container to print.
/// * `output_format` - Output format (table or json).
/// * `output_type` - Verbosity level for table output.
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
