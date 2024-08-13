use crate::args::commands::{VersionActionValues, VersionArgs};
use crate::config::config_file::ConfigFile;
use crate::runners::jira_cmd_runners::version_cmd_runner::{
    print_table_full, print_table_single, VersionCmdParams, VersionCmdRunner,
};

use super::ExecJiraCommand;

/// VersionExecutor struct
pub struct VersionExecutor {
    version_cmd_runner: VersionCmdRunner,
    version_action: VersionActionValues,
    version_args: VersionArgs,
}

/// VersionExecutor implementation
///
/// #Methods
///
/// * `nef(cfg_data: ConfigFile, version_action: VersionActionValues, version_args: VersionArgs) -> Self` - creates a new VersionExecutor instance
/// * `exec_jira_command(&self) -> Result<(), Box<dyn std::error::Error>>` - executes the Jira command
impl VersionExecutor {
    /// Creates a new VersionExecutor instance
    ///
    /// # Arguments
    ///
    /// * `cfg_data: ConfigFile` - configuration file data
    /// * `version_action: VersionActionValues` - version action
    /// * `version_args: VersionArgs` - version arguments
    ///
    /// # Returns
    ///
    /// * `Self` - a new VersionExecutor instance
    ///
    /// # Examples
    ///
    /// ```
    /// let version_executor = VersionExecutor::new(cfg_data, version_action, version_args);
    /// ```
    pub fn new(
        cfg_data: ConfigFile,
        version_action: VersionActionValues,
        version_args: VersionArgs,
    ) -> Self {
        let version_cmd_runner = VersionCmdRunner::new(cfg_data.clone());
        Self {
            version_cmd_runner,
            version_action,
            version_args,
        }
    }
}

/// Implements the `ExecJiraCommand` trait for `VersionExecutor`
impl ExecJiraCommand for VersionExecutor {
    /// Executes the Jira command
    ///
    /// # Returns
    ///
    /// * `Result<(), Box<dyn std::error::Error>>` - Result with the execution status
    ///
    /// # Examples
    ///
    /// ```
    /// version_executor.exec_jira_command().await?;
    /// ```
    async fn exec_jira_command(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self.version_action {
            VersionActionValues::Create => {
                let res = self
                    .version_cmd_runner
                    .create_jira_version(VersionCmdParams::from(&self.version_args))
                    .await?;
                print_table_single(res);
            }
            VersionActionValues::List => {
                let res = self
                    .version_cmd_runner
                    .list_jira_versions(VersionCmdParams::from(&self.version_args))
                    .await?;
                print_table_full(res);
            }
            VersionActionValues::Update => {
                match self
                    .version_cmd_runner
                    .get_jira_version(VersionCmdParams::from(&self.version_args))
                    .await
                {
                    Ok(res_version) => {
                        let res = self
                            .version_cmd_runner
                            .update_jira_version(VersionCmdParams::merge_args(
                                res_version,
                                Some(&self.version_args),
                            ))
                            .await;
                        match res {
                            Ok(res) => {
                                println!("Version updated successfully");
                                print_table_single(res);
                            }
                            Err(err) => eprintln!("Error updating version: {}", err),
                        }
                    }
                    Err(err) => eprintln!("Error retrieving version: {}", err),
                }
            }
            VersionActionValues::Delete => {
                let res = self
                    .version_cmd_runner
                    .delete_jira_version(VersionCmdParams::from(&self.version_args))
                    .await;
                match res {
                    Ok(_) => println!("Version deleted successfully"),
                    Err(err) => eprintln!("Error deleting version: {}", err),
                }
            }
            VersionActionValues::Release => {
                match self
                    .version_cmd_runner
                    .get_jira_version(VersionCmdParams::from(&self.version_args))
                    .await
                {
                    Ok(res_version) => {
                        let res = self
                            .version_cmd_runner
                            .update_jira_version(VersionCmdParams::mark_released(res_version))
                            .await;
                        match res {
                            Ok(res) => {
                                println!("Version released successfully");
                                print_table_single(res);
                            }
                            Err(err) => eprintln!("Error releasing version: {}", err),
                        }
                    }
                    Err(err) => eprintln!("Error retrieving version: {}", err),
                }
            }
            VersionActionValues::Archive => {
                match self
                    .version_cmd_runner
                    .get_jira_version(VersionCmdParams::from(&self.version_args))
                    .await
                {
                    Ok(res_version) => {
                        let res = self
                            .version_cmd_runner
                            .update_jira_version(VersionCmdParams::mark_archived(res_version))
                            .await;
                        match res {
                            Ok(res) => {
                                println!("Version archived successfully");
                                print_table_single(res);
                            }
                            Err(err) => eprintln!("Error archiving version: {}", err),
                        }
                    }
                    Err(err) => eprintln!("Error retrieving version: {}", err),
                }
            }
        }
        Ok(())
    }
}