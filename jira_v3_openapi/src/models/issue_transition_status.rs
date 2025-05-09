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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueTransitionStatus {
    /// The unique ID of the status.
    #[serde(rename = "statusId", skip_serializing_if = "Option::is_none")]
    pub status_id: Option<i32>,
    /// The name of the status.
    #[serde(rename = "statusName", skip_serializing_if = "Option::is_none")]
    pub status_name: Option<String>,
}

impl IssueTransitionStatus {
    pub fn new() -> IssueTransitionStatus {
        IssueTransitionStatus {
            status_id: None,
            status_name: None,
        }
    }
}

