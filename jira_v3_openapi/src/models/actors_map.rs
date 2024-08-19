/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-6138e2cd7259ce8b18fff9d4a6cde29a4e1837b6
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActorsMap {
    /// The name of the group to add. This parameter cannot be used with the `groupId` parameter. As a group's name can change, use of `groupId` is recommended.
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<Vec<String>>,
    /// The ID of the group to add. This parameter cannot be used with the `group` parameter.
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<Vec<String>>,
    /// The user account ID of the user to add.
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Vec<String>>,
}

impl ActorsMap {
    pub fn new() -> ActorsMap {
        ActorsMap {
            group: None,
            group_id: None,
            user: None,
        }
    }
}

