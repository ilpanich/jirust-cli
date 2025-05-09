/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-1461af1efd5cc75bf21cb8256a8b42f9bd4278be
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PublishDraftWorkflowScheme : Details about the status mappings for publishing a draft workflow scheme.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublishDraftWorkflowScheme {
    /// Mappings of statuses to new statuses for issue types.
    #[serde(rename = "statusMappings", skip_serializing_if = "Option::is_none")]
    pub status_mappings: Option<Vec<models::StatusMapping>>,
}

impl PublishDraftWorkflowScheme {
    /// Details about the status mappings for publishing a draft workflow scheme.
    pub fn new() -> PublishDraftWorkflowScheme {
        PublishDraftWorkflowScheme {
            status_mappings: None,
        }
    }
}

