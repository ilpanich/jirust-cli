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

/// IssueTypeScreenScheme : Details of an issue type screen scheme.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueTypeScreenScheme {
    /// The description of the issue type screen scheme.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the issue type screen scheme.
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the issue type screen scheme.
    #[serde(rename = "name")]
    pub name: String,
}

impl IssueTypeScreenScheme {
    /// Details of an issue type screen scheme.
    pub fn new(id: String, name: String) -> IssueTypeScreenScheme {
        IssueTypeScreenScheme {
            description: None,
            id,
            name,
        }
    }
}
