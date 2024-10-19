/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-d0630ad8e2b33a2fc7347459a3397d94eb3a0393
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// GroupDetails : Details about a group.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupDetails {
    /// The ID of the group, which uniquely identifies the group across all Atlassian products. For example, *952d12c3-5b5b-4d04-bb32-44d383afc4b2*.
    #[serde(rename = "groupId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<Option<String>>,
    /// The name of the group.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl GroupDetails {
    /// Details about a group.
    pub fn new() -> GroupDetails {
        GroupDetails {
            group_id: None,
            name: None,
        }
    }
}

