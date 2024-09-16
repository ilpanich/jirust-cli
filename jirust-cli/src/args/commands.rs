use clap::{Args, Parser, Subcommand, ValueEnum};
use std::error::Error;

/// Command line arguments base
/// subcommands: Config, Version
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct JirustCliArgs {
    #[clap(subcommand)]
    pub subcmd: Commands,
}

/// Available CLI commands
/// Config, Issue, Project, Transition, Version
#[derive(Subcommand, Debug)]
pub enum Commands {
    Config(ConfigArgs),
    Issue(IssueArgs),
    Project(ProjectArgs),
    Transition(TransitionArgs),
    Version(VersionArgs),
}

/// Available pagination command line arguments
///
/// * page_size: Option<i32>
/// * page_offset: Option<i64>
#[derive(Args, Debug)]
pub struct PaginationArgs {
    #[clap(
        long,
        short = 'l',
        value_name = "page_size",
        help = "page size for lists"
    )]
    pub page_size: Option<i32>,
    #[clap(
        long,
        short = 's',
        value_name = "page_offset",
        help = "page offset for list"
    )]
    pub page_offset: Option<i64>,
}

/// Available output values
/// Table, Json
#[derive(ValueEnum, Debug, Clone, Copy)]
#[value(rename_all = "kebab-case")]
pub enum OutputValues {
    #[value(name = "table", help = "Print output in table format")]
    Table,
    #[value(name = "json", help = "Print output in json format")]
    Json,
}

/// Available output values
///
/// * Table: Print output in table format
/// * Json: Print output in json format
#[derive(Args, Debug)]
pub struct OutputArgs {
    #[clap(long, short = 'o', value_name = "table|json", help = "Output format")]
    pub output: Option<OutputValues>,
}

/// Available configuration command line arguments
/// cfg_act: ConfigActionValues
///    Auth, Jira, Setup, Show
///
#[derive(Args, Debug)]
pub struct ConfigArgs {
    #[arg(
        value_name = "auth|jira|setup|show",
        help_heading = "Configuration management"
    )]
    pub cfg_act: ConfigActionValues,
}

/// Available configuration action values
/// Auth, Jira, Setup, Show
#[derive(ValueEnum, Debug, Clone, Copy)]
#[value(rename_all = "kebab-case")]
pub enum ConfigActionValues {
    #[value(name = "auth", help = "Set Jira API authentication (username, apikey)")]
    Auth,
    #[value(name = "jira", help = "Set Jira API base URL")]
    Jira,
    #[value(
        name = "setup",
        help = "Setup Jira API configuration (authentication data, jira base URL)"
    )]
    Setup,
    #[value(name = "show", help = "Show current configuration")]
    Show,
}

/// Available version command line arguments
/// version_act: VersionActionValues
///   Create, List, Update, Delete, Release, Archive
#[derive(Args, Debug)]
pub struct VersionArgs {
    #[arg(
        value_name = "archive|create|delete|list|release|update",
        help_heading = "Jira Project version management"
    )]
    pub version_act: VersionActionValues,
    #[clap(
        long,
        short = 'k',
        value_name = "project_key",
        required = true,
        help = "Jira Project key"
    )]
    pub project_key: String,
    #[clap(long, short = 'i', value_name = "project_id", help = "Jira Project ID")]
    pub project_id: Option<i64>,
    #[clap(
        long,
        short = 'v',
        value_name = "version_id",
        help = "Jira Project version ID"
    )]
    pub version_id: Option<String>,
    #[clap(
        long,
        short = 'n',
        value_name = "version_name",
        help = "Jira Project version name"
    )]
    pub version_name: Option<String>,
    #[clap(
        long,
        short = 'd',
        value_name = "version_description",
        help = "Jira Project version description"
    )]
    pub version_description: Option<String>,
    #[clap(
        long,
        value_name = "version_start_date",
        help = "Jira Project version start date"
    )]
    pub version_start_date: Option<String>,
    #[clap(
        long,
        value_name = "version_release_date",
        help = "Jira Project version release date"
    )]
    pub version_release_date: Option<String>,
    #[clap(
        long,
        short = 'a',
        value_name = "version_archived",
        help = "Jira Project version archived"
    )]
    pub version_archived: Option<bool>,
    #[clap(
        long,
        short = 'r',
        value_name = "version_released",
        help = "Jira Project version released"
    )]
    pub version_released: Option<bool>,
    #[clap(
        long,
        short = 'c',
        value_name = "changelog_file",
        help = "changelog file path to be used for automatic description generation (if set the script detects automatically the first tagged block in the changelog and use it as description)"
    )]
    pub changelog_file: Option<String>,
    #[clap(
        long,
        short = 't',
        value_name = "transition_issues",
        help = "if changelog is set and this flag is set, the script will transition all issues in the changelog of the current version release"
    )]
    pub transition_issues: Option<String>,
    #[clap(
        long,
        short = 'u',
        value_name = "transition_assignee",
        help = "if changelog is set and the resolve_issues flag is set, the script will transition all issues in the changelog of the current version release"
    )]
    pub transition_assignee: Option<String>,
    #[clap(flatten)]
    pub pagination: PaginationArgs,
    #[clap(flatten)]
    pub output: OutputArgs,
}

/// Available version action values
/// Archive, Create, Delete, List, Release, Update
#[derive(ValueEnum, Debug, Clone, Copy)]
#[value(rename_all = "kebab-case")]
pub enum VersionActionValues {
    #[value(name = "archive", help = "Archive a Jira Project version")]
    Archive,
    #[value(name = "create", help = "Create a Jira Project version")]
    Create,
    #[value(name = "delete", help = "Delete a Jira Project version")]
    Delete,
    #[value(name = "list", help = "List Jira Project versions")]
    List,
    #[value(name = "release", help = "Release a Jira Project version")]
    Release,
    #[value(name = "update", help = "Update a Jira Project version")]
    Update,
}

/// Available project command line arguments
/// version_act: ProjectActionValues
/// GetIssueTypes, GetIssueTypeFields, List
#[derive(Args, Debug)]
pub struct ProjectArgs {
    #[arg(
        value_name = "get-issue-types|get-issue-type-fields|list",
        help_heading = "Jira Project management",
        required = true
    )]
    pub project_act: ProjectActionValues,
    #[clap(
        long,
        short = 'k',
        value_name = "project_key",
        help = "Jira Project key",
        required = true
    )]
    pub project_key: Option<String>,
    #[clap(
        long,
        short = 'i',
        value_name = "project_issue_type",
        help = "Jira Project issue type ID"
    )]
    pub project_issue_type: Option<String>,
    #[clap(flatten)]
    pub pagination: PaginationArgs,
    #[clap(flatten)]
    pub output: OutputArgs,
}

/// Available project action values
/// GetIssueTypes, GetIssueTypeFields, List
#[derive(ValueEnum, Debug, Clone, Copy)]
#[value(rename_all = "kebab-case")]
pub enum ProjectActionValues {
    #[value(
        name = "get-issue-types",
        help = "Get Jira Project issue types by Jira project key"
    )]
    GetIssueTypes,
    #[value(
        name = "get-issue-type-fields",
        help = "Get Jira Project issue type fields by Jira project key and issue type ID"
    )]
    GetIssueTypeFields,
    #[value(name = "list", help = "List Jira Projects")]
    List,
}

/// Available version command line arguments
/// version_act: VersionActionValues
///   Create, List, Update, Delete, Release, Archive
#[derive(Args, Debug)]
pub struct IssueArgs {
    #[arg(
        value_name = "assign|create|delete|get|transition|update",
        help_heading = "Jira Project issue management"
    )]
    pub issue_act: IssueActionValues,
    #[clap(
        long,
        short = 'p',
        value_name = "project_key",
        help = "Jira Project key",
        required = true
    )]
    pub project_key: String,
    #[clap(
        long,
        short = 'i',
        value_name = "issue_key",
        help = "Jira Project issue key",
        required = true
    )]
    pub issue_key: Option<String>,
    #[clap(long,
        short = 'f',
        value_name = "issue_fields",
        value_parser = parse_key_val::<String, String>,
        help = "Jira Project issue fields (field_name=value)")]
    pub issue_fields: Option<Vec<(String, String)>>,
    #[clap(
        long,
        short = 't',
        value_name = "transition_to",
        help = "Jira Project issue transition to"
    )]
    pub transition_to: Option<String>,
    #[clap(
        long,
        short = 'a',
        value_name = "assignee",
        help = "Jira Project issue assignee"
    )]
    pub assignee: Option<String>,
    #[clap(flatten)]
    pub pagination: PaginationArgs,
    #[clap(flatten)]
    pub output: OutputArgs,
}

/// Available issue action values
/// Create, List, Update, Delete, Release, Archive
#[derive(ValueEnum, Debug, Clone, Copy)]
#[value(rename_all = "kebab-case")]
pub enum IssueActionValues {
    #[value(name = "assign", help = "Assign a Jira Project issue")]
    Assign,
    #[value(name = "create", help = "Create a Jira Project issue")]
    Create,
    #[value(name = "delete", help = "Delete a Jira Project issue")]
    Delete,
    #[value(name = "get", help = "Get a specific Jira Project issue")]
    Get,
    #[value(name = "transition", help = "Transition a Jira Project issue")]
    Transition,
    #[value(name = "update", help = "Update a Jira Project issue")]
    Update,
}

#[derive(Args, Debug)]
pub struct TransitionArgs {
    #[arg(value_name = "list", help_heading = "Jira issue transition list")]
    pub transition_act: TransitionActionValues,
    #[clap(
        long,
        short = 'i',
        value_name = "issue_key",
        help = "Jira Project issue key",
        required = true
    )]
    pub issue_key: String,
    #[clap(flatten)]
    pub output: OutputArgs,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
#[value(rename_all = "kebab-case")]
pub enum TransitionActionValues {
    #[value(name = "list", help = "List Jira issue available transitions")]
    List,
}

/// Parse a single key-value pair
/// Thanks to the example from the clap documentation (https://github.com/clap-rs/clap/blob/master/examples/typed-derive.rs)
fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: Error + Send + Sync + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}
