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

/// BulkChangeOwnerDetails : Details for changing owners of shareable entities
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BulkChangeOwnerDetails {
    /// Whether the name is fixed automatically if it's duplicated after changing owner.
    #[serde(rename = "autofixName")]
    pub autofix_name: bool,
    /// The account id of the new owner.
    #[serde(rename = "newOwner")]
    pub new_owner: String,
}

impl BulkChangeOwnerDetails {
    /// Details for changing owners of shareable entities
    pub fn new(autofix_name: bool, new_owner: String) -> BulkChangeOwnerDetails {
        BulkChangeOwnerDetails {
            autofix_name,
            new_owner,
        }
    }
}

