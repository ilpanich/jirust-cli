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

/// MappingsByIssueTypeOverride : Overrides, for the selected issue types, any status mappings provided in `statusMappingsByWorkflows`. Status mappings are required when the new workflow for an issue type doesn't contain all statuses that the old workflow has. Status mappings can be provided by a combination of `statusMappingsByWorkflows` and `statusMappingsByIssueTypeOverride`.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MappingsByIssueTypeOverride {
    /// The ID of the issue type for this mapping.
    #[serde(rename = "issueTypeId")]
    pub issue_type_id: String,
    /// The list of status mappings.
    #[serde(rename = "statusMappings")]
    pub status_mappings: Vec<models::WorkflowAssociationStatusMapping>,
}

impl MappingsByIssueTypeOverride {
    /// Overrides, for the selected issue types, any status mappings provided in `statusMappingsByWorkflows`. Status mappings are required when the new workflow for an issue type doesn't contain all statuses that the old workflow has. Status mappings can be provided by a combination of `statusMappingsByWorkflows` and `statusMappingsByIssueTypeOverride`.
    pub fn new(issue_type_id: String, status_mappings: Vec<models::WorkflowAssociationStatusMapping>) -> MappingsByIssueTypeOverride {
        MappingsByIssueTypeOverride {
            issue_type_id,
            status_mappings,
        }
    }
}

