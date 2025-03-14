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

/// StatusMigration : The mapping of old to new status ID.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusMigration {
    /// The new status ID.
    #[serde(rename = "newStatusReference")]
    pub new_status_reference: String,
    /// The old status ID.
    #[serde(rename = "oldStatusReference")]
    pub old_status_reference: String,
}

impl StatusMigration {
    /// The mapping of old to new status ID.
    pub fn new(new_status_reference: String, old_status_reference: String) -> StatusMigration {
        StatusMigration {
            new_status_reference,
            old_status_reference,
        }
    }
}

