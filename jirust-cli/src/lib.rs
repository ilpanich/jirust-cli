#[macro_use]
extern crate prettytable;

use crate::args::commands::{Commands, JirustCliArgs};

use crate::executors::jira_commands_executors::jira_version_executor::VersionExecutor;
use clap::Parser;
use config::config_file::ConfigFile;
use executors::config_executor::ConfigExecutor;
use executors::jira_commands_executors::ExecJiraCommand;
use executors::jira_commands_executors::jira_issue_executor::IssueExecutor;
use executors::jira_commands_executors::jira_issue_link_executor::LinkIssueExecutor;
use executors::jira_commands_executors::jira_issue_transition_executor::IssueTransitionExecutor;
use executors::jira_commands_executors::jira_project_executor::ProjectExecutor;
use std::env::Args;
use std::io::{Error, ErrorKind};
use utils::PrintableData;

pub mod args;
pub mod config;
pub mod executors;
pub mod runners;
pub mod utils;

/// Manages the loading of the CLI configuration
///
/// # Arguments
/// * `config_file_path` - The path to the configuration file
/// * `args` - The arguments passed to the CLI
///
/// # Returns
/// * A tuple containing the configuration file and the command to execute
///
/// # Errors
/// * If the configuration file is not found
/// * If the configuration file is missing mandatory fields
///
/// # Examples
///
/// ```no_run
/// use jirust_cli::manage_config;
/// use jirust_cli::config::config_file::ConfigFile;
/// use jirust_cli::args::commands::Commands;
///
/// # fn main() -> Result<(), std::io::Error> {
/// let config_file_path = String::from("config.json");
/// let args = std::env::args();
/// let (cfg_data, command) = manage_config(config_file_path, args)?;
/// # Ok(())
/// # }
/// ```
pub fn manage_config(
    config_file_path: String,
    args: Args,
) -> Result<(ConfigFile, Commands), Error> {
    let opts = match JirustCliArgs::try_parse_from(args) {
        Ok(opts) => opts,
        Err(err) => {
            eprintln!("Error: {}", err);
            err.exit();
        }
    };
    let cfg_data = match ConfigFile::read_from_file(config_file_path.as_str()) {
        Ok(cfg) => cfg,
        Err(_) => {
            return Err(Error::new(
                ErrorKind::NotFound,
                "Missing basic configuration, setup mandatory!",
            ));
        }
    };
    if cfg_data.get_auth_key().is_empty() || cfg_data.get_jira_url().is_empty() {
        Err(Error::new(
            ErrorKind::NotFound,
            "Missing basic configuration, setup mandatory!",
        ))
    } else {
        Ok((cfg_data, opts.subcmd))
    }
}

/// Processes the command passed to the CLI
///
/// # Arguments
/// * `command` - The command to execute
/// * `config_file_path` - The path to the configuration file (optional, for Jira commands but mandatory to setup config)
/// * `cfg_data` - The configuration file data
///
/// # Returns
/// * A Result containing the result of the command execution
///
/// # Errors
/// * If the command execution fails
///
/// # Examples
///
/// ```no_run
/// use jirust_cli::process_command;
/// use jirust_cli::config::config_file::ConfigFile;
/// use jirust_cli::args::commands::{Commands, VersionArgs, VersionActionValues, PaginationArgs, OutputArgs};
///
/// # fn main() -> Result<(), std::io::Error> {
/// let config_file_path = String::from("config.json");
/// let args = VersionArgs {
///   version_act: VersionActionValues::List,
///   project_key: "project_key".to_string(),
///   project_id: None,
///   version_id: Some("97531".to_string()),
///   version_name: Some("version_name".to_string()),
///   version_description: Some("version_description".to_string()),
///   version_start_date: None,
///   version_release_date: None,
///   version_archived: None,
///   version_released: Some(true),
///   changelog_file: None,
///   pagination: PaginationArgs { page_size: Some(20), page_offset: None },
///   output: OutputArgs { output_format: None, output_type: None },
///   transition_assignee: None,
///   transition_issues: None,
/// };
///
/// let result = process_command(Commands::Version(args), config_file_path, ConfigFile::default());
/// # Ok(())
/// # }
/// ```
pub async fn process_command(
    command: Commands,
    config_file_path: Option<String>,
    cfg_data: ConfigFile,
) -> Result<Vec<PrintableData>, Box<dyn std::error::Error>> {
    match command {
        Commands::Config(args) => match config_file_path {
            Some(path) => {
                let config_executor = ConfigExecutor::new(path, args.cfg_act);
                config_executor.exec_config_command(cfg_data).await
            }
            None => Err(Box::new(Error::new(
                ErrorKind::NotFound,
                "Missing config file path!",
            ))),
        },
        Commands::Version(args) => {
            let version_executor = VersionExecutor::new(cfg_data, args.version_act, args);
            version_executor.exec_jira_command().await
        }
        Commands::Project(args) => {
            let project_executor = ProjectExecutor::new(cfg_data, args.project_act, args);
            project_executor.exec_jira_command().await
        }
        Commands::Issue(args) => {
            let issue_executor = IssueExecutor::new(cfg_data, args.issue_act, args);
            issue_executor.exec_jira_command().await
        }
        Commands::Transition(args) => {
            let issue_transition_executor =
                IssueTransitionExecutor::new(cfg_data, args.transition_act, args);
            issue_transition_executor.exec_jira_command().await
        }
        Commands::Link(args) => {
            let link_issue_executor = LinkIssueExecutor::new(cfg_data, args.link_act, args);
            link_issue_executor.exec_jira_command().await
        }
    }
}
