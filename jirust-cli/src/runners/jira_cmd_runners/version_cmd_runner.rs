use crate::args::commands::VersionArgs;
use crate::config::config_file::{AuthData, ConfigFile};
use chrono::{DateTime, Utc};
use openapi::apis::configuration::Configuration;
use openapi::apis::project_versions_api::*;
use openapi::apis::Error;
use openapi::models::{DeleteAndReplaceVersionBean, Version};

pub struct VersionCmdRunner {
    cfg: Configuration,
}

impl VersionCmdRunner {
    pub fn new(cfg_file: ConfigFile) -> VersionCmdRunner {
        let mut config = Configuration::new();
        let auth_data = AuthData::from_base64(cfg_file.get_auth_key());
        config.base_path = cfg_file.get_jira_url().to_string();
        config.basic_auth = Some((auth_data.0, Some(auth_data.1)));
        VersionCmdRunner { cfg: config }
    }

    pub async fn create_jira_version(
        &self,
        params: VersionCmdParams,
    ) -> Result<Version, Box<dyn std::error::Error>> {
        let version = Version {
            project: Some(params.project),
            name: Some(
                params
                    .version_name
                    .expect("VersionName is mandatory on cretion!"),
            ),
            description: params.version_description,
            start_date: params.version_start_date,
            release_date: params.version_release_date,
            archived: params.version_archived,
            released: params.version_released,
            ..Default::default()
        };
        Ok(create_version(&self.cfg, version).await?)
    }

    pub async fn get_jira_version(
        &self,
        params: VersionCmdParams,
    ) -> Result<Version, Error<GetVersionError>> {
        Ok(get_version(
            &self.cfg,
            params.version_id.expect("VersionID is mandatory!").as_str(),
            None,
        )
        .await?)
    }

    pub async fn list_jira_versions(
        &self,
        params: VersionCmdParams,
    ) -> Result<Vec<Version>, Box<dyn std::error::Error>> {
        if Option::is_some(&params.version_page_size) {
            match get_project_versions_paginated(
                &self.cfg,
                params.project.as_str(),
                params.version_page_offset,
                params.version_page_size,
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

    pub async fn update_jira_version(
        &self,
        params: VersionCmdParams,
    ) -> Result<Version, Error<UpdateVersionError>> {
        let version = openapi::models::Version {
            id: Some(params.version_id.clone().expect("VersionID is mandatory!")),
            name: params.version_name,
            description: params.version_description,
            start_date: params.version_start_date,
            release_date: params.version_release_date,
            archived: params.version_archived,
            released: params.version_released,
            ..Default::default()
        };
        Ok(update_version(
            &self.cfg,
            params.version_id.expect("VersionID is mandatory!").as_str(),
            version,
        )
        .await?)
    }

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
}

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
    pub version_page_size: Option<i32>,
    pub version_page_offset: Option<i64>,
}

impl VersionCmdParams {
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
            version_page_size: None,
            version_page_offset: None,
        }
    }

    pub fn merge_args(
        current_version: Version,
        opt_args: Option<&VersionArgs>,
    ) -> VersionCmdParams {
        match opt_args {
            Some(args) => VersionCmdParams {
                project: current_version.project.clone().unwrap_or("".to_string()),
                project_id: current_version.project_id.clone(),
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
                version_page_size: None,
                version_page_offset: None,
            },
            None => VersionCmdParams {
                project: current_version.project.clone().unwrap_or("".to_string()),
                project_id: current_version.project_id.clone(),
                version_id: current_version.id,
                version_name: current_version.name,
                version_description: current_version.description,
                version_start_date: current_version.start_date,
                version_release_date: current_version.release_date,
                version_archived: current_version.archived,
                version_released: current_version.released,
                version_page_size: None,
                version_page_offset: None,
            },
        }
    }

    pub fn mark_released(version: Version) -> VersionCmdParams {
        let mut version_to_release = Self::merge_args(version, None);
        version_to_release.version_released = Some(true);
        version_to_release.version_release_date = Some(Utc::now().format("%Y-%m-%d").to_string());
        version_to_release
    }

    pub fn mark_archived(version: Version) -> VersionCmdParams {
        let mut version_to_archive = Self::merge_args(version, None);
        version_to_archive.version_archived = Some(true);
        version_to_archive
    }
}

impl From<&VersionArgs> for VersionCmdParams {
    fn from(args: &VersionArgs) -> Self {
        let now: DateTime<Utc> = Utc::now();
        VersionCmdParams {
            project: args.project.clone(),
            project_id: args.project_id.clone(),
            version_name: args.version_name.clone(),
            version_id: args.version_id.clone(),
            version_description: args.version_description.clone(),
            version_start_date: Some(
                args.version_start_date
                    .clone()
                    .unwrap_or(now.format("%Y-%m-%d").to_string()),
            ),
            version_release_date: args.version_release_date.clone(),
            version_archived: args.version_archived.clone(),
            version_released: args.version_released.clone(),
            version_page_size: args.version_page_size.clone(),
            version_page_offset: args.version_page_offset.clone(),
        }
    }
}

pub fn print_table_full(versions: Vec<Version>) {
    let mut table = prettytable::Table::new();
    table.add_row(row![
        bFC->"Project ID",
        bFc->"ID",
        bFm->"Name",
        bFw->"Description",
        bFy->"Start Date",
        bFr->"Release Date",
        bFb->"Archived",
        bFg->"Released"
    ]);
    for version in versions {
        table.add_row(row![
            FC->version.project_id.unwrap_or_default(),
            Fc->version.id.unwrap_or_default(),
            Fm->version.name.unwrap_or_default(),
            Fw->version.description.unwrap_or_default(),
            Fy->version.start_date.unwrap_or_default(),
            Fr->version.release_date.unwrap_or_default(),
            Fb->version.archived.unwrap_or_default(),
            Fg->version.released.unwrap_or_default()
        ]);
    }
    table.printstd();
}

pub fn print_table_basic(versions: Vec<Version>) {
    let mut table = prettytable::Table::new();
    table.add_row(row![
        bFC->"Project ID",
        bFc->"ID",
        bFm->"Name",
        bFy->"Start Date",
        bFr->"Release Date",
        bFb->"Archived",
        bFg->"Released"
    ]);
    for version in versions {
        table.add_row(row![
            FC->version.project_id.unwrap_or_default(),
            Fc->version.id.unwrap_or_default(),
            Fm->version.name.unwrap_or_default(),
            Fy->version.start_date.unwrap_or_default(),
            Fr->version.release_date.unwrap_or_default(),
            Fb->version.archived.unwrap_or_default(),
            Fg->version.released.unwrap_or_default()
        ]);
    }
    table.printstd();
}

pub fn print_table_single(version: Version) {
    let mut table = prettytable::Table::new();
    table.add_row(row![
        FC->version.project_id.unwrap_or_default(),
        bFC->"Project ID",
        bFc->"ID",
        bFm->"Name",
        bFw->"Description",
        bFy->"Start Date",
        bFr->"Release Date",
        bFb->"Archived",
        bFg->"Released"
    ]);

    table.add_row(row![
        Fc->version.id.unwrap_or_default(),
        Fm->version.name.unwrap_or_default(),
        Fw->version.description.unwrap_or_default(),
        Fy->version.start_date.unwrap_or_default(),
        Fr->version.release_date.unwrap_or_default(),
        Fb->version.archived.unwrap_or_default(),
        Fg->version.released.unwrap_or_default()
    ]);

    table.printstd();
}
