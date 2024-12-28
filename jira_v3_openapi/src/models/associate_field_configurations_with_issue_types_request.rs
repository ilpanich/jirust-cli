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

/// AssociateFieldConfigurationsWithIssueTypesRequest : Details of a field configuration to issue type mappings.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssociateFieldConfigurationsWithIssueTypesRequest {
    /// Field configuration to issue type mappings.
    #[serde(rename = "mappings")]
    pub mappings: Vec<models::FieldConfigurationToIssueTypeMapping>,
}

impl AssociateFieldConfigurationsWithIssueTypesRequest {
    /// Details of a field configuration to issue type mappings.
    pub fn new(mappings: Vec<models::FieldConfigurationToIssueTypeMapping>) -> AssociateFieldConfigurationsWithIssueTypesRequest {
        AssociateFieldConfigurationsWithIssueTypesRequest {
            mappings,
        }
    }
}

