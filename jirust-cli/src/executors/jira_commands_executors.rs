pub mod jira_version_executor;

/// Trait to be impelemented to execute Jira commands in Jira commands executors
pub trait ExecJiraCommand {
    async fn exec_jira_command(&self) -> Result<(), Box<dyn std::error::Error>>;
}
