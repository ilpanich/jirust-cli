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

/// LinkedIssue : The ID or key of a linked issue.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkedIssue {
    /// The fields associated with the issue.
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Box<models::Fields>>,
    /// The ID of an issue. Required if `key` isn't provided.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The key of an issue. Required if `id` isn't provided.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The URL of the issue.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
}

impl LinkedIssue {
    /// The ID or key of a linked issue.
    pub fn new() -> LinkedIssue {
        LinkedIssue {
            fields: None,
            id: None,
            key: None,
            param_self: None,
        }
    }
}

