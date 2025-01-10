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

/// UpdateScreenSchemeDetails : Details of a screen scheme.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateScreenSchemeDetails {
    /// The description of the screen scheme. The maximum length is 255 characters.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the screen scheme. The name must be unique. The maximum length is 255 characters.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The IDs of the screens for the screen types of the screen scheme. Only screens used in classic projects are accepted.
    #[serde(rename = "screens", skip_serializing_if = "Option::is_none")]
    pub screens: Option<Box<models::UpdateScreenTypes>>,
}

impl UpdateScreenSchemeDetails {
    /// Details of a screen scheme.
    pub fn new() -> UpdateScreenSchemeDetails {
        UpdateScreenSchemeDetails {
            description: None,
            name: None,
            screens: None,
        }
    }
}

