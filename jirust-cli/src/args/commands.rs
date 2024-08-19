use clap::{Args, Parser, Subcommand, ValueEnum};

/// Command line arguments base
/// subcommands: Config, Version
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct JirustCliArgs {
    #[clap(subcommand)]
    pub subcmd: Commands,
}

/// Available CLI commands
/// Config, Project, Version
#[derive(Subcommand, Debug)]
pub enum Commands {
    Config(ConfigArgs),
    Project(ProjectArgs),
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
        short = 'o',
        value_name = "page_offset",
        help = "page offset for list"
    )]
    pub page_offset: Option<i64>,
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
        value_name = "create|list|update|delete|release",
        help_heading = "Jira Project versions (a.k.a \"Releases\") management"
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
    #[clap(flatten)]
    pub pagination: PaginationArgs,
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
        help_heading = "Jira Project management"
    )]
    pub project_act: ProjectActionValues,
    #[clap(
        long,
        short = 'k',
        value_name = "project_key",
        help = "Jira Project key"
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
