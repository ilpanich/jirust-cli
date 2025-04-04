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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkIssueRequestJsonBean {
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<models::Comment>,
    #[serde(rename = "inwardIssue")]
    pub inward_issue: Box<models::LinkedIssue>,
    #[serde(rename = "outwardIssue")]
    pub outward_issue: Box<models::LinkedIssue>,
    #[serde(rename = "type")]
    pub r#type: Box<models::IssueLinkType>,
}

impl LinkIssueRequestJsonBean {
    pub fn new(inward_issue: models::LinkedIssue, outward_issue: models::LinkedIssue, r#type: models::IssueLinkType) -> LinkIssueRequestJsonBean {
        LinkIssueRequestJsonBean {
            comment: None,
            inward_issue: Box::new(inward_issue),
            outward_issue: Box::new(outward_issue),
            r#type: Box::new(r#type),
        }
    }
}

