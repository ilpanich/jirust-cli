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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowSchemeReadResponse {
    #[serde(rename = "defaultWorkflow", skip_serializing_if = "Option::is_none")]
    pub default_workflow: Option<Box<models::WorkflowMetadataRestModel>>,
    /// The description of the workflow scheme.
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// The ID of the workflow scheme.
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the workflow scheme.
    #[serde(rename = "name")]
    pub name: String,
    /// Deprecated. See the [deprecation notice](https://developer.atlassian.com/cloud/jira/platform/changelog/#CHANGE-2298) for details.  The IDs of projects using the workflow scheme.
    #[serde(rename = "projectIdsUsingScheme", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub project_ids_using_scheme: Option<Option<Vec<String>>>,
    #[serde(rename = "scope")]
    pub scope: Box<models::WorkflowScope>,
    /// Indicates if there's an [asynchronous task](#async-operations) for this workflow scheme.
    #[serde(rename = "taskId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub task_id: Option<Option<String>>,
    #[serde(rename = "version")]
    pub version: Box<models::DocumentVersion>,
    /// Mappings from workflows to issue types.
    #[serde(rename = "workflowsForIssueTypes")]
    pub workflows_for_issue_types: Vec<models::WorkflowMetadataAndIssueTypeRestModel>,
}

impl WorkflowSchemeReadResponse {
    pub fn new(id: String, name: String, scope: models::WorkflowScope, version: models::DocumentVersion, workflows_for_issue_types: Vec<models::WorkflowMetadataAndIssueTypeRestModel>) -> WorkflowSchemeReadResponse {
        WorkflowSchemeReadResponse {
            default_workflow: None,
            description: None,
            id,
            name,
            project_ids_using_scheme: None,
            scope: Box::new(scope),
            task_id: None,
            version: Box::new(version),
            workflows_for_issue_types,
        }
    }
}

