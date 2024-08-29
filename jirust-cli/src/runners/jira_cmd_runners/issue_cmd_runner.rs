use std::collections::HashMap;

use jira_v3_openapi::apis::configuration::Configuration;
use serde_json::Value;

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
}

pub struct IssueCmdParams {
    pub project_key: String,
    pub issue_key: Option<String>,
    pub issue_fields: Option<HashMap<String, Value>>,
    pub transtion: Option<String>,
}

impl IssueCmdParams {
    pub fn new() -> IssueCmdParams {
        IssueCmdParams {
            project_key: "".to_string(),
            issue_key: None,
            issue_fields: None,
            transtion: None,
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
            transtion: value.transition_to.clone(),
        }
    }
}
