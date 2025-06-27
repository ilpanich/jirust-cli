use std::io::Error;

use crate::args::commands::{VersionActionValues, VersionArgs};
use crate::config::config_file::ConfigFile;
use crate::runners::jira_cmd_runners::version_cmd_runner::{VersionCmdParams, VersionCmdRunner};
use crate::utils::PrintableData;

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
    ///   output: OutputArgs { output_format: None, output_type: None },
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
        let version_cmd_runner = VersionCmdRunner::new(&cfg_data);
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
    ///   output: OutputArgs { output_format: None, output_type: None },
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
    async fn exec_jira_command(&self) -> Result<Vec<PrintableData>, Box<dyn std::error::Error>> {
        let result: Result<Vec<PrintableData>, Box<dyn std::error::Error>>;
        match self.version_action {
            VersionActionValues::Create => {
                result = match self
                    .version_cmd_runner
                    .create_jira_version(VersionCmdParams::from(&self.version_args))
                    .await
                {
                    Ok((version, transitions)) => {
                        let mut res = Vec::new();
                        res.push(PrintableData::Version {
                            versions: vec![version],
                        });
                        if Option::is_some(&transitions) {
                            res.push(PrintableData::TransitionedIssue {
                                issues: transitions.unwrap_or(vec![]),
                            });
                        }
                        Ok(res)
                    }
                    Err(err) => Err(Box::new(Error::other(format!(
                        "Error creating version: {err}"
                    )))),
                };
            }
            VersionActionValues::List => {
                result = match self
                    .version_cmd_runner
                    .list_jira_versions(VersionCmdParams::from(&self.version_args))
                    .await
                {
                    Ok(version) => Ok(vec![PrintableData::Version { versions: version }]),
                    Err(err) => Err(Box::new(Error::other(format!(
                        "Error listing versions: {err}"
                    )))),
                }
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
                                result = Ok(vec![PrintableData::Version {
                                    versions: vec![res],
                                }]);
                            }
                            Err(err) => {
                                result = Err(Box::new(Error::other(format!(
                                    "Error updating version: {err}"
                                ))))
                            }
                        }
                    }
                    Err(err) => {
                        result = Err(Box::new(Error::other(format!(
                            "Error retrieving version: {err}"
                        ))))
                    }
                }
            }
            VersionActionValues::Delete => {
                result = match self
                    .version_cmd_runner
                    .delete_jira_version(VersionCmdParams::from(&self.version_args))
                    .await
                {
                    Ok(_) => Ok(vec![PrintableData::Generic {
                        data: vec![serde_json::Value::String(
                            "Version deleted successfully".to_string(),
                        )],
                    }]),
                    Err(err) => Err(Box::new(Error::other(format!(
                        "Error deleting version: {err}"
                    )))),
                }
            }
            VersionActionValues::RelatedWorkItems => {
                result = match self
                    .version_cmd_runner
                    .get_jira_version_related_work(VersionCmdParams::from(&self.version_args))
                    .await
                {
                    Ok(items) => Ok(vec![PrintableData::VersionRelatedWork {
                        version_related_work_items: items,
                    }]),
                    Err(err) => Err(Box::new(Error::other(format!(
                        "Error listing version Related Workitems: {err}"
                    )))),
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
                                result = Ok(vec![PrintableData::Version {
                                    versions: vec![res],
                                }]);
                            }
                            Err(err) => {
                                result = Err(Box::new(Error::other(format!(
                                    "Error releasing version: {err}"
                                ))))
                            }
                        }
                    }
                    Err(err) => {
                        result = Err(Box::new(Error::other(format!(
                            "Error retrieving version: {err}"
                        ))))
                    }
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
                                result = Ok(vec![PrintableData::Version {
                                    versions: vec![res],
                                }]);
                            }
                            Err(err) => {
                                result = Err(Box::new(Error::other(format!(
                                    "Error archiving version: {err}"
                                ))))
                            }
                        }
                    }
                    Err(err) => {
                        result = Err(Box::new(Error::other(format!(
                            "Error retrieving version: {err}"
                        ))))
                    }
                }
            }
        }
        result
    }
}
