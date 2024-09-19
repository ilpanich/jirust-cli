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

/// ExportArchivedIssuesTaskProgressResponse : The response for status request for a running/completed export task.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportArchivedIssuesTaskProgressResponse {
    #[serde(rename = "fileUrl", skip_serializing_if = "Option::is_none")]
    pub file_url: Option<String>,
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
    #[serde(rename = "progress", skip_serializing_if = "Option::is_none")]
    pub progress: Option<i64>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "submittedTime", skip_serializing_if = "Option::is_none")]
    pub submitted_time: Option<String>,
    #[serde(rename = "taskId", skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

impl ExportArchivedIssuesTaskProgressResponse {
    /// The response for status request for a running/completed export task.
    pub fn new() -> ExportArchivedIssuesTaskProgressResponse {
        ExportArchivedIssuesTaskProgressResponse {
            file_url: None,
            payload: None,
            progress: None,
            status: None,
            submitted_time: None,
            task_id: None,
        }
    }
}

