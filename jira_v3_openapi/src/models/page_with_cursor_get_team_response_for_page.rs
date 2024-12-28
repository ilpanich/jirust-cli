/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-04504326a3bb85891591d523cc28ed61aefaa46b
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PageWithCursorGetTeamResponseForPage {
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "last", skip_serializing_if = "Option::is_none")]
    pub last: Option<bool>,
    #[serde(rename = "nextPageCursor", skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<models::GetTeamResponseForPage>>,
}

impl PageWithCursorGetTeamResponseForPage {
    pub fn new() -> PageWithCursorGetTeamResponseForPage {
        PageWithCursorGetTeamResponseForPage {
            cursor: None,
            last: None,
            next_page_cursor: None,
            size: None,
            total: None,
            values: None,
        }
    }
}

