use std::collections::HashMap;
use std::sync::Arc;

use crate::args::commands::VersionArgs;
use crate::config::config_file::{AuthData, ConfigFile};
use crate::jira_doc_std_field;
use crate::utils::changelog_extractor::ChangelogExtractor;
use chrono::Utc;
use jira_v3_openapi::apis::Error;
use jira_v3_openapi::apis::configuration::Configuration;
use jira_v3_openapi::apis::issues_api::{assign_issue, do_transition, edit_issue, get_transitions};
use jira_v3_openapi::apis::project_versions_api::*;
use jira_v3_openapi::models::user::AccountType;
use jira_v3_openapi::models::{
    DeleteAndReplaceVersionBean, FieldUpdateOperation, IssueTransition, IssueUpdateDetails, User,
    Version, VersionRelatedWork,
};
use serde_json::Value;

#[cfg(any(windows, unix))]
use futures::StreamExt;
#[cfg(any(windows, unix))]
use futures::stream::FuturesUnordered;

/// Version command runner struct
///
/// This struct is responsible for holding the version command runner parameters
/// and it is used to pass the parameters to the version commands runner
#[derive(Clone)]
pub struct VersionCmdRunner {
    cfg: Configuration,
    resolution_value: Value,
    resolution_comment: Value,
    resolution_transition_name: Option<Vec<String>>,
}

/// Version command runner implementation.
///
///
/// # Methods
///
/// * `new` - This method creates a new instance of the VersionCmdRunner struct
/// * `create_jira_version` - This method creates a new Jira version
/// * `get_jira_version` - This method gets a Jira version
/// * `list_jira_versions` - This method lists Jira versions
/// * `update_jira_version` - This method updates a Jira version
/// * `delete_jira_version` - This method deletes a Jira version
/// * `release_jira_version` - This method releases a Jira version
/// * `archive_jira_version` - This method archives a Jira version
impl VersionCmdRunner {
    /// This method creates a new instance of the VersionCmdRunner struct
    ///
    /// # Arguments
    ///
    /// * `cfg_file` - A ConfigFile struct
    ///
    /// # Returns
    ///
    /// * A new instance of the VersionCmdRunner struct
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdRunner;
    /// use toml::Table;
    ///
    /// let cfg_file = ConfigFile::new("dXNlcm5hbWU6YXBpX2tleQ==".to_string(), "jira_url".to_string(), "standard_resolution".to_string(), "standard_resolution_comment".to_string(), Table::new());
    ///
    /// let version_cmd_runner = VersionCmdRunner::new(&cfg_file);
    /// ```
    pub fn new(cfg_file: &ConfigFile) -> VersionCmdRunner {
        let mut config = Configuration::new();
        let auth_data = AuthData::from_base64(cfg_file.get_auth_key());
        config.base_path = cfg_file.get_jira_url().to_string();
        config.basic_auth = Some((auth_data.0, Some(auth_data.1)));
        VersionCmdRunner {
            cfg: config,
            resolution_value: serde_json::from_str(cfg_file.get_standard_resolution().as_str())
                .unwrap_or(Value::Null),
            resolution_comment: serde_json::from_str(
                format!(
                    "{{\"body\": {}}}",
                    jira_doc_std_field!(cfg_file.get_standard_resolution_comment().as_str())
                )
                .as_str(),
            )
            .unwrap_or(Value::Null),
            resolution_transition_name: cfg_file.get_transition_name("resolve"),
        }
    }

    /// This method creates a new Jira version with the given parameters
    /// and returns the created version
    ///
    /// # Arguments
    ///
    /// * `params` - A VersionCmdParams struct
    ///
    /// # Returns
    ///
    /// * A Result containing a Version struct or a Box<dyn std::error::Error>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jira_v3_openapi::models::Version;
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdParams;
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdRunner;
    /// use toml::Table;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # tokio_test::block_on(async {
    /// let cfg_file = ConfigFile::new("dXNlcm5hbWU6YXBpX2tleQ==".to_string(), "jira_url".to_string(), "standard_resolution".to_string(), "standard_resolution_comment".to_string(), Table::new());
    ///
    /// let version_cmd_runner = VersionCmdRunner::new(&cfg_file);
    /// let params = VersionCmdParams::new();
    ///
    /// let version = version_cmd_runner.create_jira_version(params).await?;
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    #[cfg(any(windows, unix))]
    pub async fn create_jira_version(
        &self,
        params: VersionCmdParams,
    ) -> Result<(Version, Option<Vec<(String, String, String, String)>>), Box<dyn std::error::Error>>
    {
        let version_description: Option<String>;
        let mut resolved_issues = vec![];
        let mut transitioned_issue: Arc<Vec<(String, String, String, String)>> = Arc::new(vec![]);
        if Option::is_some(&params.changelog_file) {
            let changelog_extractor = ChangelogExtractor::new(params.changelog_file.unwrap());
            version_description = Some(changelog_extractor.extract_version_changelog().unwrap_or(
                if Option::is_some(&params.version_description) {
                    params.version_description.unwrap()
                } else {
                    "No changelog found for this version".to_string()
                },
            ));
            if Option::is_some(&params.transition_issues) && params.transition_issues.unwrap() {
                resolved_issues = changelog_extractor
                    .extract_issues_from_changelog(
                        &version_description.clone().unwrap(),
                        &params.project,
                    )
                    .unwrap_or_default();
            }
        } else {
            version_description = params.version_description;
        }
        let release_date =
            if Option::is_some(&params.version_released) && params.version_released.unwrap() {
                if Option::is_some(&params.version_release_date) {
                    params.version_release_date
                } else {
                    Some(Utc::now().format("%Y-%m-%d").to_string())
                }
            } else {
                None
            };
        let version = Version {
            project: Some(params.project),
            name: Some(
                params
                    .version_name
                    .expect("VersionName is mandatory on cretion!"),
            ),
            description: version_description,
            start_date: params.version_start_date,
            release_date,
            archived: params.version_archived,
            released: params.version_released,
            ..Default::default()
        };
        let version = create_version(&self.cfg, version).await?;
        if !resolved_issues.is_empty() {
            let user_data = if Option::is_some(&params.transition_assignee) {
                Some(User {
                    account_id: Some(params.transition_assignee.expect("Assignee is required")),
                    account_type: Some(AccountType::Atlassian),
                    ..Default::default()
                })
            } else {
                None
            };
            let mut handles = FuturesUnordered::new();
            for issue in resolved_issues {
                handles.push(self.manage_version_related_issues(issue, &user_data, &version));
            }
            while let Some(result) = handles.next().await {
                match result {
                    Ok((issue, transition_result, assign_result, fixversion_result)) => {
                        Arc::make_mut(&mut transitioned_issue).push((
                            issue,
                            transition_result,
                            assign_result,
                            fixversion_result,
                        ));
                    }
                    Err(err) => {
                        eprintln!("Error managing version related issues: {:?}", err);
                    }
                }
            }
        }
        let transitioned_issue_owned: Vec<(String, String, String, String)> =
            (*transitioned_issue).clone();
        Ok((
            version,
            if !transitioned_issue.is_empty() {
                Some(transitioned_issue_owned)
            } else {
                None
            },
        ))
    }

    /// This method creates a new Jira version with the given parameters
    /// and returns the created version
    ///
    /// # Arguments
    ///
    /// * `params` - A VersionCmdParams struct
    ///
    /// # Returns
    ///
    /// * A Result containing a Version struct or a Box<dyn std::error::Error>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jira_v3_openapi::models::Version;
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdParams;
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdRunner;
    /// use toml::Table;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # tokio_test::block_on(async {
    /// let cfg_file = ConfigFile::new("dXNlcm5hbWU6YXBpX2tleQ==".to_string(), "jira_url".to_string(), "standard_resolution".to_string(), "standard_resolution_comment".to_string(), Table::new());
    ///
    /// let version_cmd_runner = VersionCmdRunner::new(&cfg_file);
    /// let params = VersionCmdParams::new();
    ///
    /// let version = version_cmd_runner.create_jira_version(params).await?;
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    #[cfg(target_family = "wasm")]
    pub async fn create_jira_version(
        &self,
        params: VersionCmdParams,
    ) -> Result<(Version, Option<Vec<(String, String, String, String)>>), Box<dyn std::error::Error>>
    {
        let version_description: Option<String>;
        let mut resolved_issues = vec![];
        let mut transitioned_issue: Vec<(String, String, String, String)> = vec![];
        if Option::is_some(&params.changelog_file) {
            let changelog_extractor = ChangelogExtractor::new(params.changelog_file.unwrap());
            version_description = Some(changelog_extractor.extract_version_changelog().unwrap_or(
                if Option::is_some(&params.version_description) {
                    params.version_description.unwrap()
                } else {
                    "No changelog found for this version".to_string()
                },
            ));
            if Option::is_some(&params.transition_issues) && params.transition_issues.unwrap() {
                resolved_issues = changelog_extractor
                    .extract_issues_from_changelog(
                        &version_description.clone().unwrap(),
                        &params.project,
                    )
                    .unwrap_or_default();
            }
        } else {
            version_description = params.version_description;
        }
        let release_date =
            if Option::is_some(&params.version_released) && params.version_released.unwrap() {
                if Option::is_some(&params.version_release_date) {
                    params.version_release_date
                } else {
                    Some(Utc::now().format("%Y-%m-%d").to_string())
                }
            } else {
                None
            };
        let version = Version {
            project: Some(params.project),
            name: Some(
                params
                    .version_name
                    .expect("VersionName is mandatory on cretion!"),
            ),
            description: version_description,
            start_date: params.version_start_date,
            release_date,
            archived: params.version_archived,
            released: params.version_released,
            ..Default::default()
        };
        let version = create_version(&self.cfg, version).await?;
        if !resolved_issues.is_empty() {
            let user_data = if Option::is_some(&params.transition_assignee) {
                Some(User {
                    account_id: Some(params.transition_assignee.expect("Assignee is required")),
                    account_type: Some(AccountType::Atlassian),
                    ..Default::default()
                })
            } else {
                None
            };
            for issue in resolved_issues {
                let all_transitions: Vec<IssueTransition> = get_transitions(
                    &self.cfg,
                    issue.clone().as_str(),
                    None,
                    None,
                    None,
                    Some(false),
                    None,
                )
                .await?
                .transitions
                .unwrap_or_default();
                let transition_names: Vec<String> = self
                    .resolution_transition_name
                    .clone()
                    .expect("Transition name is required and must be set in the config file");
                let resolve_transitions: Vec<IssueTransition> = all_transitions
                    .into_iter()
                    .filter(|t| {
                        transition_names.contains(&t.name.clone().unwrap_or("".to_string()))
                    })
                    .collect();
                let transition_ids = resolve_transitions
                    .into_iter()
                    .map(|t| t.id.clone().unwrap_or("".to_string()))
                    .collect::<Vec<String>>();
                let transitions = transition_ids
                    .into_iter()
                    .map(|id| {
                        Some(IssueTransition {
                            id: Some(id),
                            ..Default::default()
                        })
                    })
                    .collect::<Vec<Option<IssueTransition>>>();
                let mut update_fields_hashmap: HashMap<String, Vec<FieldUpdateOperation>> =
                    HashMap::new();
                let mut transition_fields_hashmap: HashMap<String, Vec<FieldUpdateOperation>> =
                    HashMap::new();
                let mut version_update_op = FieldUpdateOperation::new();
                let mut version_resolution_update_field = HashMap::new();
                let mut version_resolution_comment_op = FieldUpdateOperation::new();
                let version_json: Value =
                    serde_json::from_str(serde_json::to_string(&version).unwrap().as_str())
                        .unwrap_or(Value::Null);
                let resolution_value = self.resolution_value.clone();
                let comment_value = self.resolution_comment.clone();
                version_update_op.add = Some(Some(version_json));
                version_resolution_update_field.insert("resolution".to_string(), resolution_value);
                version_resolution_comment_op.add = Some(Some(comment_value));
                update_fields_hashmap.insert("fixVersions".to_string(), vec![version_update_op]);
                transition_fields_hashmap
                    .insert("comment".to_string(), vec![version_resolution_comment_op]);
                let issue_update_data = IssueUpdateDetails {
                    fields: None,
                    history_metadata: None,
                    properties: None,
                    transition: None,
                    update: Some(update_fields_hashmap),
                };
                let mut transition_result: String = "KO".to_string();
                if !Vec::is_empty(&transitions) {
                    for transition in transitions {
                        let issue_transition_data = IssueUpdateDetails {
                            fields: Some(version_resolution_update_field.clone()),
                            history_metadata: None,
                            properties: None,
                            transition: Some(transition.clone().unwrap()),
                            update: Some(transition_fields_hashmap.clone()),
                        };
                        match do_transition(
                            &self.cfg,
                            issue.clone().as_str(),
                            issue_transition_data,
                        )
                        .await
                        {
                            Ok(_) => {
                                transition_result = "OK".to_string();
                                break;
                            }
                            Err(Error::Serde(e)) => {
                                if e.is_eof() {
                                    transition_result = "OK".to_string();
                                    break;
                                } else {
                                    transition_result = "KO".to_string()
                                }
                            }
                            Err(_) => transition_result = "KO".to_string(),
                        }
                    }
                }
                let assign_result: String = match assign_issue(
                    &self.cfg,
                    issue.clone().as_str(),
                    user_data.clone().unwrap(),
                )
                .await
                {
                    Ok(_) => "OK".to_string(),
                    Err(Error::Serde(e)) => {
                        if e.is_eof() {
                            "OK".to_string()
                        } else {
                            "KO".to_string()
                        }
                    }
                    Err(_) => "KO".to_string(),
                };
                let fixversion_result: String = match edit_issue(
                    &self.cfg,
                    issue.clone().as_str(),
                    issue_update_data,
                    Some(true),
                    None,
                    None,
                    Some(true),
                    None,
                )
                .await
                {
                    Ok(_) => version.clone().name.unwrap_or("".to_string()),
                    Err(_) => "NO fixVersion set".to_string(),
                };
                transitioned_issue.push((
                    issue.clone(),
                    transition_result,
                    assign_result,
                    fixversion_result,
                ));
            }
        }
        Ok((
            version,
            if !transitioned_issue.is_empty() {
                Some(transitioned_issue)
            } else {
                None
            },
        ))
    }

    /// This method gets a Jira version with the given parameters
    /// and returns the version
    /// If the version is not found, it returns an error
    ///
    /// # Arguments
    ///
    /// * `params` - A VersionCmdParams struct
    ///
    /// # Returns
    ///
    /// * A Result containing a Version struct or an Error<GetVersionError>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jira_v3_openapi::models::Version;
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdParams;
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdRunner;
    /// use toml::Table;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # tokio_test::block_on(async {
    /// let cfg_file = ConfigFile::new("dXNlcm5hbWU6YXBpX2tleQ==".to_string(), "jira_url".to_string(), "standard_resolution".to_string(), "standard_resolution_comment".to_string(), Table::new());
    /// let version_cmd_runner = VersionCmdRunner::new(&cfg_file);
    /// let params = VersionCmdParams::new();
    ///
    /// let version = version_cmd_runner.get_jira_version(params).await?;
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    pub async fn get_jira_version(
        &self,
        params: VersionCmdParams,
    ) -> Result<Version, Error<GetVersionError>> {
        get_version(
            &self.cfg,
            params.version_id.expect("VersionID is mandatory!").as_str(),
            None,
        )
        .await
    }

    /// This method lists Jira versions with the given parameters
    /// and returns the versions
    /// If there are no versions, it returns an empty vector
    /// If the version is not found, it returns an error
    /// If the version page size is given, it returns the paginated versions
    /// Otherwise, it returns all versions
    ///
    /// # Arguments
    ///
    /// * `params` - A VersionCmdParams struct
    ///
    /// # Returns
    ///
    /// * A Result containing a vector of Version structs or a Box<dyn std::error::Error>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jira_v3_openapi::models::Version;
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdParams;
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdRunner;
    /// use toml::Table;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # tokio_test::block_on(async {
    /// let cfg_file = ConfigFile::new("dXNlcm5hbWU6YXBpX2tleQ==".to_string(), "jira_url".to_string(), "standard_resolution".to_string(), "standard_resolution_comment".to_string(), Table::new());
    /// let version_cmd_runner = VersionCmdRunner::new(&cfg_file);
    /// let params = VersionCmdParams::new();
    ///
    /// let versions = version_cmd_runner.list_jira_versions(params).await?;
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    pub async fn list_jira_versions(
        &self,
        params: VersionCmdParams,
    ) -> Result<Vec<Version>, Box<dyn std::error::Error>> {
        if Option::is_some(&params.versions_page_size) {
            match get_project_versions_paginated(
                &self.cfg,
                params.project.as_str(),
                params.versions_page_offset,
                params.versions_page_size,
                None,
                None,
                None,
                None,
            )
            .await?
            .values
            {
                Some(values) => Ok(values),
                None => Ok(vec![]),
            }
        } else {
            Ok(get_project_versions(&self.cfg, params.project.as_str(), None).await?)
        }
    }

    /// This method updates a Jira version with the given parameters
    /// and returns the updated version
    /// If the version is not found, it returns an error
    /// If the version ID is not given, it returns an error
    ///
    /// # Arguments
    ///
    /// * `params` - A VersionCmdParams struct
    ///
    /// # Returns
    ///
    /// * A Result containing a Version struct or an Error<UpdateVersionError>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jira_v3_openapi::models::Version;
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdParams;
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdRunner;
    /// use toml::Table;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # tokio_test::block_on(async {
    /// let cfg_file = ConfigFile::new("dXNlcm5hbWU6YXBpX2tleQ==".to_string(), "jira_url".to_string(), "standard_resolution".to_string(), "standard_resolution_comment".to_string(), Table::new());
    /// let version_cmd_runner = VersionCmdRunner::new(&cfg_file);
    /// let params = VersionCmdParams::new();
    ///
    /// let version = version_cmd_runner.update_jira_version(params).await?;
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    pub async fn update_jira_version(
        &self,
        params: VersionCmdParams,
    ) -> Result<Version, Error<UpdateVersionError>> {
        let release_date =
            if Option::is_some(&params.version_released) && params.version_released.unwrap() {
                if Option::is_some(&params.version_release_date) {
                    params.version_release_date
                } else {
                    Some(Utc::now().format("%Y-%m-%d").to_string())
                }
            } else {
                None
            };
        let version = Version {
            id: Some(params.version_id.clone().expect("VersionID is mandatory!")),
            name: params.version_name,
            description: params.version_description,
            start_date: params.version_start_date,
            release_date,
            archived: params.version_archived,
            released: params.version_released,
            ..Default::default()
        };
        update_version(
            &self.cfg,
            params.version_id.expect("VersionID is mandatory!").as_str(),
            version,
        )
        .await
    }

    /// This method deletes a Jira version with the given parameters
    /// and returns the status of the deletion
    ///
    /// # Arguments
    ///
    /// * `params` - A VersionCmdParams struct
    ///
    /// # Returns
    ///
    /// * A Result containing a serde_json::Value or an Error<DeleteAndReplaceVersionError>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdParams;
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdRunner;
    /// use toml::Table;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # tokio_test::block_on(async {
    /// let cfg_file = ConfigFile::new("dXNlcm5hbWU6YXBpX2tleQ==".to_string(), "jira_url".to_string(), "standard_resolution".to_string(), "standard_resolution_comment".to_string(), Table::new());
    /// let version_cmd_runner = VersionCmdRunner::new(&cfg_file);
    /// let params = VersionCmdParams::new();
    ///
    /// let status = version_cmd_runner.delete_jira_version(params).await?;
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    pub async fn delete_jira_version(
        &self,
        params: VersionCmdParams,
    ) -> Result<serde_json::Value, Error<DeleteAndReplaceVersionError>> {
        match delete_and_replace_version(
            &self.cfg,
            params.version_id.expect("VersionID is mandatory!").as_str(),
            DeleteAndReplaceVersionBean::new(),
        )
        .await
        {
            Ok(_) => Ok(serde_json::json!({"status": "success"})),
            Err(e) => match e {
                Error::Serde(_) => Ok(
                    serde_json::json!({"status": "success", "warning": "Version was deleted, some issues in deserializing response!"}),
                ),
                _ => Err(e),
            },
        }
    }

    /// This method retrieves the related work for a given version.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for the command.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `VersionRelatedWork` or an error.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdParams;
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdRunner;
    /// use toml::Table;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # tokio_test::block_on(async {
    /// let cfg_file = ConfigFile::new("dXNlcm5hbWU6YXBpX2tleQ==".to_string(), "jira_url".to_string(), "standard_resolution".to_string(), "standard_resolution_comment".to_string(), Table::new());
    /// let version_cmd_runner = VersionCmdRunner::new(&cfg_file);
    /// let params = VersionCmdParams::new();
    ///
    /// let items = version_cmd_runner.get_jira_version_related_work(params).await?;
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    pub async fn get_jira_version_related_work(
        &self,
        params: VersionCmdParams,
    ) -> Result<Vec<VersionRelatedWork>, Error<GetRelatedWorkError>> {
        get_related_work(
            &self.cfg,
            params.version_id.expect("VersionID is mandatory!").as_str(),
        )
        .await
    }

    /// Manage version related issues helper function
    /// Use FuturesUnordered to manage multiple operation concurrently
    ///
    /// # Arguments
    /// * `issue` - The issue to manage
    /// * `user_data` - The user data
    /// * `version` - The version
    ///
    /// # Returns
    /// A Result containing a tuple with the issue key, status, resolution, and fix version or an error
    ///
    /// # Example
    /// ```ignore
    /// use jira_v3_openapi::models::{ User, Version };
    /// use jirust_cli::config::config_file::ConfigFile;
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdRunner;
    /// use toml::Table;
    ///
    /// let cfg_file = ConfigFile::new("dXNlcm5hbWU6YXBpX2tleQ==".to_string(), "jira_url".to_string(), "standard_resolution".to_string(), "standard_resolution_comment".to_string(), Table::new());
    /// let version_cmd_runner = VersionCmdRunner::new(&cfg_file);
    /// let issue_key = "ABC-123";
    /// let user_data = Some(User::default());
    /// let version = Version::default();
    /// let result = self.manage_version_related_issues(issue_key, &user_data, &version).await;
    /// ```
    ///
    #[cfg(any(windows, unix))]
    async fn manage_version_related_issues(
        &self,
        issue: String,
        user_data: &Option<User>,
        version: &Version,
    ) -> Result<(String, String, String, String), Box<dyn std::error::Error>> {
        let all_transitions: Vec<IssueTransition> = match get_transitions(
            &self.cfg,
            issue.clone().as_str(),
            None,
            None,
            None,
            Some(false),
            None,
        )
        .await
        {
            Ok(transitions) => transitions.transitions.unwrap_or_default(),
            Err(_) => {
                return Ok((
                    issue,
                    "KO".to_string(),
                    "KO".to_string(),
                    "NO fixVersion set".to_string(),
                ));
            }
        };
        let transition_names: Vec<String> = self
            .resolution_transition_name
            .clone()
            .expect("Transition name is required and must be set in the config file");
        let resolve_transitions: Vec<IssueTransition> = all_transitions
            .into_iter()
            .filter(|t| transition_names.contains(&t.name.clone().unwrap_or("".to_string())))
            .collect();
        let transition_ids = resolve_transitions
            .into_iter()
            .map(|t| t.id.clone().unwrap_or("".to_string()))
            .collect::<Vec<String>>();
        let transitions = transition_ids
            .into_iter()
            .map(|id| {
                Some(IssueTransition {
                    id: Some(id),
                    ..Default::default()
                })
            })
            .collect::<Vec<Option<IssueTransition>>>();
        let mut update_fields_hashmap: HashMap<String, Vec<FieldUpdateOperation>> = HashMap::new();
        let mut transition_fields_hashmap: HashMap<String, Vec<FieldUpdateOperation>> =
            HashMap::new();
        let mut version_update_op = FieldUpdateOperation::new();
        let mut version_resolution_update_field = HashMap::new();
        let mut version_resolution_comment_op = FieldUpdateOperation::new();
        let version_json: Value =
            serde_json::from_str(serde_json::to_string(&version).unwrap().as_str())
                .unwrap_or(Value::Null);
        let resolution_value = self.resolution_value.clone();
        let comment_value = self.resolution_comment.clone();
        version_update_op.add = Some(Some(version_json));
        version_resolution_update_field.insert("resolution".to_string(), resolution_value);
        version_resolution_comment_op.add = Some(Some(comment_value));
        update_fields_hashmap.insert("fixVersions".to_string(), vec![version_update_op]);
        transition_fields_hashmap
            .insert("comment".to_string(), vec![version_resolution_comment_op]);
        let issue_update_data = IssueUpdateDetails {
            fields: None,
            history_metadata: None,
            properties: None,
            transition: None,
            update: Some(update_fields_hashmap),
        };
        let mut transition_result: String = "KO".to_string();
        if !Vec::is_empty(&transitions) {
            for transition in transitions {
                let issue_transition_data = IssueUpdateDetails {
                    fields: Some(version_resolution_update_field.clone()),
                    history_metadata: None,
                    properties: None,
                    transition: Some(transition.clone().unwrap()),
                    update: Some(transition_fields_hashmap.clone()),
                };
                match do_transition(&self.cfg, issue.clone().as_str(), issue_transition_data).await
                {
                    Ok(_) => {
                        transition_result = "OK".to_string();
                        break;
                    }
                    Err(Error::Serde(e)) => {
                        if e.is_eof() {
                            transition_result = "OK".to_string();
                            break;
                        } else {
                            transition_result = "KO".to_string()
                        }
                    }
                    Err(_) => transition_result = "KO".to_string(),
                }
            }
        }
        let assign_result: String = match assign_issue(
            &self.cfg,
            issue.clone().as_str(),
            user_data.clone().unwrap(),
        )
        .await
        {
            Ok(_) => "OK".to_string(),
            Err(Error::Serde(e)) => {
                if e.is_eof() {
                    "OK".to_string()
                } else {
                    "KO".to_string()
                }
            }
            Err(_) => "KO".to_string(),
        };
        let fixversion_result: String = match edit_issue(
            &self.cfg,
            issue.clone().as_str(),
            issue_update_data,
            Some(true),
            None,
            None,
            Some(true),
            None,
        )
        .await
        {
            Ok(_) => version.clone().name.unwrap_or("".to_string()),
            Err(_) => "NO fixVersion set".to_string(),
        };
        Ok((issue, transition_result, assign_result, fixversion_result))
    }
}

/// This struct defines the parameters for the Version commands
///
/// # Fields
///
/// * `project` - The project key, always **required**.
/// * `project_id` - The project ID, optional.
/// * `version_name` - The version name, optional.
/// * `version_id` - The version ID, **required** for archive, delete, release and update.
/// * `version_description` - The version description, optional.
/// * `version_start_date` - The version start date, optional (default: today on create command).
/// * `version_release_date` - The version release date, optional (default: today on release command).
/// * `version_archived` - The version archived status, optional.
/// * `version_released` - The version released status, optional.
/// * `changelog_file` - The changelog file path, to be used for automatic description generation (changelog-based), optional: if set the script detects automatically the first tagged block in the changelog and use it as description
/// * `resolve_issues` - The flag to resolve issues in the version, optional.
/// * `versions_page_size` - The page size for the version, optional.
/// * `versions_page_offset` - The page offset for the version, optional.
pub struct VersionCmdParams {
    pub project: String,
    pub project_id: Option<i64>,
    pub version_name: Option<String>,
    pub version_id: Option<String>,
    pub version_description: Option<String>,
    pub version_start_date: Option<String>,
    pub version_release_date: Option<String>,
    pub version_archived: Option<bool>,
    pub version_released: Option<bool>,
    pub changelog_file: Option<String>,
    pub transition_issues: Option<bool>,
    pub transition_assignee: Option<String>,
    pub versions_page_size: Option<i32>,
    pub versions_page_offset: Option<i64>,
}

/// Implementation of the VersionCmdParams struct
///
/// # Methods
///
/// * `new` - returns a new VersionCmdParams struct
/// * `merge_args` - merges the current version with the optional arguments
/// * `mark_released` - marks the version as released
/// * `mark_archived` - marks the version as archived
impl VersionCmdParams {
    /// This method returns a new VersionCmdParams struct
    ///
    /// # Returns
    ///
    /// * A VersionCmdParams struct
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdParams;
    ///
    /// let params = VersionCmdParams::new();
    /// ```
    pub fn new() -> VersionCmdParams {
        VersionCmdParams {
            project: "".to_string(),
            project_id: None,
            version_name: None,
            version_id: None,
            version_description: None,
            version_start_date: None,
            version_release_date: None,
            version_archived: None,
            version_released: None,
            changelog_file: None,
            transition_issues: None,
            transition_assignee: None,
            versions_page_size: None,
            versions_page_offset: None,
        }
    }

    /// This method merges the current version with the optional arguments
    /// and returns a VersionCmdParams struct
    /// If the optional arguments are not given, it uses the current version values
    ///
    /// # Arguments
    ///
    /// * `current_version` - A Version struct
    /// * `opt_args` - An Option<&VersionArgs> struct
    ///
    /// # Returns
    ///
    /// * A VersionCmdParams struct
    ///
    /// # Examples
    ///
    /// ```
    /// use jira_v3_openapi::models::Version;
    /// use jirust_cli::args::commands::{VersionArgs, VersionActionValues, PaginationArgs, OutputArgs};
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdParams;
    ///
    /// let mut current_version: Version = Version::new();
    /// current_version.id = Some("12345".to_string());
    /// current_version.project_id = Some(9876);
    /// current_version.project = Some("TEST_PROJECT".to_string());
    /// current_version.name = Some("v1.0".to_string());
    /// current_version.description = Some("This is the first version".to_string());
    ///
    /// let opt_args = VersionArgs {
    ///   version_act: VersionActionValues::List,
    ///   project_key: "project_key".to_string(),
    ///   project_id: None,
    ///   version_id: Some("97531".to_string()),
    ///   version_name: Some("version_name".to_string()),
    ///   version_description: Some("version_description".to_string()),
    ///   version_start_date: None,
    ///   version_release_date: None,
    ///   version_archived: None,
    ///   version_released: Some(true),
    ///   changelog_file: None,
    ///   pagination: PaginationArgs { page_size: None, page_offset: None },
    ///   output: OutputArgs { output_format: None, output_type: None },
    ///   transition_issues: None,
    ///   transition_assignee: None,
    /// };
    ///
    /// let params = VersionCmdParams::merge_args(current_version, Some(&opt_args));
    ///
    /// assert_eq!(params.project, "TEST_PROJECT".to_string());
    /// assert_eq!(params.project_id, Some(9876));
    /// assert_eq!(params.version_id, Some("12345".to_string()));
    /// assert_eq!(params.version_name, Some("version_name".to_string()));
    /// assert_eq!(params.version_description, Some("version_description".to_string()));
    /// assert_eq!(params.version_released, Some(true));
    /// ```
    pub fn merge_args(
        current_version: Version,
        opt_args: Option<&VersionArgs>,
    ) -> VersionCmdParams {
        match opt_args {
            Some(args) => VersionCmdParams {
                project: current_version.project.clone().unwrap_or("".to_string()),
                project_id: current_version.project_id,
                version_id: current_version.id,
                version_name: if Option::is_some(&args.version_name) {
                    args.version_name.clone()
                } else {
                    current_version.name
                },
                version_description: if Option::is_some(&args.version_description) {
                    args.version_description.clone()
                } else {
                    current_version.description
                },
                version_start_date: if Option::is_some(&args.version_start_date) {
                    args.version_start_date.clone()
                } else {
                    current_version.start_date
                },
                version_release_date: if Option::is_some(&args.version_release_date) {
                    args.version_release_date.clone()
                } else {
                    current_version.release_date
                },
                version_archived: if Option::is_some(&args.version_archived) {
                    args.version_archived
                } else {
                    current_version.archived
                },
                version_released: if Option::is_some(&args.version_released) {
                    args.version_released
                } else {
                    current_version.released
                },
                changelog_file: None,
                transition_issues: None,
                transition_assignee: None,
                versions_page_size: None,
                versions_page_offset: None,
            },
            None => VersionCmdParams {
                project: current_version.project.clone().unwrap_or("".to_string()),
                project_id: current_version.project_id,
                version_id: current_version.id,
                version_name: current_version.name,
                version_description: current_version.description,
                version_start_date: current_version.start_date,
                version_release_date: current_version.release_date,
                version_archived: current_version.archived,
                version_released: current_version.released,
                changelog_file: None,
                transition_issues: None,
                transition_assignee: None,
                versions_page_size: None,
                versions_page_offset: None,
            },
        }
    }

    /// This method marks the version as released
    /// and returns a VersionCmdParams struct
    /// It sets the version_released and version_release_date fields
    /// with the current date
    ///
    /// # Arguments
    ///
    /// * `version` - A Version struct
    ///
    /// # Returns
    ///
    /// * A VersionCmdParams struct
    ///
    /// # Examples
    ///
    /// ```
    /// use jira_v3_openapi::models::Version;
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdParams;
    ///
    /// let mut version: Version = Version::new();
    /// version.id = Some("12345".to_string());
    /// version.project_id = Some(9876);
    /// version.project = Some("TEST_PROJECT".to_string());
    /// version.name = Some("v1.0".to_string());
    /// version.description = Some("This is the first version".to_string());
    ///
    /// assert_eq!(version.released, None);
    ///
    /// let params = VersionCmdParams::mark_released(version);
    ///
    /// assert_eq!(params.version_released, Some(true));
    /// ```
    pub fn mark_released(version: Version) -> VersionCmdParams {
        let mut version_to_release = Self::merge_args(version, None);
        version_to_release.version_released = Some(true);
        version_to_release.version_release_date = Some(Utc::now().format("%Y-%m-%d").to_string());
        version_to_release
    }

    /// This method marks the version as archived
    /// and returns a VersionCmdParams struct
    ///
    /// # Arguments
    ///
    /// * `version` - A Version struct
    ///
    /// # Returns
    ///
    /// * A VersionCmdParams struct
    ///
    /// # Examples
    ///
    /// ```
    /// use jira_v3_openapi::models::Version;
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdParams;
    ///
    /// let mut version: Version = Version::new();
    /// version.id = Some("12345".to_string());
    /// version.project_id = Some(9876);
    /// version.project = Some("TEST_PROJECT".to_string());
    /// version.name = Some("v1.0".to_string());
    /// version.description = Some("This is the first version".to_string());
    ///
    /// assert_eq!(version.archived, None);
    ///
    /// let params = VersionCmdParams::mark_archived(version);
    ///
    /// assert_eq!(params.version_archived, Some(true));
    /// ```
    ///
    pub fn mark_archived(version: Version) -> VersionCmdParams {
        let mut version_to_archive = Self::merge_args(version, None);
        version_to_archive.version_archived = Some(true);
        version_to_archive
    }
}

/// Implementation of the From trait for the VersionArgs struct
/// This implementation allows the conversion of a VersionArgs struct to a VersionCmdParams struct.
impl From<&VersionArgs> for VersionCmdParams {
    /// This method converts the VersionArgs struct to a VersionCmdParams struct
    /// and returns a VersionCmdParams struct
    ///
    /// # Arguments
    ///
    /// * `args` - A VersionArgs struct
    ///
    /// # Returns
    ///
    /// * A VersionCmdParams struct
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::args::commands::{VersionActionValues, VersionArgs, PaginationArgs, OutputArgs};
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdParams;
    ///
    /// let version_args = VersionArgs {
    ///   version_act: VersionActionValues::List,
    ///   project_key: "project_key".to_string(),
    ///   project_id: None,
    ///   version_id: None,
    ///   version_name: Some("version_name".to_string()),
    ///   version_description: Some("version_description".to_string()),
    ///   version_start_date: None,
    ///   version_release_date: None,
    ///   version_archived: None,
    ///   version_released: None,
    ///   changelog_file: None,
    ///   pagination: PaginationArgs { page_size: Some(10), page_offset: Some(0) },
    ///   output: OutputArgs { output_format: None, output_type: None },
    ///   transition_issues: None,
    ///   transition_assignee: None,
    /// };
    ///
    /// let params = VersionCmdParams::from(&version_args);
    ///
    /// assert_eq!(params.project, "project_key".to_string());
    /// assert_eq!(params.version_name, Some("version_name".to_string()));
    /// assert_eq!(params.version_description, Some("version_description".to_string()));
    /// assert_eq!(params.versions_page_size, Some(10));
    /// assert_eq!(params.versions_page_offset, Some(0));
    /// ```
    fn from(args: &VersionArgs) -> Self {
        VersionCmdParams {
            project: args.project_key.clone(),
            project_id: args.project_id,
            version_name: args.version_name.clone(),
            version_id: args.version_id.clone(),
            version_description: args.version_description.clone(),
            version_start_date: Some(
                args.version_start_date
                    .clone()
                    .unwrap_or(Utc::now().format("%Y-%m-%d").to_string()),
            ),
            version_release_date: args.version_release_date.clone(),
            version_archived: args.version_archived,
            version_released: args.version_released,
            changelog_file: args.changelog_file.clone(),
            transition_issues: args.transition_issues,
            transition_assignee: args.transition_assignee.clone(),
            versions_page_size: args.pagination.page_size,
            versions_page_offset: args.pagination.page_offset,
        }
    }
}

/// Implementation of the Default trait for the VersionCmdParams struct
impl Default for VersionCmdParams {
    /// This method returns a VersionCmdParams struct with default values
    /// and returns a VersionCmdParams struct
    ///
    /// # Returns
    ///
    /// * A VersionCmdParams struct initialized with default values
    ///
    /// # Examples
    ///
    /// ```
    /// use jirust_cli::runners::jira_cmd_runners::version_cmd_runner::VersionCmdParams;
    ///
    /// let params = VersionCmdParams::default();
    ///
    /// assert_eq!(params.project, "".to_string());
    /// assert_eq!(params.project_id, None);
    /// assert_eq!(params.version_name, None);
    /// assert_eq!(params.version_id, None);
    /// assert_eq!(params.version_description, None);
    /// assert_eq!(params.version_start_date, None);
    /// assert_eq!(params.version_release_date, None);
    /// assert_eq!(params.version_archived, None);
    /// assert_eq!(params.version_released, None);
    /// assert_eq!(params.changelog_file, None);
    /// assert_eq!(params.transition_issues, None);
    /// assert_eq!(params.transition_assignee, None);
    /// assert_eq!(params.versions_page_size, None);
    /// assert_eq!(params.versions_page_offset, None);
    /// ```
    fn default() -> Self {
        VersionCmdParams::new()
    }
}
