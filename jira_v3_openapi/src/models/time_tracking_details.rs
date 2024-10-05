/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-7dd452bf833f9d1d005d3d3d429c42e9aacb344e
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// TimeTrackingDetails : Time tracking details.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeTrackingDetails {
    /// The original estimate of time needed for this issue in readable format.
    #[serde(rename = "originalEstimate", skip_serializing_if = "Option::is_none")]
    pub original_estimate: Option<String>,
    /// The original estimate of time needed for this issue in seconds.
    #[serde(rename = "originalEstimateSeconds", skip_serializing_if = "Option::is_none")]
    pub original_estimate_seconds: Option<i64>,
    /// The remaining estimate of time needed for this issue in readable format.
    #[serde(rename = "remainingEstimate", skip_serializing_if = "Option::is_none")]
    pub remaining_estimate: Option<String>,
    /// The remaining estimate of time needed for this issue in seconds.
    #[serde(rename = "remainingEstimateSeconds", skip_serializing_if = "Option::is_none")]
    pub remaining_estimate_seconds: Option<i64>,
    /// Time worked on this issue in readable format.
    #[serde(rename = "timeSpent", skip_serializing_if = "Option::is_none")]
    pub time_spent: Option<String>,
    /// Time worked on this issue in seconds.
    #[serde(rename = "timeSpentSeconds", skip_serializing_if = "Option::is_none")]
    pub time_spent_seconds: Option<i64>,
}

impl TimeTrackingDetails {
    /// Time tracking details.
    pub fn new() -> TimeTrackingDetails {
        TimeTrackingDetails {
            original_estimate: None,
            original_estimate_seconds: None,
            remaining_estimate: None,
            remaining_estimate_seconds: None,
            time_spent: None,
            time_spent_seconds: None,
        }
    }
}

