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

/// ChangeFilterOwner : The account ID of the new owner.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeFilterOwner {
    /// The account ID of the new owner.
    #[serde(rename = "accountId")]
    pub account_id: String,
}

impl ChangeFilterOwner {
    /// The account ID of the new owner.
    pub fn new(account_id: String) -> ChangeFilterOwner {
        ChangeFilterOwner {
            account_id,
        }
    }
}

