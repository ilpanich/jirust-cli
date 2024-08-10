/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-48dd1e2d75e7eac65741e465279d2700f927137d
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// BulkPermissionsRequestBean : Details of global permissions to look up and project permissions with associated projects and issues to look up.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BulkPermissionsRequestBean {
    /// The account ID of a user.
    #[serde(rename = "accountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Global permissions to look up.
    #[serde(rename = "globalPermissions", skip_serializing_if = "Option::is_none")]
    pub global_permissions: Option<Vec<String>>,
    /// Project permissions with associated projects and issues to look up.
    #[serde(rename = "projectPermissions", skip_serializing_if = "Option::is_none")]
    pub project_permissions: Option<Vec<models::BulkProjectPermissions>>,
}

impl BulkPermissionsRequestBean {
    /// Details of global permissions to look up and project permissions with associated projects and issues to look up.
    pub fn new() -> BulkPermissionsRequestBean {
        BulkPermissionsRequestBean {
            account_id: None,
            global_permissions: None,
            project_permissions: None,
        }
    }
}

