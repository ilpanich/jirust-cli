use crate::{
    args::commands::{TransitionActionValues, TransitionArgs},
    config::config_file::ConfigFile,
    runners::jira_cmd_runners::issue_cmd_runner::{IssueCmdRunner, IssueTransitionCmdParams},
    utils::{print_data, OutputType, PrintableData},
};

use super::ExecJiraCommand;

pub struct IssueTransitionExecutor {
    issue_transition_cmd_runner: IssueCmdRunner,
    issue_transition_action: TransitionActionValues,
    issue_transition_args: TransitionArgs,
}

impl IssueTransitionExecutor {
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

impl ExecJiraCommand for IssueTransitionExecutor {
    async fn exec_jira_command(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self.issue_transition_action {
            TransitionActionValues::List => {
                let res = self
                    .issue_transition_cmd_runner
                    .get_issue_available_transitions(IssueTransitionCmdParams::from(
                        &self.issue_transition_args,
                    ))
                    .await?;
                print_data(
                    PrintableData::IssueTransitions {
                        transitions: res.transitions.unwrap_or(vec![]),
                    },
                    self.issue_transition_args.output.output,
                    OutputType::Full,
                )
            }
        }
        Ok(())
    }
}
