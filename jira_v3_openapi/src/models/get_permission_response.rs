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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetPermissionResponse {
    /// The permission holder.
    #[serde(rename = "holder")]
    pub holder: Box<models::GetPermissionHolderResponse>,
    /// The permission type. This is \"View\" or \"Edit\".
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl GetPermissionResponse {
    pub fn new(holder: models::GetPermissionHolderResponse, r#type: Type) -> GetPermissionResponse {
        GetPermissionResponse {
            holder: Box::new(holder),
            r#type,
        }
    }
}
/// The permission type. This is \"View\" or \"Edit\".
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

