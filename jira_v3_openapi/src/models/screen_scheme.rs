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

/// ScreenScheme : A screen scheme.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScreenScheme {
    /// The description of the screen scheme.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the screen scheme.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Details of the issue type screen schemes associated with the screen scheme.
    #[serde(rename = "issueTypeScreenSchemes", skip_serializing_if = "Option::is_none")]
    pub issue_type_screen_schemes: Option<Box<models::PageBeanIssueTypeScreenScheme>>,
    /// The name of the screen scheme.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The IDs of the screens for the screen types of the screen scheme.
    #[serde(rename = "screens", skip_serializing_if = "Option::is_none")]
    pub screens: Option<Box<models::ScreenTypes>>,
}

impl ScreenScheme {
    /// A screen scheme.
    pub fn new() -> ScreenScheme {
        ScreenScheme {
            description: None,
            id: None,
            issue_type_screen_schemes: None,
            name: None,
            screens: None,
        }
    }
}

