/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-48dd1e2d75e7eac65741e465279d2700f927137d
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// IssueEvent : Details about an issue event.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueEvent {
    /// The ID of the event.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The name of the event.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl IssueEvent {
    /// Details about an issue event.
    pub fn new() -> IssueEvent {
        IssueEvent {
            id: None,
            name: None,
        }
    }
}

