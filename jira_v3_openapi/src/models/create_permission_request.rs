/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-4168ca0e5efc137ea4ff4f0cf29358bf64e26f02
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreatePermissionRequest {
    /// The permission holder.
    #[serde(rename = "holder")]
    pub holder: Box<models::CreatePermissionHolderRequest>,
    /// The permission type. This must be \"View\" or \"Edit\".
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl CreatePermissionRequest {
    pub fn new(holder: models::CreatePermissionHolderRequest, r#type: Type) -> CreatePermissionRequest {
        CreatePermissionRequest {
            holder: Box::new(holder),
            r#type,
        }
    }
}
/// The permission type. This must be \"View\" or \"Edit\".
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "View")]
    View,
    #[serde(rename = "Edit")]
    Edit,
}

impl Default for Type {
    fn default() -> Type {
        Self::View
    }
}

