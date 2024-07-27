use std::env;

use crate::args::commands::{Commands, JirustCliArgs};
use crate::runners::cfg_cmd_runner::ConfigCmdRunner;

use args::commands::ConfigArgValues;
use clap::Parser;
use config::config_file::ConfigFile;

pub mod args;
pub mod config;
pub mod runners;

fn main() {
    let config_file_path = match env::var_os("HOME") {
        Some(home) => format!("{}/.jirust-cli/jirust-cli.conf", home.to_string_lossy()),
        None => ".jirust-cli/jirust-cli.conf".to_string(),
    };
    let config_cmd_runner = ConfigCmdRunner::new(config_file_path.clone());
    let mut opts = match JirustCliArgs::try_parse_from(std::env::args()) {
        Ok(opts) => opts,
        Err(err) => {
            eprintln!("Error: {}", err);
            err.exit();
        }
    };
    let cfg_data = match ConfigFile::read_from_file(config_file_path.as_str()) {
        Ok(cfg) => cfg,
        Err(_) => {
            eprintln!("Error: Missing config file, setup mandatory!");
            opts.subcmd = Commands::Config(args::commands::ConfigArgs {
                cfg_arg: ConfigArgValues::Setup,
            });
            ConfigFile::default()
        }
    };
    if (cfg_data.get_auth_key().is_empty() || cfg_data.get_jira_url().is_empty()) {
        eprintln!("Error: Missing basic configuration, setup mandatory!");
        opts.subcmd = Commands::Config(args::commands::ConfigArgs {
            cfg_arg: ConfigArgValues::Setup,
        });
    }
    match opts.subcmd {
        Commands::Config(arg) => match arg.cfg_arg {
            cfg_arg => match cfg_arg {
                ConfigArgValues::Auth => {
                    let _cfg_res = match config_cmd_runner.set_cfg_auth(cfg_data) {
                        Ok(_) => println!("Authentication configuration stored successfully"),
                        Err(err) => {
                            eprintln!("Error storing authentication configuration: {}", err);
                        }
                    };
                }
                ConfigArgValues::Jira => {
                    let _cfg_res = match config_cmd_runner.set_cfg_jira(cfg_data) {
                        Ok(_) => println!("Initialization configuration stored successfully"),
                        Err(err) => {
                            eprintln!("Error storing initialization configuration: {}", err);
                        }
                    };
                }
                ConfigArgValues::Setup => {
                    let _cfg_res = match config_cmd_runner.setup_cfg(cfg_data) {
                        Ok(_) => println!("Configuration setup successfully"),
                        Err(err) => {
                            eprintln!("Error setting up configuration: {}", err);
                        }
                    };
                }
                ConfigArgValues::Show => {
                    config_cmd_runner.show_cfg(cfg_data);
                }
            },
        },
    }
}
