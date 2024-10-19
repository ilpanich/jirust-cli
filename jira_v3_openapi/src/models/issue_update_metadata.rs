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

/// IssueUpdateMetadata : A list of editable field details.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueUpdateMetadata {
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::HashMap<String, models::FieldMetadata>>,
}

impl IssueUpdateMetadata {
    /// A list of editable field details.
    pub fn new() -> IssueUpdateMetadata {
        IssueUpdateMetadata {
            fields: None,
        }
    }
}

