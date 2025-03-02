use crate::{
    args::commands::{TransitionActionValues, TransitionArgs},
    config::config_file::ConfigFile,
    runners::jira_cmd_runners::issue_cmd_runner::{IssueCmdRunner, IssueTransitionCmdParams},
    utils::PrintableData,
};

use std::io::{Error, ErrorKind};

use super::ExecJiraCommand;

/// IssueTransitionExecutor is responsible for executing the Jira issue transition-related command
///
/// # Fields
///
/// * `issue_transition_cmd_runner` - the runner responsible for running the Jira issue transition-related commands
/// * `issue_transition_action` - the action to be performed on the Jira issue transition
/// * `issue_transition_args` - the arguments passed to the Jira issue transition command
pub struct IssueTransitionExecutor {
    /// issue_transition_cmd_runner is the runner responsible for running the Jira issue transition-related commands
    issue_transition_cmd_runner: IssueCmdRunner,
    /// issue_transition_action is the action to be performed on the Jira issue transition
    issue_transition_action: TransitionActionValues,
    /// issue_transition_args are the arguments passed to the Jira issue transition command
    issue_transition_args: TransitionArgs,
}

/// Implementation of IssueTransitionExecutor
///
/// # Methods
///
/// * `new(cfg_data: ConfigFile, issue_transition_action: TransitionActionValues, issue_transition_args: TransitionArgs) -> Self` - creates a new IssueTransitionExecutor instance
/// * `exec_jira_command(&self) -> Result<(), Box<dyn std::error::Error>>` - executes the Jira command
impl IssueTransitionExecutor {
    /// Creates a new IssueTransitionExecutor instance
    ///
    /// # Arguments
    ///
    /// * `cfg_data: ConfigFile` - configuration file data
    /// * `issue_transition_action: TransitionActionValues` - issue transition action
    /// * `issue_transition_args: TransitionArgs` - issue transition arguments
    ///
    /// # Returns
    ///
    /// * `Self` - a new IssueTransitionExecutor instance
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jirust_cli::executors::jira_commands_executors::jira_issue_transition_executor::IssueTransitionExecutor;
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use jirust_cli::args::commands::{TransitionActionValues, TransitionArgs, OutputArgs};
    ///
    /// let cfg_data = ConfigFile::default();
    /// let issue_transition_args = TransitionArgs {
    ///    transition_act: TransitionActionValues::List,
    ///    issue_key: "ISSUE-123".to_string(),
    ///    output: OutputArgs { output_format: None, output_type: None },
    /// };
    ///
    /// let issue_transition_executor = IssueTransitionExecutor::new(cfg_data, TransitionActionValues::List, issue_transition_args);
    /// ```
    pub fn new(
        cfg_data: ConfigFile,
        issue_transition_action: TransitionActionValues,
        issue_transition_args: TransitionArgs,
    ) -> Self {
        let issue_transition_cmd_runner = IssueCmdRunner::new(cfg_data.clone());
        Self {
            issue_transition_cmd_runner,
            issue_transition_action,
            issue_transition_args,
        }
    }
}

/// Implementation of ExecJiraCommand for IssueTransitionExecutor
/// This trait is responsible for executing the Jira command
///
/// # Methods
///
/// * `exec_jira_command(&self) -> Result<(), Box<dyn std::error::Error>>` - executes the Jira command
impl ExecJiraCommand for IssueTransitionExecutor {
    /// Executes the Jira command
    ///
    /// # Returns
    ///
    /// * `Result<(), Box<dyn std::error::Error>>` - the result of the execution
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jirust_cli::executors::jira_commands_executors::ExecJiraCommand;
    /// use jirust_cli::executors::jira_commands_executors::jira_issue_transition_executor::IssueTransitionExecutor;
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use jirust_cli::args::commands::{TransitionActionValues, TransitionArgs, OutputArgs};
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # tokio_test::block_on(async {
    /// let cfg_data = ConfigFile::default();
    /// let issue_transition_args = TransitionArgs {
    ///   transition_act: TransitionActionValues::List,
    ///   issue_key: "ISSUE-123".to_string(),
    ///   output: OutputArgs { output_format: None, output_type: None },
    /// };
    ///
    /// let issue_transition_executor = IssueTransitionExecutor::new(cfg_data, TransitionActionValues::List, issue_transition_args);
    /// issue_transition_executor.exec_jira_command();
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    async fn exec_jira_command(&self) -> Result<Vec<PrintableData>, Box<dyn std::error::Error>> {
        match self.issue_transition_action {
            TransitionActionValues::List => {
                match self
                    .issue_transition_cmd_runner
                    .get_issue_available_transitions(IssueTransitionCmdParams::from(
                        &self.issue_transition_args,
                    ))
                    .await
                {
                    Ok(issue_transitions) => Ok(vec![PrintableData::IssueTransitions {
                        transitions: issue_transitions.transitions.unwrap_or(vec![]),
                    }]),
                    Err(err) => Err(Box::new(Error::new(
                        ErrorKind::Other,
                        format!("Error listing issue transitions: {}", err),
                    ))),
                }
            }
        }
    }
}
