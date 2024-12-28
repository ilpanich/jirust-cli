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

/// WorkflowCreateRequest : The create workflows payload.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowCreateRequest {
    #[serde(rename = "scope")]
    pub scope: Box<models::WorkflowScope>,
    /// The statuses to associate with the workflows.
    #[serde(rename = "statuses")]
    pub statuses: Vec<models::WorkflowStatusUpdate>,
    /// The details of the workflows to create.
    #[serde(rename = "workflows")]
    pub workflows: Vec<models::WorkflowCreate>,
}

impl WorkflowCreateRequest {
    /// The create workflows payload.
    pub fn new(scope: models::WorkflowScope, statuses: Vec<models::WorkflowStatusUpdate>, workflows: Vec<models::WorkflowCreate>) -> WorkflowCreateRequest {
        WorkflowCreateRequest {
            scope: Box::new(scope),
            statuses,
            workflows,
        }
    }
}

