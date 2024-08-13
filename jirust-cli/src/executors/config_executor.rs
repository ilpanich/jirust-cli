use crate::args::commands::ConfigActionValues;
use crate::config::config_file::ConfigFile;
use crate::runners::cfg_cmd_runner::ConfigCmdRunner;

/// ConfigExecutor struct
pub struct ConfigExecutor {
    config_cmd_runner: ConfigCmdRunner,
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
    /// let config_executor = ConfigExecutor::new(config_file_path, args.cfg_act);
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
    /// ```
    /// config_executor.exec_config_command(cfg_data).await?;
    /// ```
    pub async fn exec_config_command(
        &self,
        cfg_data: ConfigFile,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self.config_action {
            ConfigActionValues::Auth => {
                let _cfg_res = match self.config_cmd_runner.set_cfg_auth(cfg_data) {
                    Ok(_) => println!("Authentication configuration stored successfully"),
                    Err(err) => {
                        eprintln!("Error storing authentication configuration: {}", err);
                    }
                };
            }
            ConfigActionValues::Jira => {
                let _cfg_res = match self.config_cmd_runner.set_cfg_jira(cfg_data) {
                    Ok(_) => println!("Initialization configuration stored successfully"),
                    Err(err) => {
                        eprintln!("Error storing initialization configuration: {}", err);
                    }
                };
            }
            ConfigActionValues::Setup => {
                let _cfg_res = match self.config_cmd_runner.setup_cfg(cfg_data) {
                    Ok(_) => println!("Configuration setup successfully"),
                    Err(err) => {
                        eprintln!("Error setting up configuration: {}", err);
                    }
                };
            }
            ConfigActionValues::Show => {
                self.config_cmd_runner.show_cfg(cfg_data);
            }
        }
        Ok(())
    }
}
