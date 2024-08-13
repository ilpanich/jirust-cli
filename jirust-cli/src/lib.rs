#[macro_use]
extern crate prettytable;

use crate::args::commands::{Commands, JirustCliArgs};

use crate::executors::jira_commands_executors::jira_version_executor::VersionExecutor;
use clap::Parser;
use config::config_file::ConfigFile;
use executors::config_executor::ConfigExecutor;
use executors::jira_commands_executors::ExecJiraCommand;
use std::env::Args;
use std::io::{Error, ErrorKind};

pub mod args;
pub mod config;
pub mod executors;
pub mod runners;

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
            ))
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

pub async fn process_command(
    command: Commands,
    config_file_path: String,
    cfg_data: ConfigFile,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Commands::Config(args) => {
            let config_executor = ConfigExecutor::new(config_file_path, args.cfg_act);
            config_executor.exec_config_command(cfg_data).await?
        }
        Commands::Version(args) => {
            let version_executor = VersionExecutor::new(cfg_data, args.version_act, args);
            version_executor.exec_jira_command().await?
        }
    }
    Ok(())
}
