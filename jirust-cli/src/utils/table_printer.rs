use prettytable::{Attr, Cell, Row, color};
use std::collections::HashMap;

use jira_v3_openapi::models::{
    CreatedIssue, FieldCreateMetadata, IssueBean, IssueTransition, IssueTypeIssueCreateMetadata,
    Project, ProjectCategory, Version,
};

use super::PrintableData;

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
/// use jirust_cli::utils::{PrintableData, table_printer::print_table_full};
///
/// let versions: Vec<Version> = vec![Version::new()];
///
/// print_table_full(PrintableData::Version{ versions });
/// ```
///
pub fn print_table_full(data: PrintableData) {
    let mut table = prettytable::Table::new();
    match data {
        PrintableData::Generic { data } => todo!("To Be Implemented! {:?}", data),
        PrintableData::IssueCreated { issues } => {
            table.add_row(row![
                bFC->"Issue ID",
                bFy->"Issue Key",
                bFm->"Issue URL",
            ]);
            for issue in issues {
                table.add_row(row![
                    Fc->issue.id.unwrap_or("".to_string()),
                    Fy->issue.key.unwrap_or("".to_string()),
                    Fm->issue.param_self.unwrap_or("".to_string()),
                ]);
            }
        }
        PrintableData::IssueData { issues } => {
            table.add_row(row![
                bFC->"Issue ID",
                bFy->"Issue Key",
                bFm->"Issue Fields",
                bFw->"Issue Transitions"
            ]);
            for issue in issues {
                let fields = issue
                    .fields
                    .unwrap_or(HashMap::new())
                    .iter()
                    .map(|field| format!("{}: {:?}", field.0, field.1.to_string()))
                    .collect::<Vec<String>>()
                    .join(", ");
                let transitions = issue
                    .transitions
                    .unwrap_or_default()
                    .iter()
                    .map(|transition| {
                        format!(
                            "{}: {} ({})",
                            transition.clone().id.unwrap_or_default(),
                            transition.clone().name.unwrap_or_default(),
                            transition.is_available.unwrap_or_default()
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(", ");
                table.add_row(row![
                    Fc->issue.id.unwrap_or("".to_string()),
                    Fy->issue.key.unwrap_or("".to_string()),
                    Fm->fields,
                    Fw->transitions
                ]);
            }
        }
        PrintableData::Project { projects } => {
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
        PrintableData::Version { versions } => {
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
        PrintableData::IssueTransitions { transitions } => {
            table.add_row(row![
                bFC->"Transition ID",
                bFc->"Name",
                bFm->"fields",
            ]);
            for transition in transitions {
                table.add_row(row![
                    Fc->transition.id.unwrap_or("".to_string()),
                    Fc->transition.name.unwrap_or("".to_string()),
                    Fm->transition.fields.unwrap_or(HashMap::new()).iter().map(|field| format!("{}: {:?}", field.0, field.1)).collect::<Vec<String>>().join(", "),
                ]);
            }
        }
        PrintableData::IssueType { issue_types } => {
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
        PrintableData::IssueTypeField { issue_type_fields } => {
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
        PrintableData::TransitionedIssue { issues } => {
            table.add_row(row![
                bFC->"Issue ID",
                bFy->"Transitioned",
                bFm->"Assigned",
                bFw->"Fix Version"
            ]);
            for issue in issues {
                table.add_row(Row::new(vec![
                    Cell::new(issue.0.as_str())
                        .with_style(Attr::Bold)
                        .with_style(Attr::ForegroundColor(color::CYAN)),
                    Cell::new(issue.1.as_str()).with_style(if issue.1 == "OK" {
                        Attr::ForegroundColor(color::GREEN)
                    } else {
                        Attr::ForegroundColor(color::RED)
                    }),
                    Cell::new(issue.2.as_str()).with_style(if issue.2 == "OK" {
                        Attr::ForegroundColor(color::GREEN)
                    } else {
                        Attr::ForegroundColor(color::RED)
                    }),
                    Cell::new(issue.3.as_str()).with_style(if issue.3 != "NO fixVersion set" {
                        Attr::ForegroundColor(color::GREEN)
                    } else {
                        Attr::ForegroundColor(color::RED)
                    }),
                ]));
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
/// use jirust_cli::utils::{PrintableData, table_printer::print_table_basic};
///
/// let versions: Vec<Version> = vec![Version::new()];
/// print_table_basic(PrintableData::Version { versions });
/// ```
pub fn print_table_basic(data: PrintableData) {
    let mut table = prettytable::Table::new();
    match data {
        PrintableData::Generic { data } => todo!("To Be Implemented! {:?}", data),
        PrintableData::IssueCreated { issues } => {
            table.add_row(row![
                bFC->"Issue ID",
                bFy->"Issue Key",
                bFm->"Issue URL",
            ]);
            for issue in issues {
                table.add_row(row![
                    Fc->issue.id.unwrap_or("".to_string()),
                    Fy->issue.key.unwrap_or("".to_string()),
                    Fm->issue.param_self.unwrap_or("".to_string()),
                ]);
            }
        }
        PrintableData::IssueData { issues } => {
            table.add_row(row![
                bFC->"Issue ID",
                bFy->"Issue Key",
            ]);
            for issue in issues {
                table.add_row(row![
                    Fc->issue.id.unwrap_or("".to_string()),
                    Fy->issue.key.unwrap_or("".to_string()),
                ]);
            }
        }
        PrintableData::Project { projects } => {
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
        PrintableData::Version { versions } => {
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
        PrintableData::IssueTransitions { transitions } => {
            table.add_row(row![
                bFC->"Transition ID",
                bFc->"Name",
                bFm->"fields",
            ]);
            for transition in transitions {
                table.add_row(row![
                    Fc->transition.id.unwrap_or("".to_string()),
                    Fc->transition.name.unwrap_or("".to_string()),
                    Fm->transition.fields.unwrap_or(HashMap::new()).iter().map(|field| format!("{}: {:?}", field.0, field.1)).collect::<Vec<String>>().join(", "),
                ]);
            }
        }
        PrintableData::IssueType { issue_types } => {
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
        PrintableData::IssueTypeField { issue_type_fields } => {
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
        PrintableData::TransitionedIssue { issues } => {
            table.add_row(row![
                bFC->"Issue ID",
                bFy->"Transitioned",
                bFm->"Assigned",
                bFw->"Fix Version"
            ]);
            for issue in issues {
                table.add_row(Row::new(vec![
                    Cell::new(issue.0.as_str())
                        .with_style(Attr::Bold)
                        .with_style(Attr::ForegroundColor(color::CYAN)),
                    Cell::new(issue.1.as_str()).with_style(if issue.1 == "OK" {
                        Attr::ForegroundColor(color::GREEN)
                    } else {
                        Attr::ForegroundColor(color::RED)
                    }),
                    Cell::new(issue.2.as_str()).with_style(if issue.2 == "OK" {
                        Attr::ForegroundColor(color::GREEN)
                    } else {
                        Attr::ForegroundColor(color::RED)
                    }),
                    Cell::new(issue.3.as_str()).with_style(if issue.3 != "NO fixVersion set" {
                        Attr::ForegroundColor(color::GREEN)
                    } else {
                        Attr::ForegroundColor(color::RED)
                    }),
                ]));
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
/// use jirust_cli::utils::{PrintableData, table_printer::print_table_single};
///
/// let version: Version = Version::new();
/// print_table_single(PrintableData::Version { versions: vec![version] });
/// ```
pub fn print_table_single(data: PrintableData) {
    let mut table = prettytable::Table::new();
    match data {
        PrintableData::Generic { data } => todo!("To Be Implemented! {:?}", data),
        PrintableData::IssueCreated { issues } => {
            let issue = issues.first().unwrap_or(&CreatedIssue::default()).clone();
            table.add_row(row![
                bFC->"Issue ID",
                bFy->"Issue Key",
                bFm->"Issue URL",
            ]);

            table.add_row(row![
                Fc->issue.id.unwrap_or("".to_string()),
                Fy->issue.key.unwrap_or("".to_string()),
                Fm->issue.param_self.unwrap_or("".to_string()),
            ]);
        }
        PrintableData::IssueData { issues } => {
            let issue = issues.first().unwrap_or(&IssueBean::default()).clone();
            table.add_row(row![
                bFC->"Issue ID",
                bFy->"Issue Key",
                bFm->"Issue Fields",
                bFw->"Issue Transitions"
            ]);
            let fields = issue
                .fields
                .unwrap_or(HashMap::new())
                .iter()
                .map(|field| format!("{}: {:?}", field.0, field.1.to_string()))
                .collect::<Vec<String>>()
                .join(", ");
            let transitions = issue
                .transitions
                .unwrap_or_default()
                .iter()
                .map(|transition| {
                    format!(
                        "{}: {} ({})",
                        transition.clone().id.unwrap_or_default(),
                        transition.clone().name.unwrap_or_default(),
                        transition.is_available.unwrap_or_default()
                    )
                })
                .collect::<Vec<String>>()
                .join(", ");
            table.add_row(row![
                Fc->issue.id.unwrap_or("".to_string()),
                Fy->issue.key.unwrap_or("".to_string()),
                Fm->fields,
                Fw->transitions
            ]);
        }
        PrintableData::Project { projects } => {
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
        PrintableData::Version { versions } => {
            let version = versions.first().unwrap_or(&Version::default()).clone();
            table.add_row(row![
                bFm->"Name",
                bFy->"Start Date",
                bFr->"Release Date",
            ]);

            table.add_row(row![
                Fm->version.name.unwrap_or_default(),
                Fy->version.start_date.unwrap_or_default(),
                Fr->version.release_date.unwrap_or_default(),
            ]);
        }
        PrintableData::IssueTransitions { transitions } => {
            let transition = transitions
                .first()
                .unwrap_or(&IssueTransition::default())
                .clone();
            table.add_row(row![
                bFC->"Transition ID",
                bFc->"Name",
                bFm->"fields",
            ]);
            table.add_row(row![
                Fc->transition.id.unwrap_or("".to_string()),
                Fc->transition.name.unwrap_or("".to_string()),
                Fm->transition.fields.unwrap_or(HashMap::new()).iter().map(|field| format!("{}: {:?}", field.0, field.1)).collect::<Vec<String>>().join(", "),
            ]);
        }
        PrintableData::IssueType { issue_types } => {
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
        PrintableData::IssueTypeField { issue_type_fields } => {
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
        PrintableData::TransitionedIssue { issues } => {
            table.add_row(row![
                bFC->"Issue ID",
                bFy->"Transitioned",
                bFm->"Assigned",
                bFw->"Fix Version"
            ]);
            for issue in issues {
                table.add_row(Row::new(vec![
                    Cell::new(issue.0.as_str())
                        .with_style(Attr::Bold)
                        .with_style(Attr::ForegroundColor(color::CYAN)),
                    Cell::new(issue.1.as_str()).with_style(if issue.1 == "OK" {
                        Attr::ForegroundColor(color::GREEN)
                    } else {
                        Attr::ForegroundColor(color::RED)
                    }),
                    Cell::new(issue.2.as_str()).with_style(if issue.2 == "OK" {
                        Attr::ForegroundColor(color::GREEN)
                    } else {
                        Attr::ForegroundColor(color::RED)
                    }),
                    Cell::new(issue.3.as_str()).with_style(if issue.3 != "NO fixVersion set" {
                        Attr::ForegroundColor(color::GREEN)
                    } else {
                        Attr::ForegroundColor(color::RED)
                    }),
                ]));
            }
        }
    }

    table.printstd();
}
