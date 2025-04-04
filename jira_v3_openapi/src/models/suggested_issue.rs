/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-1461af1efd5cc75bf21cb8256a8b42f9bd4278be
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// SuggestedIssue : An issue suggested for use in the issue picker auto-completion.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuggestedIssue {
    /// The ID of the issue.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The URL of the issue type's avatar.
    #[serde(rename = "img", skip_serializing_if = "Option::is_none")]
    pub img: Option<String>,
    /// The key of the issue.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The key of the issue in HTML format.
    #[serde(rename = "keyHtml", skip_serializing_if = "Option::is_none")]
    pub key_html: Option<String>,
    /// The phrase containing the query string in HTML format, with the string highlighted with HTML bold tags.
    #[serde(rename = "summary", skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// The phrase containing the query string, as plain text.
    #[serde(rename = "summaryText", skip_serializing_if = "Option::is_none")]
    pub summary_text: Option<String>,
}

impl SuggestedIssue {
    /// An issue suggested for use in the issue picker auto-completion.
    pub fn new() -> SuggestedIssue {
        SuggestedIssue {
            id: None,
            img: None,
            key: None,
            key_html: None,
            summary: None,
            summary_text: None,
        }
    }
}

