/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-af24ef23962debd9cc35cf020799e57ab332dd33
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PermissionDetails : Details for permissions of shareable entities
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PermissionDetails {
    /// The edit permissions for the shareable entities.
    #[serde(rename = "editPermissions")]
    pub edit_permissions: Vec<models::SharePermission>,
    /// The share permissions for the shareable entities.
    #[serde(rename = "sharePermissions")]
    pub share_permissions: Vec<models::SharePermission>,
}

impl PermissionDetails {
    /// Details for permissions of shareable entities
    pub fn new(edit_permissions: Vec<models::SharePermission>, share_permissions: Vec<models::SharePermission>) -> PermissionDetails {
        PermissionDetails {
            edit_permissions,
            share_permissions,
        }
    }
}

