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

/// BulkEditShareableEntityRequest : Details of a request to bulk edit shareable entity.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BulkEditShareableEntityRequest {
    /// Allowed action for bulk edit shareable entity
    #[serde(rename = "action")]
    pub action: Action,
    /// The details of change owner action.
    #[serde(rename = "changeOwnerDetails", skip_serializing_if = "Option::is_none")]
    pub change_owner_details: Option<Box<models::BulkChangeOwnerDetails>>,
    /// The id list of shareable entities to be changed.
    #[serde(rename = "entityIds")]
    pub entity_ids: Vec<i64>,
    /// Whether the actions are executed by users with Administer Jira global permission.
    #[serde(rename = "extendAdminPermissions", skip_serializing_if = "Option::is_none")]
    pub extend_admin_permissions: Option<bool>,
    /// The permission details to be changed.
    #[serde(rename = "permissionDetails", skip_serializing_if = "Option::is_none")]
    pub permission_details: Option<Box<models::PermissionDetails>>,
}

impl BulkEditShareableEntityRequest {
    /// Details of a request to bulk edit shareable entity.
    pub fn new(action: Action, entity_ids: Vec<i64>) -> BulkEditShareableEntityRequest {
        BulkEditShareableEntityRequest {
            action,
            change_owner_details: None,
            entity_ids,
            extend_admin_permissions: None,
            permission_details: None,
        }
    }
}
/// Allowed action for bulk edit shareable entity
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "changeOwner")]
    ChangeOwner,
    #[serde(rename = "changePermission")]
    ChangePermission,
    #[serde(rename = "addPermission")]
    AddPermission,
    #[serde(rename = "removePermission")]
    RemovePermission,
}

impl Default for Action {
    fn default() -> Action {
        Self::ChangeOwner
    }
}

