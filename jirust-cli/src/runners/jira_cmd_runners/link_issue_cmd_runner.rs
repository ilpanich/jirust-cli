use jira_v3_openapi::apis::Error;
use jira_v3_openapi::apis::configuration::Configuration;
use jira_v3_openapi::apis::issue_links_api::link_issues;
use jira_v3_openapi::models::{IssueLinkType, LinkIssueRequestJsonBean, LinkedIssue};
use serde_json::Value;

use crate::args::commands::LinkIssueArgs;
use crate::config::config_file::{AuthData, ConfigFile};
use crate::utils::changelog_extractor::ChangelogExtractor;

/// Link issue command runner
/// This struct is responsible for running the link issue command
/// It uses the Jira API to perform the operations
///
/// # Fields
///
/// * `cfg` - Configuration object
pub struct LinkIssueCmdRunner {
    /// Configuration object
    cfg: Configuration,
}

/// Implementation of IssueCmdRunner
///
/// # Methods
///
/// * `new` - Creates a new instance of LinkIssueCmdRunner
/// * `link_jira_issues` - Links Jira issues
impl LinkIssueCmdRunner {
    /// Creates a new instance of LinkIssueCmdRunner
    ///
    /// # Arguments
    ///
    /// * `cfg_file` - Configuration file
    ///
    /// # Returns
    ///
    /// * `LinkIssueCmdRunner` - Instance of LinkIssueCmdRunner
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use jirust_cli::runners::jira_cmd_runners::link_issue_cmd_runner::LinkIssueCmdRunner;
    /// use toml::Table;
    ///
    /// let cfg_file = ConfigFile::new("dXNlcm5hbWU6YXBpX2tleQ==".to_string(), "jira_url".to_string(), "standard_resolution".to_string(), "standard_resolution_comment".to_string(), Table::new());
    ///
    /// let link_issue_cmd_runner = LinkIssueCmdRunner::new(cfg_file);
    /// ```
    pub fn new(cfg_file: ConfigFile) -> LinkIssueCmdRunner {
        let mut config = Configuration::new();
        let auth_data = AuthData::from_base64(cfg_file.get_auth_key());
        config.base_path = cfg_file.get_jira_url().to_string();
        config.basic_auth = Some((auth_data.0, Some(auth_data.1)));
        LinkIssueCmdRunner { cfg: config }
    }

    /// Links Jira issues
    ///
    /// # Arguments
    ///
    /// * `params` - LinkIssueCmdParams struct
    ///
    /// # Returns
    ///
    /// * `Result<Value, Box<dyn std::error::Error>>` - Result of the operation
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jirust_cli::runners::jira_cmd_runners::link_issue_cmd_runner::{LinkIssueCmdRunner, LinkIssueCmdParams};
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use toml::Table;
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # tokio_test::block_on(async {
    /// let cfg_file = ConfigFile::new("dXNlcm5hbWU6YXBpX2tleQ==".to_string(), "jira_url".to_string(), "standard_resolution".to_string(), "standard_resolution_comment".to_string(), Table::new());
    /// let link_issue_cmd_runner = LinkIssueCmdRunner::new(cfg_file);
    /// let mut params = LinkIssueCmdParams::new();
    /// params.origin_issue_key = "ISSUE-1".to_string();
    /// params.destination_issue_key = Some("ISSUE-2".to_string());
    /// params.link_type = "Blocks".to_string();
    ///
    /// let result = link_issue_cmd_runner.link_jira_issues(params).await?;
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    pub async fn link_jira_issues(
        &self,
        params: LinkIssueCmdParams,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let mut link_requests: Vec<LinkIssueRequestJsonBean> = Vec::new();
        if params.destination_issue_key.is_some() {
            let link_request = LinkIssueRequestJsonBean {
                comment: None,
                inward_issue: Box::new(LinkedIssue {
                    key: Some(params.origin_issue_key),
                    id: None,
                    fields: None,
                    param_self: None,
                }),
                outward_issue: Box::new(LinkedIssue {
                    key: params.destination_issue_key,
                    id: None,
                    fields: None,
                    param_self: None,
                }),
                r#type: Box::new(IssueLinkType {
                    name: Some(params.link_type),
                    inward: None,
                    outward: None,
                    id: None,
                    param_self: None,
                }),
            };
            link_requests.push(link_request);
        } else {
            let changelog_extractor = ChangelogExtractor::new(params.changelog_file.unwrap());
            let version_data: Option<String> = Some(
                changelog_extractor
                    .extract_version_changelog()
                    .unwrap_or_default(),
            );
            if version_data.is_some() {
                let issues = changelog_extractor
                    .extract_issues_from_changelog(
                        version_data.clone().unwrap(),
                        params.project_key.clone().expect("Project key is required"),
                    )
                    .unwrap_or_default();
                link_requests = issues
                    .iter()
                    .map(|issue| LinkIssueRequestJsonBean {
                        comment: None,
                        inward_issue: Box::new(LinkedIssue {
                            key: Some(params.origin_issue_key.clone()),
                            id: None,
                            fields: None,
                            param_self: None,
                        }),
                        outward_issue: Box::new(LinkedIssue {
                            key: Some(issue.clone()),
                            id: None,
                            fields: None,
                            param_self: None,
                        }),
                        r#type: Box::new(IssueLinkType {
                            name: Some(params.link_type.clone()),
                            inward: None,
                            outward: None,
                            id: None,
                            param_self: None,
                        }),
                    })
                    .collect();
            }
        };

        let mut link_result: Value = Value::String("Linking OK".to_string());

        for link_issue_request_json_bean in link_requests {
            match link_issues(&self.cfg, link_issue_request_json_bean).await {
                Ok(_) => {}
                Err(Error::Serde(_)) => {}
                Err(_) => {
                    link_result = Value::String("Linking KO".to_string());
                }
            };
        }
        Ok(link_result)
    }
}

/// Link issue command parameters
///
/// # Fields
///
/// * `project_key` - Jira project key
/// * `origin_issue_key` - Jira origin issue key
/// * `destination_issue_key` - Jira destination issue key
/// * `link_type` - Jira link type
/// * `changelog_file` - Changelog file
pub struct LinkIssueCmdParams {
    /// Jira project key
    pub project_key: Option<String>,
    /// Jira issue key
    pub origin_issue_key: String,
    /// Jira issue key
    pub destination_issue_key: Option<String>,
    /// Jira issue fields
    pub link_type: String,
    /// Jira issue transition
    pub changelog_file: Option<String>,
}

/// Implementation of LinkIssueCmdParams struct
///
/// # Methods
///
/// * `new` - Creates a new LinkIssueCmdParams instance
impl LinkIssueCmdParams {
    /// Creates a new LinkIssueCmdParams instance
    ///
    /// # Returns
    ///
    /// * `LinkIssueCmdParams` - Issue command parameters
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::runners::jira_cmd_runners::link_issue_cmd_runner::LinkIssueCmdParams;
    ///
    /// let params = LinkIssueCmdParams::new();
    /// ```
    pub fn new() -> LinkIssueCmdParams {
        LinkIssueCmdParams {
            project_key: None,
            origin_issue_key: "".to_string(),
            destination_issue_key: None,
            link_type: "".to_string(),
            changelog_file: None,
        }
    }
}

/// Implementation of From trait for LinkIssueCmdParams struct
/// to convert LinkIssueArgs struct to LinkIssueCmdParams struct
impl From<&LinkIssueArgs> for LinkIssueCmdParams {
    /// Converts LinkIssueArgs struct to LinkIssueCmdParams struct
    /// to create a new LinkIssueCmdParams instance
    ///
    /// # Arguments
    ///
    /// * `value` - LinkIssueArgs struct
    ///
    /// # Returns
    ///
    /// * `LinkIssueArgs` - Link issue command parameters
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::runners::jira_cmd_runners::link_issue_cmd_runner::LinkIssueCmdParams;
    /// use jirust_cli::args::commands::{LinkIssueArgs, LinkIssueActionValues};
    /// use std::collections::HashMap;
    /// use serde_json::Value;
    ///
    /// let link_issue_args = LinkIssueArgs {
    ///    link_act: LinkIssueActionValues::Create,
    ///    project_key: Some("project_key".to_string()),
    ///    origin_issue_key: "origin_issue_key".to_string(),
    ///    destination_issue_key: Some("destination_issue_key".to_string()),
    ///    link_type: "link_type".to_string(),
    ///    changelog_file: None,
    /// };
    ///
    /// let params = LinkIssueCmdParams::from(&link_issue_args);
    ///
    /// assert_eq!(params.project_key, Some("project_key".to_string()));
    /// assert_eq!(params.origin_issue_key, "origin_issue_key".to_string());
    /// assert_eq!(params.destination_issue_key, Some("destination_issue_key".to_string()));
    /// assert_eq!(params.link_type, "link_type".to_string());
    /// assert_eq!(params.changelog_file, None);
    ///
    /// ```
    fn from(value: &LinkIssueArgs) -> Self {
        if (value.destination_issue_key.is_none() && value.changelog_file.is_none())
            || (value.destination_issue_key.is_some() && value.changelog_file.is_some())
        {
            panic!("Either destination issue key or changelog file is required");
        }
        if value.changelog_file.is_some() && value.project_key.is_none() {
            panic!("Project key is required when changelog file is provided");
        }
        LinkIssueCmdParams {
            project_key: value.project_key.clone(),
            origin_issue_key: value.origin_issue_key.clone(),
            destination_issue_key: value.destination_issue_key.clone(),
            link_type: value.link_type.clone(),
            changelog_file: value.changelog_file.clone(),
        }
    }
}

/// Default implementation for IssueCmdParams struct
impl Default for LinkIssueCmdParams {
    /// Creates a default LinkIssueCmdParams instance
    ///
    /// # Returns
    ///
    /// A LinkIssueCmdParams instance with default values
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::runners::jira_cmd_runners::link_issue_cmd_runner::LinkIssueCmdParams;
    ///
    /// let params = LinkIssueCmdParams::default();
    ///
    /// assert_eq!(params.project_key, None);
    /// assert_eq!(params.origin_issue_key, "".to_string());
    /// assert_eq!(params.destination_issue_key, None);
    /// assert_eq!(params.link_type, "".to_string());
    /// assert_eq!(params.changelog_file, None);
    /// ```
    fn default() -> Self {
        LinkIssueCmdParams::new()
    }
}
