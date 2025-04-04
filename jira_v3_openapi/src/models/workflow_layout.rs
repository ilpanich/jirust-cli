/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-1461af1efd5cc75bf21cb8256a8b42f9bd4278be
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// WorkflowLayout : The starting point for the statuses in the workflow.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowLayout {
    /// The x axis location.
    #[serde(rename = "x", skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    /// The y axis location.
    #[serde(rename = "y", skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
}

impl WorkflowLayout {
    /// The starting point for the statuses in the workflow.
    pub fn new() -> WorkflowLayout {
        WorkflowLayout {
            x: None,
            y: None,
        }
    }
}

