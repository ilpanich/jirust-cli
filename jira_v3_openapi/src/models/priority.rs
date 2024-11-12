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

/// Priority : An issue priority.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Priority {
    /// The avatarId of the avatar for the issue priority. This parameter is nullable and when set, this avatar references the universal avatar APIs.
    #[serde(rename = "avatarId", skip_serializing_if = "Option::is_none")]
    pub avatar_id: Option<i64>,
    /// The description of the issue priority.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The URL of the icon for the issue priority.
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// The ID of the issue priority.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether this priority is the default.
    #[serde(rename = "isDefault", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// The name of the issue priority.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Priority schemes associated with the issue priority.
    #[serde(rename = "schemes", skip_serializing_if = "Option::is_none")]
    pub schemes: Option<models::ExpandPrioritySchemePage>,
    /// The URL of the issue priority.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
    /// The color used to indicate the issue priority.
    #[serde(rename = "statusColor", skip_serializing_if = "Option::is_none")]
    pub status_color: Option<String>,
}

impl Priority {
    /// An issue priority.
    pub fn new() -> Priority {
        Priority {
            avatar_id: None,
            description: None,
            icon_url: None,
            id: None,
            is_default: None,
            name: None,
            schemes: None,
            param_self: None,
            status_color: None,
        }
    }
}

