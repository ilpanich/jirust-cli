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

/// ScreenWithTab : A screen with tab details.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScreenWithTab {
    /// The description of the screen.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the screen.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The name of the screen.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The scope of the screen.
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<models::Scope>,
    /// The tab for the screen.
    #[serde(rename = "tab", skip_serializing_if = "Option::is_none")]
    pub tab: Option<Box<models::ScreenableTab>>,
}

impl ScreenWithTab {
    /// A screen with tab details.
    pub fn new() -> ScreenWithTab {
        ScreenWithTab {
            description: None,
            id: None,
            name: None,
            scope: None,
            tab: None,
        }
    }
}

