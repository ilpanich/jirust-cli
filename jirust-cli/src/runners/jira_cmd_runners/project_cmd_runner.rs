use crate::args::commands::ProjectArgs;
use crate::config::config_file::{AuthData, ConfigFile};
use jira_v3_openapi::apis::configuration::Configuration;
use jira_v3_openapi::apis::issues_api::{
    get_create_issue_meta_issue_type_id, get_create_issue_meta_issue_types,
};
use jira_v3_openapi::apis::projects_api::search_projects;
use jira_v3_openapi::models::{
    project::Project, FieldCreateMetadata, IssueTypeIssueCreateMetadata,
};

pub struct ProjectCmdRunner {
    cfg: Configuration,
}

impl ProjectCmdRunner {
    pub fn new(cfg_file: ConfigFile) -> ProjectCmdRunner {
        let mut config = Configuration::new();
        let auth_data = AuthData::from_base64(cfg_file.get_auth_key());
        config.base_path = cfg_file.get_jira_url().to_string();
        config.basic_auth = Some((auth_data.0, Some(auth_data.1)));
        ProjectCmdRunner { cfg: config }
    }

    pub async fn list_jira_projects(
        &self,
        params: ProjectCmdParams,
    ) -> Result<Vec<Project>, Box<dyn std::error::Error>> {
        let page_size = Some(params.project_page_size.unwrap_or(10));
        let page_offset = Some(i64::from(params.project_page_offset.unwrap_or(0)));
        match search_projects(
            &self.cfg,
            page_offset,
            page_size,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
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
    }

    pub async fn get_jira_project_issue_types(
        &self,
        params: ProjectCmdParams,
    ) -> Result<Vec<IssueTypeIssueCreateMetadata>, Box<dyn std::error::Error>> {
        let page_size = Some(params.project_page_size.unwrap_or(10));
        let page_offset = Some(params.project_page_offset.unwrap_or(0));
        match get_create_issue_meta_issue_types(
            &self.cfg,
            &params.project_key,
            page_offset,
            page_size,
        )
        .await?
        .issue_types
        {
            Some(issue_types) => Ok(issue_types),
            None => Ok(vec![]),
        }
    }

    pub async fn get_jira_project_issue_type_id(
        &self,
        params: ProjectCmdParams,
    ) -> Result<Vec<FieldCreateMetadata>, Box<dyn std::error::Error>> {
        let page_size = Some(params.project_page_size.unwrap_or(10));
        let page_offset = Some(params.project_page_offset.unwrap_or(0));
        match get_create_issue_meta_issue_type_id(
            &self.cfg,
            &params.project_key,
            &params.project_issue_type.expect("Issue Type is required!"),
            page_offset,
            page_size,
        )
        .await?
        .fields
        {
            Some(id) => Ok(id),
            None => Ok(vec![]),
        }
    }
}

/// This struct defines the parameters for the Project commands
pub struct ProjectCmdParams {
    pub project_key: String,
    pub project_issue_type: Option<String>,
    pub project_page_size: Option<i32>,
    pub project_page_offset: Option<i32>,
}

impl ProjectCmdParams {
    pub fn new() -> ProjectCmdParams {
        ProjectCmdParams {
            project_key: "".to_string(),
            project_issue_type: None,
            project_page_size: None,
            project_page_offset: None,
        }
    }
}

impl From<&ProjectArgs> for ProjectCmdParams {
    fn from(value: &ProjectArgs) -> Self {
        ProjectCmdParams {
            project_key: value.project_key.clone().unwrap_or("".to_string()),
            project_issue_type: value.project_issue_type.clone(),
            project_page_size: value.projects_page_size,
            project_page_offset: value.projects_page_offset,
        }
    }
}
