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

/// FoundGroup : A group found in a search.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FoundGroup {
    /// The ID of the group, which uniquely identifies the group across all Atlassian products. For example, *952d12c3-5b5b-4d04-bb32-44d383afc4b2*.
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// The group name with the matched query string highlighted with the HTML bold tag.
    #[serde(rename = "html", skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<models::GroupLabel>>,
    /// The name of the group. The name of a group is mutable, to reliably identify a group use ``groupId`.`
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl FoundGroup {
    /// A group found in a search.
    pub fn new() -> FoundGroup {
        FoundGroup {
            group_id: None,
            html: None,
            labels: None,
            name: None,
        }
    }
}

