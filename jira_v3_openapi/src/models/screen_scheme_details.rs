/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-4f9780d932de14e7c3dec0c9382f8855050fda22
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ScreenSchemeDetails : Details of a screen scheme.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScreenSchemeDetails {
    /// The description of the screen scheme. The maximum length is 255 characters.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the screen scheme. The name must be unique. The maximum length is 255 characters.
    #[serde(rename = "name")]
    pub name: String,
    /// The IDs of the screens for the screen types of the screen scheme. Only screens used in classic projects are accepted.
    #[serde(rename = "screens")]
    pub screens: Box<models::ScreenTypes>,
}

impl ScreenSchemeDetails {
    /// Details of a screen scheme.
    pub fn new(name: String, screens: models::ScreenTypes) -> ScreenSchemeDetails {
        ScreenSchemeDetails {
            description: None,
            name,
            screens: Box::new(screens),
        }
    }
}

