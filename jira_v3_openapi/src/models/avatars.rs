/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-7dd452bf833f9d1d005d3d3d429c42e9aacb344e
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

