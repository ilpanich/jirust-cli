/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-04504326a3bb85891591d523cc28ed61aefaa46b
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

