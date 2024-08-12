use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct JirustCliArgs {
    #[clap(subcommand)]
    pub subcmd: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Config(ConfigArgs),
    Version(VersionArgs),
}

#[derive(Args, Debug)]
pub struct ConfigArgs {
    #[arg(value_name = "auth|jira|setup|show", help_heading = "Authentication")]
    pub cfg_act: ConfigActionValues,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
#[value(rename_all = "kebab-case")]
pub enum ConfigActionValues {
    Auth,
    Jira,
    Setup,
    Show,
}

#[derive(Args, Debug)]
pub struct VersionArgs {
    #[arg(
        value_name = "create|list|update|delete|release",
        help_heading = "Jira Project version management"
    )]
    pub version_act: VersionActionValues,
    #[clap(long)]
    pub project: String,
    #[clap(long)]
    pub project_id: Option<i64>,
    #[clap(long)]
    pub version_id: Option<String>,
    #[clap(long)]
    pub version_name: Option<String>,
    #[clap(long)]
    pub version_description: Option<String>,
    #[clap(long)]
    pub version_start_date: Option<String>,
    #[clap(long)]
    pub version_release_date: Option<String>,
    #[clap(long)]
    pub version_archived: Option<bool>,
    #[clap(long)]
    pub version_released: Option<bool>,
    #[clap(long)]
    pub version_page_size: Option<i32>,
    #[clap(long)]
    pub version_page_offset: Option<i64>,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
#[value(rename_all = "kebab-case")]
pub enum VersionActionValues {
    Create,
    List,
    Update,
    Delete,
    Release,
    Archive,
}
