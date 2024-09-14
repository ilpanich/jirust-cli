use super::PrintableData;

pub fn print_json(data: PrintableData) {
    match data {
        PrintableData::Generic { data } => {
            println!("{}", serde_json::to_string_pretty(&data).unwrap());
        }
        PrintableData::IssueTransitions { transitions } => {
            println!("{}", serde_json::to_string_pretty(&transitions).unwrap());
        }
        PrintableData::IssueCreated { issues } => {
            println!("{}", serde_json::to_string_pretty(&issues).unwrap());
        }
        PrintableData::IssueData { issues } => {
            println!("{}", serde_json::to_string_pretty(&issues).unwrap());
        }
        PrintableData::IssueType { issue_types } => {
            println!("{}", serde_json::to_string_pretty(&issue_types).unwrap());
        }
        PrintableData::IssueTypeField { issue_type_fields } => {
            println!(
                "{}",
                serde_json::to_string_pretty(&issue_type_fields).unwrap()
            );
        }
        PrintableData::Project { projects } => {
            println!("{}", serde_json::to_string_pretty(&projects).unwrap());
        }
        PrintableData::Version { versions } => {
            println!("{}", serde_json::to_string_pretty(&versions).unwrap());
        }
    }
}
