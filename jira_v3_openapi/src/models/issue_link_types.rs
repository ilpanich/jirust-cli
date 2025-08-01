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

/// IssueLinkTypes : A list of issue link type beans.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueLinkTypes {
    /// The issue link type bean.
    #[serde(rename = "issueLinkTypes", skip_serializing_if = "Option::is_none")]
    pub issue_link_types: Option<Vec<models::IssueLinkType>>,
}

impl IssueLinkTypes {
    /// A list of issue link type beans.
    pub fn new() -> IssueLinkTypes {
        IssueLinkTypes {
            issue_link_types: None,
        }
    }
}

