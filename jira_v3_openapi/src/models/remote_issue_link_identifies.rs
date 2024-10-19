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

/// RemoteIssueLinkIdentifies : Details of the identifiers for a created or updated remote issue link.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoteIssueLinkIdentifies {
    /// The ID of the remote issue link, such as the ID of the item on the remote system.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The URL of the remote issue link.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
}

impl RemoteIssueLinkIdentifies {
    /// Details of the identifiers for a created or updated remote issue link.
    pub fn new() -> RemoteIssueLinkIdentifies {
        RemoteIssueLinkIdentifies {
            id: None,
            param_self: None,
        }
    }
}

