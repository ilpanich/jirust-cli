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

/// Screen : A screen.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Screen {
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
}

impl Screen {
    /// A screen.
    pub fn new() -> Screen {
        Screen {
            description: None,
            id: None,
            name: None,
            scope: None,
        }
    }
}

