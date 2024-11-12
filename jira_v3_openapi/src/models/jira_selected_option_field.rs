/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-af24ef23962debd9cc35cf020799e57ab332dd33
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JiraSelectedOptionField {
    #[serde(rename = "optionId", skip_serializing_if = "Option::is_none")]
    pub option_id: Option<i64>,
}

impl JiraSelectedOptionField {
    pub fn new() -> JiraSelectedOptionField {
        JiraSelectedOptionField {
            option_id: None,
        }
    }
}

