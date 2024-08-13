pub mod jira_version_executor;

/// Trait to be impelemented to execute Jira commands in Jira commands executors
pub trait ExecJiraCommand {
    fn exec_jira_command(
        &self,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> + Send;
}
