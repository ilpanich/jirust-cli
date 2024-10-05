/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-7dd452bf833f9d1d005d3d3d429c42e9aacb344e
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// BulkPermissionGrants : Details of global and project permissions granted to the user.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BulkPermissionGrants {
    /// List of permissions granted to the user.
    #[serde(rename = "globalPermissions")]
    pub global_permissions: Vec<String>,
    /// List of project permissions and the projects and issues those permissions provide access to.
    #[serde(rename = "projectPermissions")]
    pub project_permissions: Vec<models::BulkProjectPermissionGrants>,
}

impl BulkPermissionGrants {
    /// Details of global and project permissions granted to the user.
    pub fn new(global_permissions: Vec<String>, project_permissions: Vec<models::BulkProjectPermissionGrants>) -> BulkPermissionGrants {
        BulkPermissionGrants {
            global_permissions,
            project_permissions,
        }
    }
}

