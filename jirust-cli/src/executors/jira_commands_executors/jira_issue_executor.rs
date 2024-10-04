use crate::{
    args::commands::{IssueActionValues, IssueArgs, OutputValues},
    config::config_file::ConfigFile,
    runners::jira_cmd_runners::issue_cmd_runner::{IssueCmdParams, IssueCmdRunner},
    utils::{print_data, OutputType, PrintableData},
};

use super::ExecJiraCommand;

/// IssueExecutor is responsible for executing the Jira issue-related command
///
/// # Fields
///
/// * `issue_cmd_runner` - the runner responsible for running the Jira issue-related commands
/// * `issue_action` - the action to be performed on the Jira issue
/// * `issue_args` - the arguments passed to the Jira issue command
pub struct IssueExecutor {
    /// issue_cmd_runner is the runner responsible for running the Jira issue-related commands
    issue_cmd_runner: IssueCmdRunner,
    /// issue_action is the action to be performed on the Jira issue
    issue_action: IssueActionValues,
    /// issue_args are the arguments passed to the Jira issue command
    issue_args: IssueArgs,
}

/// Implementation of IssueExecutor
///
/// # Methods
///
/// * `new(cfg_data: ConfigFile, issue_action: IssueActionValues, issue_args: IssueArgs) -> Self` - creates a new IssueExecutor instance
/// * `exec_jira_command(&self) -> Result<(), Box<dyn std::error::Error>>` - executes the Jira command
impl IssueExecutor {
    /// Creates a new IssueExecutor instance
    ///
    /// # Arguments
    ///
    /// * `cfg_data: ConfigFile` - configuration file data
    /// * `issue_action: IssueActionValues` - issue action
    /// * `issue_args: IssueArgs` - issue arguments
    ///
    /// # Returns
    ///
    /// * `Self` - a new IssueExecutor instance
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jirust_cli::executors::jira_commands_executors::jira_issue_executor::IssueExecutor;
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use jirust_cli::args::commands::{IssueActionValues, IssueArgs, PaginationArgs, OutputArgs};
    ///
    /// let cfg_data = ConfigFile::default();
    /// let issue_action = IssueActionValues::Get;
    /// let issue_args = IssueArgs {
    ///    issue_act: issue_action.clone(),
    ///    project_key: "project_key".to_string(),
    ///    issue_key: Some("issue_key".to_string()),
    ///    issue_fields: None,
    ///    transition_to: None,
    ///    assignee: None,
    ///    pagination: PaginationArgs { page_size: Some(20), page_offset: None },
    ///    output: OutputArgs { output: None },
    /// };
    ///
    /// let issue_executor = IssueExecutor::new(cfg_data, issue_action, issue_args);
    /// ```
    pub fn new(
        cfg_data: ConfigFile,
        issue_action: IssueActionValues,
        issue_args: IssueArgs,
    ) -> Self {
        let issue_cmd_runner = IssueCmdRunner::new(cfg_data.clone());
        Self {
            issue_cmd_runner,
            issue_action,
            issue_args,
        }
    }
}

/// Implementation of ExecJiraCommand for IssueExecutor
/// This trait is responsible for executing the Jira command
///
/// # Methods
///
/// * `exec_jira_command(&self) -> Result<(), Box<dyn std::error::Error>` - executes the Jira command
impl ExecJiraCommand for IssueExecutor {
    /// Executes the Jira command
    ///
    /// # Returns
    ///
    /// * `Result<(), Box<dyn std::error::Error>>` - the result of the command execution
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jirust_cli::executors::jira_commands_executors::ExecJiraCommand;
    /// use jirust_cli::executors::jira_commands_executors::jira_issue_executor::IssueExecutor;
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use jirust_cli::args::commands::{IssueActionValues, IssueArgs, PaginationArgs, OutputArgs};
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # tokio_test::block_on(async {
    /// let cfg_data = ConfigFile::default();
    /// let issue_action = IssueActionValues::Get;
    /// let issue_args = IssueArgs {
    ///   issue_act: issue_action.clone(),
    ///   project_key: "project_key".to_string(),
    ///   issue_key: Some("issue_key".to_string()),
    ///   issue_fields: None,
    ///   transition_to: None,
    ///   assignee: None,
    ///   pagination: PaginationArgs { page_size: Some(20), page_offset: None },
    ///   output: OutputArgs { output: None },
    /// };
    ///
    /// let issue_executor = IssueExecutor::new(cfg_data, issue_action, issue_args);
    ///
    /// let res = issue_executor.exec_jira_command();
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    async fn exec_jira_command(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self.issue_action {
            IssueActionValues::Assign => {
                let _res = self
                    .issue_cmd_runner
                    .assign_jira_issue(IssueCmdParams::from(&self.issue_args))
                    .await?;
            }
            IssueActionValues::Create => {
                let res = self
                    .issue_cmd_runner
                    .create_jira_issue(IssueCmdParams::from(&self.issue_args))
                    .await?;
                print_data(
                    PrintableData::IssueCreated {
                        issues: (vec![res]),
                    },
                    self.issue_args.output.output.unwrap_or(OutputValues::Json),
                    OutputType::Basic,
                )
            }
            IssueActionValues::Delete => {
                self.issue_cmd_runner
                    .delete_jira_issue(IssueCmdParams::from(&self.issue_args))
                    .await?;
            }
            IssueActionValues::Get => {
                let res = self
                    .issue_cmd_runner
                    .get_jira_issue(IssueCmdParams::from(&self.issue_args))
                    .await?;
                print_data(
                    PrintableData::IssueData { issues: vec![res] },
                    self.issue_args.output.output.unwrap_or(OutputValues::Json),
                    OutputType::Single,
                )
            }
            IssueActionValues::Transition => {
                let _res = self
                    .issue_cmd_runner
                    .transition_jira_issue(IssueCmdParams::from(&self.issue_args))
                    .await?;
            }
            IssueActionValues::Update => {
                let _res = self
                    .issue_cmd_runner
                    .update_jira_issue(IssueCmdParams::from(&self.issue_args))
                    .await?;
            }
        }
        Ok(())
    }
}
