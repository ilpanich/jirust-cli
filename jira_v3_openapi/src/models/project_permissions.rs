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

/// ProjectPermissions : Permissions which a user has on a project.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectPermissions {
    /// Whether the logged user can edit the project.
    #[serde(rename = "canEdit", skip_serializing_if = "Option::is_none")]
    pub can_edit: Option<bool>,
}

impl ProjectPermissions {
    /// Permissions which a user has on a project.
    pub fn new() -> ProjectPermissions {
        ProjectPermissions {
            can_edit: None,
        }
    }
}

