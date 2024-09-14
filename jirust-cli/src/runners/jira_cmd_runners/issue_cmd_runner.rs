use jira_v3_openapi::apis::issues_api::*;
use jira_v3_openapi::models::user::AccountType;
use jira_v3_openapi::models::{CreatedIssue, IssueBean, IssueTransition, Transitions, User};
use jira_v3_openapi::{apis::configuration::Configuration, models::IssueUpdateDetails};
use serde_json::Value;
use std::collections::HashMap;

use crate::args::commands::TransitionArgs;
use crate::{
    args::commands::IssueArgs,
    config::config_file::{AuthData, ConfigFile},
};

pub struct IssueCmdRunner {
    cfg: Configuration,
}

impl IssueCmdRunner {
    pub fn new(cfg_file: ConfigFile) -> IssueCmdRunner {
        let mut config = Configuration::new();
        let auth_data = AuthData::from_base64(cfg_file.get_auth_key());
        config.base_path = cfg_file.get_jira_url().to_string();
        config.basic_auth = Some((auth_data.0, Some(auth_data.1)));
        IssueCmdRunner { cfg: config }
    }

    pub async fn assign_jira_issue(
        &self,
        params: IssueCmdParams,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let user_data = User {
            account_id: Some(params.assignee.expect("Assignee is required")),
            account_type: Some(AccountType::Atlassian),
            ..Default::default()
        };

        Ok(assign_issue(
            &self.cfg,
            params.issue_key.unwrap_or("".to_string()).as_str(),
            user_data,
        )
        .await?)
    }

    pub async fn create_jira_issue(
        &self,
        params: IssueCmdParams,
    ) -> Result<CreatedIssue, Box<dyn std::error::Error>> {
        let issue_data = IssueUpdateDetails {
            fields: params.issue_fields,
            history_metadata: None,
            properties: None,
            transition: None,
            update: None,
        };
        Ok(create_issue(&self.cfg, issue_data, None).await?)
    }

    pub async fn delete_jira_issue(
        &self,
        params: IssueCmdParams,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(delete_issue(
            &self.cfg,
            params.issue_key.expect("Issue key is required!").as_str(),
            Some("true"),
        )
        .await?)
    }

    pub async fn get_jira_issue(
        &self,
        params: IssueCmdParams,
    ) -> Result<IssueBean, Box<dyn std::error::Error>> {
        Ok(get_issue(
            &self.cfg,
            params.issue_key.expect("Issue key is required!").as_str(),
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await?)
    }

    pub async fn transition_jira_issue(
        &self,
        params: IssueCmdParams,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let transition = IssueTransition {
            id: Some(params.transition.expect("Transition is required!")),
            ..Default::default()
        };
        let issue_data = IssueUpdateDetails {
            fields: params.issue_fields,
            history_metadata: None,
            properties: None,
            transition: Some(transition),
            update: None,
        };
        Ok(do_transition(
            &self.cfg,
            params.issue_key.expect("Issue key is required").as_str(),
            issue_data,
        )
        .await?)
    }

    pub async fn update_jira_issue(
        &self,
        params: IssueCmdParams,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let issue_data = IssueUpdateDetails {
            fields: params.issue_fields,
            history_metadata: None,
            properties: None,
            transition: None,
            update: None,
        };
        Ok(edit_issue(
            &self.cfg,
            params.issue_key.expect("Issue key is required!").as_str(),
            issue_data,
            None,
            None,
            None,
            Some(true),
            None,
        )
        .await?)
    }

    pub async fn get_issue_available_transitions(
        &self,
        params: IssueTransitionCmdParams,
    ) -> Result<Transitions, Box<dyn std::error::Error>> {
        Ok(get_transitions(
            &self.cfg,
            &params.issue_key,
            None,
            None,
            None,
            Some(false),
            None,
        )
        .await?)
    }
}

pub struct IssueCmdParams {
    pub project_key: String,
    pub issue_key: Option<String>,
    pub issue_fields: Option<HashMap<String, Value>>,
    pub transition: Option<String>,
    pub assignee: Option<String>,
}

impl IssueCmdParams {
    pub fn new() -> IssueCmdParams {
        IssueCmdParams {
            project_key: "".to_string(),
            issue_key: None,
            issue_fields: None,
            transition: None,
            assignee: None,
        }
    }
}

impl From<&IssueArgs> for IssueCmdParams {
    fn from(value: &IssueArgs) -> Self {
        IssueCmdParams {
            project_key: value.project_key.clone(),
            issue_key: value.issue_key.clone(),
            issue_fields: Some(
                value
                    .issue_fields
                    .clone()
                    .unwrap_or(vec![])
                    .iter()
                    .map(|elem| {
                        (
                            elem.0.clone(),
                            serde_json::from_str(elem.1.clone().as_str()).unwrap_or(Value::Null),
                        )
                    })
                    .collect::<HashMap<_, _>>(),
            ),
            transition: value.transition_to.clone(),
            assignee: value.assignee.clone(),
        }
    }
}

pub struct IssueTransitionCmdParams {
    pub issue_key: String,
}

impl IssueTransitionCmdParams {
    pub fn new() -> IssueTransitionCmdParams {
        IssueTransitionCmdParams {
            issue_key: "".to_string(),
        }
    }
}

impl From<&TransitionArgs> for IssueTransitionCmdParams {
    fn from(value: &TransitionArgs) -> Self {
        IssueTransitionCmdParams {
            issue_key: value.issue_key.clone(),
        }
    }
}
