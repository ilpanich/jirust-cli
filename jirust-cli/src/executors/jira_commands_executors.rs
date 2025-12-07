//! Jira-specific command executors wrapping runner implementations.
use crate::utils::PrintableData;
/// Executor for issue-related commands.
pub mod jira_issue_executor;
/// Executor for issue link commands.
pub mod jira_issue_link_executor;
/// Executor for issue transition commands.
pub mod jira_issue_transition_executor;
/// Executor for project commands.
pub mod jira_project_executor;
/// Executor for version commands.
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
    ) -> impl std::future::Future<Output = Result<Vec<PrintableData>, Box<dyn std::error::Error>>>;
}
