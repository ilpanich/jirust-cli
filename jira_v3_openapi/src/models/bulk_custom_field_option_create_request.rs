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

/// BulkCustomFieldOptionCreateRequest : Details of the options to create for a custom field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BulkCustomFieldOptionCreateRequest {
    /// Details of options to create.
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<models::CustomFieldOptionCreate>>,
}

impl BulkCustomFieldOptionCreateRequest {
    /// Details of the options to create for a custom field.
    pub fn new() -> BulkCustomFieldOptionCreateRequest {
        BulkCustomFieldOptionCreateRequest {
            options: None,
        }
    }
}

