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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SoftwareNavigationInfo {
    #[serde(rename = "boardId", skip_serializing_if = "Option::is_none")]
    pub board_id: Option<i64>,
    #[serde(rename = "boardName", skip_serializing_if = "Option::is_none")]
    pub board_name: Option<String>,
    #[serde(rename = "simpleBoard", skip_serializing_if = "Option::is_none")]
    pub simple_board: Option<bool>,
    #[serde(rename = "totalBoardsInProject", skip_serializing_if = "Option::is_none")]
    pub total_boards_in_project: Option<i64>,
}

impl SoftwareNavigationInfo {
    pub fn new() -> SoftwareNavigationInfo {
        SoftwareNavigationInfo {
            board_id: None,
            board_name: None,
            simple_board: None,
            total_boards_in_project: None,
        }
    }
}

