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

/// Avatars : Details about system and custom avatars.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Avatars {
    /// Custom avatars list.
    #[serde(rename = "custom", skip_serializing_if = "Option::is_none")]
    pub custom: Option<Vec<models::Avatar>>,
    /// System avatars list.
    #[serde(rename = "system", skip_serializing_if = "Option::is_none")]
    pub system: Option<Vec<models::Avatar>>,
}

impl Avatars {
    /// Details about system and custom avatars.
    pub fn new() -> Avatars {
        Avatars {
            custom: None,
            system: None,
        }
    }
}

