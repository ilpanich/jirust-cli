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
}

#[derive(Args, Debug)]
pub struct ConfigArgs {
    #[arg(value_name = "auth|jira|setup|show", help_heading = "Authentication")]
    pub cfg_arg: ConfigArgValues,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
#[value(rename_all = "kebab-case")]
pub enum ConfigArgValues {
    Auth,
    Jira,
    Setup,
    Show,
}
