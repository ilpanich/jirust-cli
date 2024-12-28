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

/// Workflow : Details about a workflow.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Workflow {
    /// The creation date of the workflow.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// The description of the workflow.
    #[serde(rename = "description")]
    pub description: String,
    /// Whether the workflow has a draft version.
    #[serde(rename = "hasDraftWorkflow", skip_serializing_if = "Option::is_none")]
    pub has_draft_workflow: Option<bool>,
    #[serde(rename = "id")]
    pub id: Box<models::PublishedWorkflowId>,
    /// Whether this is the default workflow.
    #[serde(rename = "isDefault", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "operations", skip_serializing_if = "Option::is_none")]
    pub operations: Option<Box<models::WorkflowOperations>>,
    /// The projects the workflow is assigned to, through workflow schemes.
    #[serde(rename = "projects", skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<models::ProjectDetails>>,
    /// The workflow schemes the workflow is assigned to.
    #[serde(rename = "schemes", skip_serializing_if = "Option::is_none")]
    pub schemes: Option<Vec<models::WorkflowSchemeIdName>>,
    /// The statuses of the workflow.
    #[serde(rename = "statuses", skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<models::WorkflowStatus>>,
    /// The transitions of the workflow.
    #[serde(rename = "transitions", skip_serializing_if = "Option::is_none")]
    pub transitions: Option<Vec<models::Transition>>,
    /// The last edited date of the workflow.
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

impl Workflow {
    /// Details about a workflow.
    pub fn new(description: String, id: models::PublishedWorkflowId) -> Workflow {
        Workflow {
            created: None,
            description,
            has_draft_workflow: None,
            id: Box::new(id),
            is_default: None,
            operations: None,
            projects: None,
            schemes: None,
            statuses: None,
            transitions: None,
            updated: None,
        }
    }
}

