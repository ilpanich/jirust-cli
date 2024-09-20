use crate::args::commands::{OutputValues, VersionActionValues, VersionArgs};
use crate::config::config_file::ConfigFile;
use crate::runners::jira_cmd_runners::version_cmd_runner::{VersionCmdParams, VersionCmdRunner};
use crate::utils::{print_data, OutputType, PrintableData};

use super::ExecJiraCommand;

/// VersionExecutor is responsible for executing the Jira version-related commands
///
/// # Fields
///
/// * `version_cmd_runner` - the runner responsible for running the Jira version-related commands
/// * `version_action` - the action to be performed on the Jira version
/// * `version_args` - the arguments passed to the Jira version command
pub struct VersionExecutor {
    /// version_cmd_runner is the runner responsible for running the Jira version-related commands
    version_cmd_runner: VersionCmdRunner,
    /// version_action is the action to be performed on the Jira version
    version_action: VersionActionValues,
    /// version_args are the arguments passed to the Jira version command
    version_args: VersionArgs,
}

/// VersionExecutor implementation
///
/// #Methods
///
/// * `new(cfg_data: ConfigFile, version_action: VersionActionValues, version_args: VersionArgs) -> Self` - creates a new VersionExecutor instance
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
    /// ```no_run
    /// use jirust_cli::executors::jira_commands_executors::jira_version_executor::VersionExecutor;
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use jirust_cli::args::commands::{VersionActionValues, VersionArgs, PaginationArgs, OutputArgs};
    ///
    /// let cfg_data = ConfigFile::default();
    /// let version_action = VersionActionValues::List;
    /// let version_args = VersionArgs {
    ///   version_act: version_action.clone(),
    ///   project_key: "project_key".to_string(),
    ///   project_id: None,
    ///   version_id: None,
    ///   version_name: None,
    ///   version_description: None,
    ///   version_start_date: None,
    ///   version_release_date: None,
    ///   version_archived: None,
    ///   version_released: None,
    ///   changelog_file: None,
    ///   pagination: PaginationArgs { page_size: Some(20), page_offset: None },
    ///   output: OutputArgs { output: None },
    ///   transition_assignee: None,
    ///   transition_issues: None,
    /// };
    ///
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

/// Implementation of the `ExecJiraCommand` trait for `VersionExecutor`
/// This trait is responsible for executing the Jira command
///
/// # Methods
///
/// * `exec_jira_command(&self) -> Result<(), Box<dyn std::error::Error>>` - executes the Jira command
impl ExecJiraCommand for VersionExecutor {
    /// Executes the Jira command
    ///
    /// # Returns
    ///
    /// * `Result<(), Box<dyn std::error::Error>>` - Result with the execution status
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jirust_cli::executors::jira_commands_executors::ExecJiraCommand;
    /// use jirust_cli::executors::jira_commands_executors::jira_version_executor::VersionExecutor;
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use jirust_cli::args::commands::{VersionArgs, VersionActionValues, PaginationArgs, OutputArgs};
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # tokio_test::block_on(async {
    /// let version_action = VersionActionValues::List;
    /// let version_args = VersionArgs {
    ///   version_act: version_action.clone(),
    ///   project_key: "project_key".to_string(),
    ///   project_id: None,
    ///   version_id: None,
    ///   version_name: None,
    ///   version_description: None,
    ///   version_start_date: None,
    ///   version_release_date: None,
    ///   version_archived: None,
    ///   version_released: None,
    ///   changelog_file: None,
    ///   pagination: PaginationArgs { page_size: Some(20), page_offset: None },
    ///   output: OutputArgs { output: None },
    ///   transition_assignee: None,
    ///   transition_issues: None,
    /// };
    ///
    /// let cfg_data = ConfigFile::default();
    /// let version_executor = VersionExecutor::new(cfg_data, version_action, version_args);
    ///
    /// version_executor.exec_jira_command().await?;
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    async fn exec_jira_command(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self.version_action {
            VersionActionValues::Create => {
                let res = self
                    .version_cmd_runner
                    .create_jira_version(VersionCmdParams::from(&self.version_args))
                    .await?;
                print_data(
                    PrintableData::Version {
                        versions: vec![res.0],
                    },
                    self.version_args
                        .output
                        .output
                        .unwrap_or(OutputValues::Json),
                    OutputType::Single,
                );
                if Option::is_some(&res.1) {
                    print_data(
                        PrintableData::TransitionedIssue {
                            issues: res.1.unwrap_or(vec![]),
                        },
                        self.version_args
                            .output
                            .output
                            .unwrap_or(OutputValues::Json),
                        OutputType::Full,
                    );
                }
            }
            VersionActionValues::List => {
                let res = self
                    .version_cmd_runner
                    .list_jira_versions(VersionCmdParams::from(&self.version_args))
                    .await?;
                print_data(
                    PrintableData::Version { versions: res },
                    self.version_args
                        .output
                        .output
                        .unwrap_or(OutputValues::Json),
                    OutputType::Full,
                );
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
                                print_data(
                                    PrintableData::Version {
                                        versions: vec![res],
                                    },
                                    self.version_args
                                        .output
                                        .output
                                        .unwrap_or(OutputValues::Json),
                                    OutputType::Single,
                                );
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
                                print_data(
                                    PrintableData::Version {
                                        versions: vec![res],
                                    },
                                    self.version_args
                                        .output
                                        .output
                                        .unwrap_or(OutputValues::Json),
                                    OutputType::Single,
                                );
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
                                print_data(
                                    PrintableData::Version {
                                        versions: vec![res],
                                    },
                                    self.version_args
                                        .output
                                        .output
                                        .unwrap_or(OutputValues::Json),
                                    OutputType::Single,
                                );
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
