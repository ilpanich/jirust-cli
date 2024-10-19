/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-d0630ad8e2b33a2fc7347459a3397d94eb3a0393
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// IssueLink : Details of a link between issues.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueLink {
    /// The ID of the issue link.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Provides details about the linked issue. If presenting this link in a user interface, use the `inward` field of the issue link type to label the link.
    #[serde(rename = "inwardIssue")]
    pub inward_issue: Box<models::LinkedIssue>,
    /// Provides details about the linked issue. If presenting this link in a user interface, use the `outward` field of the issue link type to label the link.
    #[serde(rename = "outwardIssue")]
    pub outward_issue: Box<models::LinkedIssue>,
    /// The URL of the issue link.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
    /// The type of link between the issues.
    #[serde(rename = "type")]
    pub r#type: Box<models::IssueLinkType>,
}

impl IssueLink {
    /// Details of a link between issues.
    pub fn new(inward_issue: models::LinkedIssue, outward_issue: models::LinkedIssue, r#type: models::IssueLinkType) -> IssueLink {
        IssueLink {
            id: None,
            inward_issue: Box::new(inward_issue),
            outward_issue: Box::new(outward_issue),
            param_self: None,
            r#type: Box::new(r#type),
        }
    }
}

