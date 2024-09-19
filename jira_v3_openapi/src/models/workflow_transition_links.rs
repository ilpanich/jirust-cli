/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-2776b5c6be42695cc76ed18bb9006304d509a541
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// WorkflowTransitionLinks : The statuses the transition can start from, and the mapping of ports between the statuses.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowTransitionLinks {
    /// The port that the transition starts from.
    #[serde(rename = "fromPort", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub from_port: Option<Option<i32>>,
    /// The status that the transition starts from.
    #[serde(rename = "fromStatusReference", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub from_status_reference: Option<Option<String>>,
    /// The port that the transition goes to.
    #[serde(rename = "toPort", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub to_port: Option<Option<i32>>,
}

impl WorkflowTransitionLinks {
    /// The statuses the transition can start from, and the mapping of ports between the statuses.
    pub fn new() -> WorkflowTransitionLinks {
        WorkflowTransitionLinks {
            from_port: None,
            from_status_reference: None,
            to_port: None,
        }
    }
}

