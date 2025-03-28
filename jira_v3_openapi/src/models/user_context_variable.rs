/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-4168ca0e5efc137ea4ff4f0cf29358bf64e26f02
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// UserContextVariable : A [user](https://developer.atlassian.com/cloud/jira/platform/jira-expressions-type-reference#user) specified as an Atlassian account ID.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserContextVariable {
    /// The account ID of the user.
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// Type of custom context variable.
    #[serde(rename = "type")]
    pub r#type: String,
}

impl UserContextVariable {
    /// A [user](https://developer.atlassian.com/cloud/jira/platform/jira-expressions-type-reference#user) specified as an Atlassian account ID.
    pub fn new(account_id: String, r#type: String) -> UserContextVariable {
        UserContextVariable {
            account_id,
            r#type,
        }
    }
}

