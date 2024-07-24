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
    config_cmd_runner
        .init_file()
        .expect("Cannot init config file");
    let opts = match JirustCliArgs::try_parse_from(std::env::args()) {
        Ok(opts) => opts,
        Err(err) => {
            eprintln!("Error: {}", err);
            err.exit();
        }
    };
    match opts.subcmd {
        Commands::Config(arg) => match arg.cfg_arg {
            cfg_arg => {
                let cfg_data = ConfigFile::read_from_file(config_file_path.as_str())
                    .unwrap_or(ConfigFile::new("".to_string(), "".to_string()));
                match cfg_arg {
                    ConfigArgValues::Auth => {
                        let cfg_res = match config_cmd_runner.set_cfg_auth(cfg_data) {
                            Ok(_) => println!("Authentication configuration stored successfully"),
                            Err(err) => {
                                eprintln!("Error storing authentication configuration: {}", err);
                            }
                        };
                    }
                    ConfigArgValues::Init => {
                        let cfg_res = match config_cmd_runner.set_cfg_init(cfg_data) {
                            Ok(_) => println!("Initialization configuration stored successfully"),
                            Err(err) => {
                                eprintln!("Error storing initialization configuration: {}", err);
                            }
                        };
                    }
                    _ => {
                        eprintln!("Invalid configuration argument provided");
                    }
                }
            }
        },
    }
}
