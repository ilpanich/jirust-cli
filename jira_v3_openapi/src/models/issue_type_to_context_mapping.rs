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

/// IssueTypeToContextMapping : Mapping of an issue type to a context.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueTypeToContextMapping {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// Whether the context is mapped to any issue type.
    #[serde(rename = "isAnyIssueType", skip_serializing_if = "Option::is_none")]
    pub is_any_issue_type: Option<bool>,
    /// The ID of the issue type.
    #[serde(rename = "issueTypeId", skip_serializing_if = "Option::is_none")]
    pub issue_type_id: Option<String>,
}

impl IssueTypeToContextMapping {
    /// Mapping of an issue type to a context.
    pub fn new(context_id: String) -> IssueTypeToContextMapping {
        IssueTypeToContextMapping {
            context_id,
            is_any_issue_type: None,
            issue_type_id: None,
        }
    }
}

