pub mod jira_issue_executor;
pub mod jira_issue_link_executor;
pub mod jira_issue_transition_executor;
pub mod jira_project_executor;
pub mod jira_version_executor;

/// Trait to be impelemented to execute Jira commands in Jira commands executors
pub trait ExecJiraCommand {
    /// Asynchronous function to execute Jira command.
    ///
    /// # Returns
    ///
    /// * A Result with a unit type or a Box with a dyn std::error::Error trait.
    fn exec_jira_command(
        &self,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> + Send;
}
