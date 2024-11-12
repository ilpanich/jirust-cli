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

/// StatusReferenceAndPort : The status reference and port that a transition is connected to.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusReferenceAndPort {
    /// The port this transition uses to connect to this status.
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// The reference of this status.
    #[serde(rename = "statusReference")]
    pub status_reference: String,
}

impl StatusReferenceAndPort {
    /// The status reference and port that a transition is connected to.
    pub fn new(status_reference: String) -> StatusReferenceAndPort {
        StatusReferenceAndPort {
            port: None,
            status_reference,
        }
    }
}

