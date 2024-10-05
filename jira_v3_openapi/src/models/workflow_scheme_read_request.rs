/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-7dd452bf833f9d1d005d3d3d429c42e9aacb344e
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// WorkflowSchemeReadRequest : The workflow scheme read request body.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowSchemeReadRequest {
    /// The list of project IDs to query.
    #[serde(rename = "projectIds", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub project_ids: Option<Option<Vec<String>>>,
    /// The list of workflow scheme IDs to query.
    #[serde(rename = "workflowSchemeIds", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub workflow_scheme_ids: Option<Option<Vec<String>>>,
}

impl WorkflowSchemeReadRequest {
    /// The workflow scheme read request body.
    pub fn new() -> WorkflowSchemeReadRequest {
        WorkflowSchemeReadRequest {
            project_ids: None,
            workflow_scheme_ids: None,
        }
    }
}

