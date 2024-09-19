/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-2776b5c6be42695cc76ed18bb9006304d509a541
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// IssueTypeScreenSchemesProjects : Issue type screen scheme with a list of the projects that use it.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueTypeScreenSchemesProjects {
    /// Details of an issue type screen scheme.
    #[serde(rename = "issueTypeScreenScheme")]
    pub issue_type_screen_scheme: Box<models::IssueTypeScreenScheme>,
    /// The IDs of the projects using the issue type screen scheme.
    #[serde(rename = "projectIds")]
    pub project_ids: Vec<String>,
}

impl IssueTypeScreenSchemesProjects {
    /// Issue type screen scheme with a list of the projects that use it.
    pub fn new(issue_type_screen_scheme: models::IssueTypeScreenScheme, project_ids: Vec<String>) -> IssueTypeScreenSchemesProjects {
        IssueTypeScreenSchemesProjects {
            issue_type_screen_scheme: Box::new(issue_type_screen_scheme),
            project_ids,
        }
    }
}

