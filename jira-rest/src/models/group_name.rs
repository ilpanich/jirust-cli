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

/// GroupName : Details about a group.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupName {
    /// The ID of the group, which uniquely identifies the group across all Atlassian products. For example, *952d12c3-5b5b-4d04-bb32-44d383afc4b2*.
    #[serde(rename = "groupId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<Option<String>>,
    /// The name of group.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL for these group details.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
}

impl GroupName {
    /// Details about a group.
    pub fn new() -> GroupName {
        GroupName {
            group_id: None,
            name: None,
            param_self: None,
        }
    }
}

