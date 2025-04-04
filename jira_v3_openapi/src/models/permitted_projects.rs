/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-1461af1efd5cc75bf21cb8256a8b42f9bd4278be
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PermittedProjects : A list of projects in which a user is granted permissions.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PermittedProjects {
    /// A list of projects.
    #[serde(rename = "projects", skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<models::ProjectIdentifierBean>>,
}

impl PermittedProjects {
    /// A list of projects in which a user is granted permissions.
    pub fn new() -> PermittedProjects {
        PermittedProjects {
            projects: None,
        }
    }
}

