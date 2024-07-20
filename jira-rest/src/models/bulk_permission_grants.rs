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

