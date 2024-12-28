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

/// StatusUpdateRequest : The list of statuses that will be updated.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusUpdateRequest {
    /// The list of statuses that will be updated.
    #[serde(rename = "statuses")]
    pub statuses: Vec<models::StatusUpdate>,
}

impl StatusUpdateRequest {
    /// The list of statuses that will be updated.
    pub fn new(statuses: Vec<models::StatusUpdate>) -> StatusUpdateRequest {
        StatusUpdateRequest {
            statuses,
        }
    }
}

