use jira_v3_openapi::models::{
    FieldCreateMetadata, IssueTypeIssueCreateMetadata, Project, ProjectCategory, Version,
};

use super::TablePrintable;

/// This function allows to print the objects details in a pretty way (full data).
///
/// It uses the prettytable library to print the version details.
///
/// # Arguments
///
/// * `data` - A TablePrintable enum
///     * Project: A vector of Project structs
///     * Version: A vector of Version structs
///     * IssueType: A vector of IssueType structs
///     * IssueTypeFields: A vector of IssueTypeFields structs
///
/// # Examples
///
/// ```
/// use jira_v3_openapi::models::Version;
/// use jirust_cli::utils::{TablePrintable, table_printer::print_table_full};
///
/// let versions: Vec<Version> = vec![Version::new()];
///
/// print_table_full(TablePrintable::Version{ versions });
/// ```
///
pub fn print_table_full(data: TablePrintable) {
    let mut table = prettytable::Table::new();
    match data {
        TablePrintable::Project { projects } => {
            table.add_row(row![
                bFC->"Project ID",
                bFc->"Project Key",
                bFm->"Name",
                bFw->"Description",
                bFb->"Category",
            ]);
            for project in projects {
                table.add_row(row![
                    Fc->project.id.unwrap_or("".to_string()),
                    Fc->project.key.unwrap_or("".to_string()),
                    Fm->project.name.unwrap_or("".to_string()),
                    Fw->project.description.unwrap_or("".to_string()),
                    Fb->project.project_category.unwrap_or(Box::new(ProjectCategory::default())).name.unwrap_or("".to_string()),
                ]);
            }
        }
        TablePrintable::Version { versions } => {
            table.add_row(row![
                bFC->"Project ID",
                bFc->"ID",
                bFm->"Name",
                bFw->"Description",
                bFy->"Start Date",
                bFr->"Release Date",
                bFb->"Archived",
                bFg->"Released"
            ]);

            for version in versions {
                table.add_row(row![
                    FC->version.project_id.unwrap_or_default(),
                    Fc->version.id.unwrap_or_default(),
                    Fm->version.name.unwrap_or_default(),
                    Fw->version.description.unwrap_or_default(),
                    Fy->version.start_date.unwrap_or_default(),
                    Fr->version.release_date.unwrap_or_default(),
                    Fb->version.archived.unwrap_or_default(),
                    Fg->version.released.unwrap_or_default()
                ]);
            }
        }
        TablePrintable::IssueType { issue_types } => {
            table.add_row(row![
                bFC->"Issue Type ID",
                bFc->"Name",
                bFm->"Description",
                bFw->"Hierarchy Level",
                bFb->"Subtasks",
            ]);
            for issue_type in issue_types {
                table.add_row(row![
                    Fc->issue_type.id.unwrap_or("".to_string()),
                    Fc->issue_type.name.unwrap_or("".to_string()),
                    Fm->issue_type.description.unwrap_or("".to_string()),
                    Fw->issue_type.hierarchy_level.unwrap_or(-1),
                    Fb->issue_type.subtask.unwrap_or(false),
                ]);
            }
        }
        TablePrintable::IssueTypeField { issue_type_fields } => {
            table.add_row(row![
                bFC->"Field ID",
                bFc->"Field Key",
                bFm->"Field Name",
                bFb->"Required",
            ]);
            for field in issue_type_fields {
                table.add_row(row![
                    Fc->field.field_id,
                    Fc->field.key,
                    Fm->field.name,
                    Fb->field.required,
                ]);
            }
        }
    }
    table.printstd();
}

/// This function allows to print the objects details in a pretty way (basic data).
///
/// It uses the prettytable library to print the version details.
///
/// # Arguments
///
/// * `data` - A TablePrintable enum
///     * Project: A vector of Project structs
///     * Version: A vector of Version structs
///     * IssueType: A vector of IssueType structs
///     * IssueTypeFields: A vector of IssueTypeFields structs
///
/// # Examples
///
/// ```
/// use jira_v3_openapi::models::Version;
/// use jirust_cli::utils::{TablePrintable, table_printer::print_table_basic};
///
/// let versions: Vec<Version> = vec![Version::new()];
/// print_table_basic(TablePrintable::Version { versions });
/// ```
pub fn print_table_basic(data: TablePrintable) {
    let mut table = prettytable::Table::new();
    match data {
        TablePrintable::Project { projects } => {
            table.add_row(row![
                bFC->"Project ID",
                bFc->"Project Key",
                bFm->"Name",
                bFw->"Description",
                bFb->"Category",
            ]);
            for project in projects {
                table.add_row(row![
                    Fc->project.id.unwrap_or("".to_string()),
                    Fc->project.key.unwrap_or("".to_string()),
                    Fm->project.name.unwrap_or("".to_string()),
                    Fw->project.description.unwrap_or("".to_string()),
                    Fb->project.project_category.unwrap_or(Box::new(ProjectCategory::default())).name.unwrap_or("".to_string()),
                ]);
            }
        }
        TablePrintable::Version { versions } => {
            table.add_row(row![
                bFC->"Project ID",
                bFc->"ID",
                bFm->"Name",
                bFy->"Start Date",
                bFr->"Release Date",
                bFb->"Archived",
                bFg->"Released"
            ]);

            for version in versions {
                table.add_row(row![
                    FC->version.project_id.unwrap_or_default(),
                    Fc->version.id.unwrap_or_default(),
                    Fm->version.name.unwrap_or_default(),
                    Fy->version.start_date.unwrap_or_default(),
                    Fr->version.release_date.unwrap_or_default(),
                    Fb->version.archived.unwrap_or_default(),
                    Fg->version.released.unwrap_or_default()
                ]);
            }
        }
        TablePrintable::IssueType { issue_types } => {
            table.add_row(row![
                bFC->"Issue Type ID",
                bFc->"Name",
                bFm->"Description",
                bFw->"Hierarchy Level",
                bFb->"Subtasks",
            ]);
            for issue_type in issue_types {
                table.add_row(row![
                    Fc->issue_type.id.unwrap_or("".to_string()),
                    Fc->issue_type.name.unwrap_or("".to_string()),
                    Fm->issue_type.description.unwrap_or("".to_string()),
                    Fw->issue_type.hierarchy_level.unwrap_or(-1),
                    Fb->issue_type.subtask.unwrap_or(false),
                ]);
            }
        }
        TablePrintable::IssueTypeField { issue_type_fields } => {
            table.add_row(row![
                bFC->"Field ID",
                bFc->"Field Key",
                bFm->"Field Name",
                bFb->"Required",
            ]);
            for field in issue_type_fields {
                table.add_row(row![
                    Fc->field.field_id,
                    Fc->field.key,
                    Fm->field.name,
                    Fb->field.required,
                ]);
            }
        }
    }
    table.printstd();
}

/// This function allows to print the objects details in a pretty way (single data).
///
/// It uses the prettytable library to print the version details.
///
///
/// # Arguments
///
/// * `data` - A TablePrintable enum
///     * Project: A vector of Project structs
///     * Version: A vector of Version structs
///     * IssueType: A vector of IssueType structs
///     * IssueTypeFields: A vector of IssueTypeFields structs
///
/// # Examples
///
/// ```
/// use jira_v3_openapi::models::Version;
/// use jirust_cli::utils::{TablePrintable, table_printer::print_table_single};
///
/// let version: Version = Version::new();
/// print_table_single(TablePrintable::Version { versions: vec![version] });
/// ```
pub fn print_table_single(data: TablePrintable) {
    let mut table = prettytable::Table::new();
    match data {
        TablePrintable::Project { projects } => {
            let project = projects.first().unwrap_or(&Project::default()).clone();
            table.add_row(row![
                bFC->"Project ID",
                bFc->"Project Key",
                bFm->"Name",
                bFw->"Description",
                bFb->"Category",
            ]);

            table.add_row(row![
                    Fc->project.id.unwrap_or("".to_string()),
                    Fc->project.key.unwrap_or("".to_string()),
                    Fm->project.name.unwrap_or("".to_string()),
                    Fw->project.description.unwrap_or("".to_string()),
                    Fb->project.project_category.unwrap_or(Box::new(ProjectCategory::default())).name.unwrap_or("".to_string()),
                ]);
        }
        TablePrintable::Version { versions } => {
            let version = versions.first().unwrap_or(&Version::default()).clone();
            table.add_row(row![
                FC->version.project_id.unwrap_or_default(),
                bFC->"Project ID",
                bFc->"ID",
                bFm->"Name",
                bFw->"Description",
                bFy->"Start Date",
                bFr->"Release Date",
                bFb->"Archived",
                bFg->"Released"
            ]);

            table.add_row(row![
                Fc->version.id.unwrap_or_default(),
                Fm->version.name.unwrap_or_default(),
                Fw->version.description.unwrap_or_default(),
                Fy->version.start_date.unwrap_or_default(),
                Fr->version.release_date.unwrap_or_default(),
                Fb->version.archived.unwrap_or_default(),
                Fg->version.released.unwrap_or_default()
            ]);
        }
        TablePrintable::IssueType { issue_types } => {
            let issue_type = issue_types
                .first()
                .unwrap_or(&IssueTypeIssueCreateMetadata::default())
                .clone();
            table.add_row(row![
                bFC->"Issue Type ID",
                bFc->"Name",
                bFm->"Description",
                bFw->"Hierarchy Level",
                bFb->"Subtasks",
            ]);

            table.add_row(row![
                Fc->issue_type.id.unwrap_or("".to_string()),
                Fc->issue_type.name.unwrap_or("".to_string()),
                Fm->issue_type.description.unwrap_or("".to_string()),
                Fw->issue_type.hierarchy_level.unwrap_or(-1),
                Fb->issue_type.subtask.unwrap_or(false),
            ]);
        }
        TablePrintable::IssueTypeField { issue_type_fields } => {
            let field = issue_type_fields
                .first()
                .unwrap_or(&FieldCreateMetadata::default())
                .clone();
            table.add_row(row![
                bFC->"Field ID",
                bFc->"Field Key",
                bFm->"Field Name",
                bFb->"Required",
            ]);

            table.add_row(row![
                Fc->field.field_id,
                Fc->field.key,
                Fm->field.name,
                Fb->field.required,
            ]);
        }
    }

    table.printstd();
}
