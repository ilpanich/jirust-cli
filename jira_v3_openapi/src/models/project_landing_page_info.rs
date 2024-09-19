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
pub struct ProjectLandingPageInfo {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "boardId", skip_serializing_if = "Option::is_none")]
    pub board_id: Option<i64>,
    #[serde(rename = "boardName", skip_serializing_if = "Option::is_none")]
    pub board_name: Option<String>,
    #[serde(rename = "projectKey", skip_serializing_if = "Option::is_none")]
    pub project_key: Option<String>,
    #[serde(rename = "projectType", skip_serializing_if = "Option::is_none")]
    pub project_type: Option<String>,
    #[serde(rename = "queueCategory", skip_serializing_if = "Option::is_none")]
    pub queue_category: Option<String>,
    #[serde(rename = "queueId", skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<i64>,
    #[serde(rename = "queueName", skip_serializing_if = "Option::is_none")]
    pub queue_name: Option<String>,
    #[serde(rename = "simpleBoard", skip_serializing_if = "Option::is_none")]
    pub simple_board: Option<bool>,
    #[serde(rename = "simplified", skip_serializing_if = "Option::is_none")]
    pub simplified: Option<bool>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ProjectLandingPageInfo {
    pub fn new() -> ProjectLandingPageInfo {
        ProjectLandingPageInfo {
            attributes: None,
            board_id: None,
            board_name: None,
            project_key: None,
            project_type: None,
            queue_category: None,
            queue_id: None,
            queue_name: None,
            simple_board: None,
            simplified: None,
            url: None,
        }
    }
}

