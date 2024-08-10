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

/// PublishedWorkflowId : Properties that identify a published workflow.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublishedWorkflowId {
    /// The entity ID of the workflow.
    #[serde(rename = "entityId", skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    /// The name of the workflow.
    #[serde(rename = "name")]
    pub name: String,
}

impl PublishedWorkflowId {
    /// Properties that identify a published workflow.
    pub fn new(name: String) -> PublishedWorkflowId {
        PublishedWorkflowId {
            entity_id: None,
            name,
        }
    }
}

