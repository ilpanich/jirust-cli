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

/// WorkflowStatusLayout : The x and y location of the status in the workflow.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowStatusLayout {
    /// The x axis location.
    #[serde(rename = "x", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub x: Option<Option<f64>>,
    /// The y axis location.
    #[serde(rename = "y", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub y: Option<Option<f64>>,
}

impl WorkflowStatusLayout {
    /// The x and y location of the status in the workflow.
    pub fn new() -> WorkflowStatusLayout {
        WorkflowStatusLayout {
            x: None,
            y: None,
        }
    }
}

