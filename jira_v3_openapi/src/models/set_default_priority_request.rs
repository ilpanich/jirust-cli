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

/// SetDefaultPriorityRequest : The new default issue priority.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SetDefaultPriorityRequest {
    /// The ID of the new default issue priority. Must be an existing ID or null. Setting this to null erases the default priority setting.
    #[serde(rename = "id")]
    pub id: String,
}

impl SetDefaultPriorityRequest {
    /// The new default issue priority.
    pub fn new(id: String) -> SetDefaultPriorityRequest {
        SetDefaultPriorityRequest {
            id,
        }
    }
}

