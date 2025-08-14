use crate::args::commands::{AttachmentActionValues, AttachmentArgs};
use crate::config::config_file::ConfigFile;
use crate::runners::jira_cmd_runners::attachment_cmd_runner::{
    AttachmentCmdRunner, AttachmentCmdParams,
};
use crate::utils::PrintableData;

use std::error::Error;
use super::ExecJiraCommand;

/// AttachmentExecutor is responsible for executing the Jira attachment-related commands
///
/// # Fields
///
/// * `attachment_cmd_runner` - the runner responsible for running the Jira attachment-related commands
/// * `attachment_action` - the action to be performed on the Jira attachment
/// * `attachment_args` - the arguments passed to the Jira attachment command
pub struct AttachmentExecutor {
    /// attachment_cmd_runner is the runner responsible for running the Jira attachment-related commands
    attachment_cmd_runner: AttachmentCmdRunner,
    /// attachment_action is the action to be performed on the Jira attachment
    attachment_action: AttachmentActionValues,
    /// attachment_args are the arguments passed to the Jira attachment command
    attachment_args: AttachmentArgs,
}


/// AttachmentExecutor implementation
///
/// #Methods
///
/// * `new(cfg_data: ConfigFile, attachment_action: AttachmentActionValues, attachment_args: AttachmentArgs) -> Self` - creates a new AttachmentExecutor instance
/// * `exec_jira_command(&self) -> Result<(), Box<dyn std::error::Error>>` - executes the Jira command
impl AttachmentExecutor {
    /// Creates a new AttachmentExecutor instance
    ///
    /// # Arguments
    ///
    /// * `cfg_data: ConfigFile` - configuration file data
    /// * `attachment_action` - the action to be performed on the Jira attachment
    /// * `attachment_args` - the arguments passed to the Jira attachment command
    ///
    /// # Returns
    ///
    /// * `Self` - a new AttachmentExecutor instance
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jirust_cli::executors::jira_commands_executors::jira_issue_attach_executor::AttachmentExecutor;
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use jirust_cli::args::commands::{AttachmentActionValues, AttachmentArgs};
    ///
    /// let cfg_data = ConfigFile::default();
    /// let attachment_args = AttachmentArgs {
    ///   attachment_act: AttachmentActionValues::Add,
    ///   issue_key: "ISSUE-123".to_string(),
    ///   attachment_file: "/tmp/test.txt".to_string(),
    ///   output: jirust_cli::args::commands::OutputArgs::default(),
    /// };
    ///
    /// let attachment_executor = AttachmentExecutor::new(cfg_data, AttachmentActionValues::Add, attachment_args);
    ///
    /// ```
    pub fn new(
        cfg_data: ConfigFile,
        attachment_action: AttachmentActionValues,
        attachment_args: AttachmentArgs,
    ) -> Self {
        let attachment_cmd_runner = AttachmentCmdRunner::new(&cfg_data);
        Self {
            attachment_cmd_runner,
            attachment_action,
            attachment_args,
        }
    }

    async fn add_attachment(&self) -> Result<Vec<PrintableData>, Box<dyn Error>> {
        let params = AttachmentCmdParams::from(&self.attachment_args);
        match self
            .attachment_cmd_runner
            .add_attachment(params)
            .await
        {
            Ok(res) => Ok(vec![PrintableData::Generic { data: vec![res] }]),
            Err(err) => Err(Box::new(std::io::Error::other(format!(
                "Error adding attachment: {err}"
            )))),
        }
    }
}

/// Implementation of the `ExecJiraCommand` trait for `AttachmentExecutor`
/// This trait is responsible for executing the Jira command
///
/// # Methods
///
/// * `exec_jira_command(&self) -> Result<(), Box<dyn std::error::Error>>` - executes the Jira command
impl ExecJiraCommand for AttachmentExecutor {
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
    /// use jirust_cli::executors::jira_commands_executors::jira_issue_attach_executor::AttachmentExecutor;
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use jirust_cli::args::commands::{AttachmentArgs, AttachmentActionValues};
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # tokio_test::block_on(async {
    /// let attachment_action = AttachmentActionValues::Add;
    /// let attachment_args = AttachmentArgs {
    ///   attachment_act: attachment_action.clone(),
    ///   issue_key: "ISSUE-123".to_string(),
    ///   attachment_file: "/tmp/test.txt".to_string(),
    ///   output: jirust_cli::args::commands::OutputArgs::default(),
    /// };
    ///
    /// let cfg_data = ConfigFile::default();
    /// let attachment_executor = AttachmentExecutor::new(cfg_data, attachment_action, attachment_args);
    ///
    /// attachment_executor.exec_jira_command().await?;
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    async fn exec_jira_command(&self) -> Result<Vec<PrintableData>, Box<dyn std::error::Error>> {
        match self.attachment_action {
            AttachmentActionValues::Add => self.add_attachment().await,
            AttachmentActionValues::List => self.list_attachments().await,
        }
    }
}

impl AttachmentExecutor {
    async fn list_attachments(&self) -> Result<Vec<PrintableData>, Box<dyn Error>> {
        let params = AttachmentCmdParams::from(&self.attachment_args);
        match self
            .attachment_cmd_runner
            .list_attachments(params)
            .await
        {
            Ok(res) => Ok(vec![PrintableData::Generic { data: vec![res] }]),
            Err(err) => Err(Box::new(std::io::Error::other(format!(
                "Error listing attachments: {err}"
            )))),
        }
    }
}
