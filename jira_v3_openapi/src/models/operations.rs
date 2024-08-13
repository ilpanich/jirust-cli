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

/// Operations : Details of the operations that can be performed on the issue.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operations {
    /// Details of the link groups defining issue operations.
    #[serde(rename = "linkGroups", skip_serializing_if = "Option::is_none")]
    pub link_groups: Option<Vec<models::LinkGroup>>,
}

impl Operations {
    /// Details of the operations that can be performed on the issue.
    pub fn new() -> Operations {
        Operations {
            link_groups: None,
        }
    }
}
