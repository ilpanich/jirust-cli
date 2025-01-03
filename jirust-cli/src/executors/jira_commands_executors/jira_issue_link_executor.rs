use crate::args::commands::{LinkIssueActionValues, LinkIssueArgs, OutputValues};
use crate::config::config_file::ConfigFile;
use crate::runners::jira_cmd_runners::link_issue_cmd_runner::{
    LinkIssueCmdParams, LinkIssueCmdRunner,
};
use crate::utils::{print_data, OutputType, PrintableData};

use super::ExecJiraCommand;

/// LinkIssueExecutor is responsible for executing the Jira link-issue-related commands
///
/// # Fields
///
/// * `link_issue_cmd_runner` - the runner responsible for running the Jira link-issue-related commands
/// * `link_issue_action` - the action to be performed on the Jira link issue
/// * `link_issue_args` - the arguments passed to the Jira link issue command
pub struct LinkIssueExecutor {
    /// link_issue_cmd_runner is the runner responsible for running the Jira version-related commands
    link_issue_cmd_runner: LinkIssueCmdRunner,
    /// link_issue_action is the action to be performed on the Jira version
    link_issue_action: LinkIssueActionValues,
    /// link_issue_args are the arguments passed to the Jira version command
    link_issue_args: LinkIssueArgs,
}

/// LinkIssueExecutor implementation
///
/// #Methods
///
/// * `new(cfg_data: ConfigFile, version_action: VersionActionValues, version_args: VersionArgs) -> Self` - creates a new LinkIssueExecutor instance
/// * `exec_jira_command(&self) -> Result<(), Box<dyn std::error::Error>>` - executes the Jira command
impl LinkIssueExecutor {
    /// Creates a new LinkIssueExecutor instance
    ///
    /// # Arguments
    ///
    /// * `cfg_data: ConfigFile` - configuration file data
    /// * `link_issue_action` - the action to be performed on the Jira link issue
    /// * `link_issue_args` - the arguments passed to the Jira link issue command
    ///
    /// # Returns
    ///
    /// * `Self` - a new LinkIssueExecutor instance
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jirust_cli::executors::jira_commands_executors::jira_issue_link_executor::LinkIssueExecutor;
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use jirust_cli::args::commands::{LinkIssueActionValues, LinkIssueArgs};
    ///
    /// let cfg_data = ConfigFile::default();
    /// let link_issue_args = LinkIssueArgs {
    ///   link_act: LinkIssueActionValues::Create,
    ///   origin_issue_key: "ISSUE-123".to_string(),
    ///   destination_issue_key: Some("ISSUE-456".to_string()),
    ///   link_type: "Relates".to_string(),
    ///   project_key: None,
    ///   changelog_file: None,
    /// };
    ///
    /// let link_issue_executor = LinkIssueExecutor::new(cfg_data, LinkIssueActionValues::Create, link_issue_args);
    ///
    /// ```
    pub fn new(
        cfg_data: ConfigFile,
        link_issue_action: LinkIssueActionValues,
        link_issue_args: LinkIssueArgs,
    ) -> Self {
        let link_issue_cmd_runner = LinkIssueCmdRunner::new(cfg_data.clone());
        Self {
            link_issue_cmd_runner,
            link_issue_action,
            link_issue_args,
        }
    }
}

/// Implementation of the `ExecJiraCommand` trait for `VersionExecutor`
/// This trait is responsible for executing the Jira command
///
/// # Methods
///
/// * `exec_jira_command(&self) -> Result<(), Box<dyn std::error::Error>>` - executes the Jira command
impl ExecJiraCommand for LinkIssueExecutor {
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
    /// use jirust_cli::executors::jira_commands_executors::jira_issue_link_executor::LinkIssueExecutor;
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use jirust_cli::args::commands::{LinkIssueArgs, LinkIssueActionValues};
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # tokio_test::block_on(async {
    /// let link_issue_action = LinkIssueActionValues::Create;
    /// let link_issue_args = LinkIssueArgs {
    ///   link_act: link_issue_action.clone(),
    ///   origin_issue_key: "ISSUE-123".to_string(),
    ///   destination_issue_key: Some("ISSUE-456".to_string()),
    ///   link_type: "Relates".to_string(),
    ///   project_key: None,
    ///   changelog_file: None,
    /// };
    ///
    /// let cfg_data = ConfigFile::default();
    /// let link_issue_executor = LinkIssueExecutor::new(cfg_data, link_issue_action, link_issue_args);
    ///
    /// link_issue_executor.exec_jira_command().await?;
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    async fn exec_jira_command(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self.link_issue_action {
            LinkIssueActionValues::Create => {
                let res = self
                    .link_issue_cmd_runner
                    .link_jira_issues(LinkIssueCmdParams::from(&self.link_issue_args))
                    .await?;
                print_data(
                    PrintableData::Generic { data: vec![res] },
                    OutputValues::Json,
                    OutputType::Single,
                );
            }
        }
        Ok(())
    }
}
