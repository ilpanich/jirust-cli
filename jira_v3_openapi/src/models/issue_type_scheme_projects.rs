/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-04504326a3bb85891591d523cc28ed61aefaa46b
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// IssueTypeSchemeProjects : Issue type scheme with a list of the projects that use it.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueTypeSchemeProjects {
    /// Details of an issue type scheme.
    #[serde(rename = "issueTypeScheme")]
    pub issue_type_scheme: Box<models::IssueTypeScheme>,
    /// The IDs of the projects using the issue type scheme.
    #[serde(rename = "projectIds")]
    pub project_ids: Vec<String>,
}

impl IssueTypeSchemeProjects {
    /// Issue type scheme with a list of the projects that use it.
    pub fn new(issue_type_scheme: models::IssueTypeScheme, project_ids: Vec<String>) -> IssueTypeSchemeProjects {
        IssueTypeSchemeProjects {
            issue_type_scheme: Box::new(issue_type_scheme),
            project_ids,
        }
    }
}

