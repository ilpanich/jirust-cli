/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-48dd1e2d75e7eac65741e465279d2700f927137d
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// DefaultWorkflow : Details about the default workflow.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefaultWorkflow {
    /// Whether a draft workflow scheme is created or updated when updating an active workflow scheme. The draft is updated with the new default workflow. Defaults to `false`.
    #[serde(rename = "updateDraftIfNeeded", skip_serializing_if = "Option::is_none")]
    pub update_draft_if_needed: Option<bool>,
    /// The name of the workflow to set as the default workflow.
    #[serde(rename = "workflow")]
    pub workflow: String,
}

impl DefaultWorkflow {
    /// Details about the default workflow.
    pub fn new(workflow: String) -> DefaultWorkflow {
        DefaultWorkflow {
            update_draft_if_needed: None,
            workflow,
        }
    }
}
