/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-6138e2cd7259ce8b18fff9d4a6cde29a4e1837b6
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// StatusLayoutUpdate : The statuses associated with this workflow.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusLayoutUpdate {
    #[serde(rename = "layout", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub layout: Option<Option<Box<models::WorkflowLayout>>>,
    /// The properties for this status layout.
    #[serde(rename = "properties")]
    pub properties: std::collections::HashMap<String, String>,
    /// A unique ID which the status will use to refer to this layout configuration.
    #[serde(rename = "statusReference")]
    pub status_reference: String,
}

impl StatusLayoutUpdate {
    /// The statuses associated with this workflow.
    pub fn new(properties: std::collections::HashMap<String, String>, status_reference: String) -> StatusLayoutUpdate {
        StatusLayoutUpdate {
            layout: None,
            properties,
            status_reference,
        }
    }
}

