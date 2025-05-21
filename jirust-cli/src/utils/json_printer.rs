use super::PrintableData;

/// Print the data in JSON format
///
/// # Arguments
///
/// * `data` - The data to be printed
///
/// # Example
///
/// ```
/// use jira_v3_openapi::models::Version;
/// use jirust_cli::utils::{PrintableData, json_printer::print_json};
///
/// let versions: Vec<Version> = vec![Version::new()];
///
/// print_json(PrintableData::Version { versions });
/// ```
pub fn print_json(data: PrintableData) {
    match data {
        PrintableData::Generic { data } => {
            if let Ok(output) = serde_json::to_string_pretty(&data) {
                println!("{}", output);
            } else {
                println!("Failed to serialize data");
            }
        }
        PrintableData::IssueTransitions { transitions } => {
            if let Ok(output) = serde_json::to_string_pretty(&transitions) {
                println!("{}", output);
            } else {
                println!("Failed to serialize data");
            }
        }
        PrintableData::IssueCreated { issues } => {
            if let Ok(output) = serde_json::to_string_pretty(&issues) {
                println!("{}", output);
            } else {
                println!("Failed to serialize data");
            }
        }
        PrintableData::IssueData { issues } => {
            if let Ok(output) = serde_json::to_string_pretty(&issues) {
                println!("{}", output);
            } else {
                println!("Failed to serialize data");
            }
        }
        PrintableData::IssueType { issue_types } => {
            if let Ok(output) = serde_json::to_string_pretty(&issue_types) {
                println!("{}", output);
            } else {
                println!("Failed to serialize data");
            }
        }
        PrintableData::IssueTypeField { issue_type_fields } => {
            if let Ok(output) = serde_json::to_string_pretty(&issue_type_fields) {
                println!("{}", output);
            } else {
                println!("Failed to serialize data");
            }
        }
        PrintableData::Project { projects } => {
            if let Ok(output) = serde_json::to_string_pretty(&projects) {
                println!("{}", output);
            } else {
                println!("Failed to serialize data");
            }
        }
        PrintableData::TransitionedIssue { issues } => {
            let printable = issues
                .iter()
                .map(|(issue, transitioned, assigned, fix_version)| {
                    serde_json::json!({
                        "issue": issue,
                        "transitioned": transitioned,
                        "assigned": assigned,
                        "fixVersion": fix_version,
                    })
                })
                .collect::<Vec<_>>();
            if let Ok(output) = serde_json::to_string_pretty(&printable) {
                println!("{}", output);
            } else {
                println!("Failed to serialize data");
            }
        }
        PrintableData::Version { versions } => {
            if let Ok(output) = serde_json::to_string_pretty(&versions) {
                println!("{}", output);
            } else {
                println!("Failed to serialize data");
            }
        }
    }
}
