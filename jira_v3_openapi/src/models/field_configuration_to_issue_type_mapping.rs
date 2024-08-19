/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-6138e2cd7259ce8b18fff9d4a6cde29a4e1837b6
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// FieldConfigurationToIssueTypeMapping : The field configuration to issue type mapping.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldConfigurationToIssueTypeMapping {
    /// The ID of the field configuration.
    #[serde(rename = "fieldConfigurationId")]
    pub field_configuration_id: String,
    /// The ID of the issue type or *default*. When set to *default* this field configuration issue type item applies to all issue types without a field configuration. An issue type can be included only once in a request.
    #[serde(rename = "issueTypeId")]
    pub issue_type_id: String,
}

impl FieldConfigurationToIssueTypeMapping {
    /// The field configuration to issue type mapping.
    pub fn new(field_configuration_id: String, issue_type_id: String) -> FieldConfigurationToIssueTypeMapping {
        FieldConfigurationToIssueTypeMapping {
            field_configuration_id,
            issue_type_id,
        }
    }
}

