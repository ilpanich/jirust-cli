use crate::config::config_file::ConfigFile;
use crate::runners::cfg_cmd_runner::ConfigCmdRunner;
use crate::{args::commands::ConfigActionValues, utils::PrintableData};

use std::io::Error;

/// ConfigExecutor struct
///
/// # Fields
///
/// * `config_cmd_runner: ConfigCmdRunner` - configuration command runner
/// * `config_action: ConfigActionValues` - configuration action
pub struct ConfigExecutor {
    /// Configuration command runner
    config_cmd_runner: ConfigCmdRunner,
    /// Configuration action
    config_action: ConfigActionValues,
}

/// ConfigExecutor implementation
///
/// # Methods
///
/// * `new(cfg_file: String, config_action: ConfigActionValues) -> Self` - returns a new ConfigExecutor instance
/// * `exec_config_command(cfg_data: ConfigFile) -> Result<(), Box<dyn std::error::Error>>` - executes the configuration command
impl ConfigExecutor {
    /// Returns a new ConfigExecutor instance
    ///
    /// # Arguments
    ///
    /// * `cfg_file: String` - configuration file path
    /// * `config_action: ConfigActionValues` - configuration action
    ///
    /// # Returns
    ///
    /// * `Self` - a new ConfigExecutor instance
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::executors::config_executor::ConfigExecutor;
    /// use jirust_cli::args::commands::ConfigArgs;
    /// use jirust_cli::args::commands::ConfigActionValues;
    ///
    /// let args = ConfigArgs {
    ///    cfg_act: ConfigActionValues::Setup,
    /// };
    ///
    /// let config_executor = ConfigExecutor::new("config_file_path".to_string(), args.cfg_act);
    /// ```
    pub fn new(cfg_file: String, config_action: ConfigActionValues) -> Self {
        let config_cmd_runner = ConfigCmdRunner::new(cfg_file.clone());
        Self {
            config_cmd_runner,
            config_action,
        }
    }

    /// Executes the selected configuration command
    ///
    /// # Arguments
    ///
    /// * `cfg_data: ConfigFile` - configuration file data
    ///
    /// # Returns
    ///
    /// * `Result<(), Box<dyn std::error::Error>>` - Result with the execution status
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jirust_cli::executors::config_executor::ConfigExecutor;
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use jirust_cli::args::commands::ConfigArgs;
    /// use jirust_cli::args::commands::ConfigActionValues;
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # tokio_test::block_on(async {
    /// let args = ConfigArgs {
    ///    cfg_act: ConfigActionValues::Setup,
    /// };
    /// let cfg_data = ConfigFile::default();
    /// let config_executor = ConfigExecutor::new("config_file_path".to_string(), args.cfg_act);
    ///
    ///
    /// config_executor.exec_config_command(cfg_data).await?;
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    pub async fn exec_config_command(
        &self,
        cfg_data: ConfigFile,
    ) -> Result<Vec<PrintableData>, Box<dyn std::error::Error>> {
        match self.config_action {
            ConfigActionValues::Auth => match self.config_cmd_runner.set_cfg_auth(cfg_data) {
                Ok(_) => Ok(vec![PrintableData::Generic {
                    data: vec![serde_json::Value::String(
                        "Authentication configuration stored successfully".to_string(),
                    )],
                }]),
                Err(err) => Err(Box::new(Error::other(format!(
                    "Error storing authentication configuration: {}",
                    err
                )))),
            },
            ConfigActionValues::Jira => match self.config_cmd_runner.set_cfg_jira(cfg_data) {
                Ok(_) => Ok(vec![PrintableData::Generic {
                    data: vec![serde_json::Value::String(
                        "Initialization configuration stored successfully".to_string(),
                    )],
                }]),
                Err(err) => Err(Box::new(Error::other(format!(
                    "Error storing initialization configuration: {}",
                    err
                )))),
            },
            ConfigActionValues::Setup => match self.config_cmd_runner.setup_cfg(cfg_data) {
                Ok(_) => Ok(vec![PrintableData::Generic {
                    data: vec![serde_json::Value::String(
                        "Configuration setup successfully".to_string(),
                    )],
                }]),
                Err(err) => Err(Box::new(Error::other(format!(
                    "Error setting up configuration: {}",
                    err
                )))),
            },
            ConfigActionValues::Show => {
                self.config_cmd_runner.show_cfg(cfg_data);
                Ok(vec![PrintableData::Generic {
                    data: vec![serde_json::Value::String("DONE!".to_string())],
                }])
            }
        }
    }
}
