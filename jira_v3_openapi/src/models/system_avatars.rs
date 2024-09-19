/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-2776b5c6be42695cc76ed18bb9006304d509a541
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// SystemAvatars : List of system avatars.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemAvatars {
    /// A list of avatar details.
    #[serde(rename = "system", skip_serializing_if = "Option::is_none")]
    pub system: Option<Vec<models::Avatar>>,
}

impl SystemAvatars {
    /// List of system avatars.
    pub fn new() -> SystemAvatars {
        SystemAvatars {
            system: None,
        }
    }
}

