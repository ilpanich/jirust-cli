/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-4f9780d932de14e7c3dec0c9382f8855050fda22
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// StatusLayoutUpdate : The statuses associated with this workflow.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusLayoutUpdate {
    #[serde(rename = "approvalConfiguration", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub approval_configuration: Option<Option<Box<models::ApprovalConfiguration>>>,
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
            approval_configuration: None,
            layout: None,
            properties,
            status_reference,
        }
    }
}

