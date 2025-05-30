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
pub struct SimplifiedIssueTransition {
    /// The issue status change of the transition.
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<Box<models::IssueTransitionStatus>>,
    /// The unique ID of the transition.
    #[serde(rename = "transitionId", skip_serializing_if = "Option::is_none")]
    pub transition_id: Option<i32>,
    /// The name of the transition.
    #[serde(rename = "transitionName", skip_serializing_if = "Option::is_none")]
    pub transition_name: Option<String>,
}

impl SimplifiedIssueTransition {
    pub fn new() -> SimplifiedIssueTransition {
        SimplifiedIssueTransition {
            to: None,
            transition_id: None,
            transition_name: None,
        }
    }
}

