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
pub struct JiraUserField {
    #[serde(rename = "accountId")]
    pub account_id: String,
}

impl JiraUserField {
    pub fn new(account_id: String) -> JiraUserField {
        JiraUserField {
            account_id,
        }
    }
}

