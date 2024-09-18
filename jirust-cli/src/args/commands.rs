use clap::{ArgAction, Args, Parser, Subcommand, ValueEnum};
use std::error::Error;

/// Command line arguments base
/// subcommands: Config, Version
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct JirustCliArgs {
    /// Subcommands
    #[clap(subcommand)]
    pub subcmd: Commands,
}

/// Available CLI commands
/// Config, Issue, Project, Transition, Version
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Configuration management
    Config(ConfigArgs),
    /// Issue management
    Issue(IssueArgs),
    /// Project management
    Project(ProjectArgs),
    /// Transition management
    Transition(TransitionArgs),
    /// Version management
    Version(VersionArgs),
}

/// Available pagination command line arguments
///
/// * page_size: Option<i32>
/// * page_offset: Option<i64>
#[derive(Args, Debug)]
pub struct PaginationArgs {
    /// page size for lists
    #[clap(
        long,
        short = 'l',
        value_name = "page_size",
        help = "page size for lists"
    )]
    pub page_size: Option<i32>,
    /// page offset for list
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
    /// Print output in table format
    #[value(name = "table", help = "Print output in table format")]
    Table,
    /// Print output in json format
    #[value(name = "json", help = "Print output in json format")]
    Json,
}

/// Available output values
///
/// * Table: Print output in table format
/// * Json: Print output in json format
#[derive(Args, Debug)]
pub struct OutputArgs {
    /// Output format
    #[clap(long, short = 'o', value_name = "table|json", help = "Output format")]
    pub output: Option<OutputValues>,
}

/// Available configuration command line arguments
/// cfg_act: ConfigActionValues
///    Auth, Jira, Setup, Show
///
#[derive(Args, Debug)]
pub struct ConfigArgs {
    /// Configuration action
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
    /// Set Jira API authentication (username, apikey)
    #[value(name = "auth", help = "Set Jira API authentication (username, apikey)")]
    Auth,
    /// Set Jira API base URL
    #[value(name = "jira", help = "Set Jira API base URL")]
    Jira,
    /// Setup Jira API configuration (authentication data, jira base URL, etc.)
    #[value(
        name = "setup",
        help = "Setup Jira API configuration (authentication data, jira base URL, etc.)"
    )]
    Setup,
    /// Show current configuration
    #[value(name = "show", help = "Show current configuration")]
    Show,
}

/// Available version command line arguments
/// version_act: VersionActionValues
///   Create, List, Update, Delete, Release, Archive
#[derive(Args, Debug)]
pub struct VersionArgs {
    /// Version action
    #[arg(
        value_name = "archive|create|delete|list|release|update",
        help_heading = "Jira Project version management"
    )]
    pub version_act: VersionActionValues,
    /// Jira Project key
    #[clap(
        long,
        short = 'k',
        value_name = "project_key",
        required = true,
        help = "Jira Project key"
    )]
    pub project_key: String,
    /// Jira Project ID
    #[clap(long, short = 'i', value_name = "project_id", help = "Jira Project ID")]
    pub project_id: Option<i64>,
    /// Jira Project version ID
    #[clap(
        long,
        short = 'v',
        value_name = "version_id",
        help = "Jira Project version ID"
    )]
    pub version_id: Option<String>,
    /// Jira Project version name
    #[clap(
        long,
        short = 'n',
        value_name = "version_name",
        help = "Jira Project version name"
    )]
    pub version_name: Option<String>,
    /// Jira Project version description
    #[clap(
        long,
        short = 'd',
        value_name = "version_description",
        help = "Jira Project version description"
    )]
    pub version_description: Option<String>,
    /// Jira Project version start date
    #[clap(
        long,
        value_name = "version_start_date",
        help = "Jira Project version start date"
    )]
    pub version_start_date: Option<String>,
    /// Jira Project version release date
    #[clap(
        long,
        value_name = "version_release_date",
        help = "Jira Project version release date"
    )]
    pub version_release_date: Option<String>,
    /// Jira Project version archived
    #[clap(
        long,
        short = 'a',
        value_name = "version_archived",
        help = "Jira Project version archived"
    )]
    pub version_archived: Option<bool>,
    /// Jira Project version released
    #[clap(
        long,
        short = 'r',
        value_name = "version_released",
        help = "Jira Project version released"
    )]
    pub version_released: Option<bool>,
    /// Jira Project version changelog file
    #[clap(
        long,
        short = 'c',
        value_name = "changelog_file",
        help = "changelog file path to be used for automatic description generation (if set the script detects automatically the first tagged block in the changelog and use it as description)"
    )]
    pub changelog_file: Option<String>,
    /// Jira Project version automatically transition issues in changelog
    #[clap(
        long,
        short = 't',
        action = ArgAction::SetTrue,
        value_name = "transition_issues",
        help = "if changelog is set and this flag is set, the script will transition all issues in the changelog of the current version release"
    )]
    pub transition_issues: Option<bool>,
    /// Jira Project version transition assignee
    #[clap(
        long,
        short = 'u',
        value_name = "transition_assignee",
        help = "if changelog is set and the resolve_issues flag is set, the script will transition all issues in the changelog of the current version release"
    )]
    pub transition_assignee: Option<String>,
    /// Jira Project version pagination
    #[clap(flatten)]
    pub pagination: PaginationArgs,
    /// Jira Project version actions result output format
    #[clap(flatten)]
    pub output: OutputArgs,
}

/// Available version action values
/// Archive, Create, Delete, List, Release, Update
#[derive(ValueEnum, Debug, Clone, Copy)]
#[value(rename_all = "kebab-case")]
pub enum VersionActionValues {
    /// Archive a Jira Project version
    #[value(name = "archive", help = "Archive a Jira Project version")]
    Archive,
    /// Create a Jira Project version
    #[value(name = "create", help = "Create a Jira Project version")]
    Create,
    /// Delete a Jira Project version
    #[value(name = "delete", help = "Delete a Jira Project version")]
    Delete,
    /// List Jira Project versions
    #[value(name = "list", help = "List Jira Project versions")]
    List,
    /// Release a Jira Project version
    #[value(name = "release", help = "Release a Jira Project version")]
    Release,
    /// Update a Jira Project version
    #[value(name = "update", help = "Update a Jira Project version")]
    Update,
}

/// Available project command line arguments
/// version_act: ProjectActionValues
/// GetIssueTypes, GetIssueTypeFields, List
#[derive(Args, Debug)]
pub struct ProjectArgs {
    /// Project action
    #[arg(
        value_name = "get-issue-types|get-issue-type-fields|list",
        help_heading = "Jira Project management",
        required = true
    )]
    pub project_act: ProjectActionValues,
    /// Jira Project key
    #[clap(
        long,
        short = 'k',
        value_name = "project_key",
        help = "Jira Project key",
        required = true
    )]
    pub project_key: Option<String>,
    /// Jira Project issue type ID
    #[clap(
        long,
        short = 'i',
        value_name = "project_issue_type",
        help = "Jira Project issue type ID"
    )]
    pub project_issue_type: Option<String>,
    /// Jira Project pagination
    #[clap(flatten)]
    pub pagination: PaginationArgs,
    /// Jira Project actions result output format
    #[clap(flatten)]
    pub output: OutputArgs,
}

/// Available project action values
/// GetIssueTypes, GetIssueTypeFields, List
#[derive(ValueEnum, Debug, Clone, Copy)]
#[value(rename_all = "kebab-case")]
pub enum ProjectActionValues {
    /// Get Jira Project issue types by Jira project key
    #[value(
        name = "get-issue-types",
        help = "Get Jira Project issue types by Jira project key"
    )]
    GetIssueTypes,
    /// Get Jira Project issue type fields by Jira project key and issue type ID
    #[value(
        name = "get-issue-type-fields",
        help = "Get Jira Project issue type fields by Jira project key and issue type ID"
    )]
    GetIssueTypeFields,
    /// List Jira Projects
    #[value(name = "list", help = "List Jira Projects")]
    List,
}

/// Available version command line arguments
/// version_act: VersionActionValues
///   Create, List, Update, Delete, Release, Archive
#[derive(Args, Debug)]
pub struct IssueArgs {
    /// Issue action
    #[arg(
        value_name = "assign|create|delete|get|transition|update",
        help_heading = "Jira Project issue management"
    )]
    pub issue_act: IssueActionValues,
    /// Jira Project key
    #[clap(
        long,
        short = 'p',
        value_name = "project_key",
        help = "Jira Project key",
        required = true
    )]
    pub project_key: String,
    /// Jira Project issue key
    #[clap(
        long,
        short = 'i',
        value_name = "issue_key",
        help = "Jira Project issue key",
        required = true
    )]
    pub issue_key: Option<String>,
    /// Jira Project issue fields
    #[clap(long,
        short = 'f',
        value_name = "issue_fields",
        value_parser = parse_key_val::<String, String>,
        help = "Jira Project issue fields (field_name=value)")]
    pub issue_fields: Option<Vec<(String, String)>>,
    /// Jira Project issue transition
    #[clap(
        long,
        short = 't',
        value_name = "transition_to",
        help = "Jira Project issue transition to"
    )]
    pub transition_to: Option<String>,
    /// Jira Project issue assignee
    #[clap(
        long,
        short = 'a',
        value_name = "assignee",
        help = "Jira Project issue assignee"
    )]
    pub assignee: Option<String>,
    /// Jira Project issue pagination
    #[clap(flatten)]
    pub pagination: PaginationArgs,
    /// Jira Project issue actions result output format
    #[clap(flatten)]
    pub output: OutputArgs,
}

/// Available issue action values
/// Create, List, Update, Delete, Release, Archive
#[derive(ValueEnum, Debug, Clone, Copy)]
#[value(rename_all = "kebab-case")]
pub enum IssueActionValues {
    /// Assign a Jira Project issue
    #[value(name = "assign", help = "Assign a Jira Project issue")]
    Assign,
    /// Create a Jira Project issue
    #[value(name = "create", help = "Create a Jira Project issue")]
    Create,
    /// Delete a Jira Project issue
    #[value(name = "delete", help = "Delete a Jira Project issue")]
    Delete,
    /// Get a specific Jira Project issue
    #[value(name = "get", help = "Get a specific Jira Project issue")]
    Get,
    /// Transition a Jira Project issue
    #[value(name = "transition", help = "Transition a Jira Project issue")]
    Transition,
    /// Update a Jira Project issue
    #[value(name = "update", help = "Update a Jira Project issue")]
    Update,
}

/// Available transition command line arguments
/// transition_act: TransitionActionValues
/// List
#[derive(Args, Debug)]
pub struct TransitionArgs {
    /// Transition action
    #[arg(value_name = "list", help_heading = "Jira issue transition list")]
    pub transition_act: TransitionActionValues,
    /// Jira issue key
    #[clap(
        long,
        short = 'i',
        value_name = "issue_key",
        help = "Jira Project issue key",
        required = true
    )]
    pub issue_key: String,
    /// Jira issue output format
    #[clap(flatten)]
    pub output: OutputArgs,
}

/// Available transition action values
/// List
#[derive(ValueEnum, Debug, Clone, Copy)]
#[value(rename_all = "kebab-case")]
pub enum TransitionActionValues {
    /// List Jira issue available transitions
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
