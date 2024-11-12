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

/// StatusDetails : A status.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusDetails {
    /// The description of the status.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The URL of the icon used to represent the status.
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// The ID of the status.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the status.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The scope of the field.
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<models::Scope>,
    /// The URL of the status.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
    /// The category assigned to the status.
    #[serde(rename = "statusCategory", skip_serializing_if = "Option::is_none")]
    pub status_category: Option<models::StatusCategory>,
}

impl StatusDetails {
    /// A status.
    pub fn new() -> StatusDetails {
        StatusDetails {
            description: None,
            icon_url: None,
            id: None,
            name: None,
            scope: None,
            param_self: None,
            status_category: None,
        }
    }
}

