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

/// IssuePickerSuggestionsIssueType : A type of issue suggested for use in auto-completion.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssuePickerSuggestionsIssueType {
    /// The ID of the type of issues suggested for use in auto-completion.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// A list of issues suggested for use in auto-completion.
    #[serde(rename = "issues", skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<models::SuggestedIssue>>,
    /// The label of the type of issues suggested for use in auto-completion.
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// If no issue suggestions are found, returns a message indicating no suggestions were found,
    #[serde(rename = "msg", skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    /// If issue suggestions are found, returns a message indicating the number of issues suggestions found and returned.
    #[serde(rename = "sub", skip_serializing_if = "Option::is_none")]
    pub sub: Option<String>,
}

impl IssuePickerSuggestionsIssueType {
    /// A type of issue suggested for use in auto-completion.
    pub fn new() -> IssuePickerSuggestionsIssueType {
        IssuePickerSuggestionsIssueType {
            id: None,
            issues: None,
            label: None,
            msg: None,
            sub: None,
        }
    }
}

