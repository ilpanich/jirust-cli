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

/// TargetStatus : Status mapping for statuses in source workflow to respective target status in target workflow.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetStatus {
    /// An object with the key as the ID of the target status and value with the list of the IDs of the current source statuses.
    #[serde(rename = "statuses")]
    pub statuses: std::collections::HashMap<String, Vec<String>>,
}

impl TargetStatus {
    /// Status mapping for statuses in source workflow to respective target status in target workflow.
    pub fn new(statuses: std::collections::HashMap<String, Vec<String>>) -> TargetStatus {
        TargetStatus {
            statuses,
        }
    }
}

