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

/// WorkflowScheme : Details about a workflow scheme.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowScheme {
    /// The name of the default workflow for the workflow scheme. The default workflow has *All Unassigned Issue Types* assigned to it in Jira. If `defaultWorkflow` is not specified when creating a workflow scheme, it is set to *Jira Workflow (jira)*.
    #[serde(rename = "defaultWorkflow", skip_serializing_if = "Option::is_none")]
    pub default_workflow: Option<String>,
    /// The description of the workflow scheme.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether the workflow scheme is a draft or not.
    #[serde(rename = "draft", skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    /// The ID of the workflow scheme.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The issue type to workflow mappings, where each mapping is an issue type ID and workflow name pair. Note that an issue type can only be mapped to one workflow in a workflow scheme.
    #[serde(rename = "issueTypeMappings", skip_serializing_if = "Option::is_none")]
    pub issue_type_mappings: Option<std::collections::HashMap<String, String>>,
    /// The issue types available in Jira.
    #[serde(rename = "issueTypes", skip_serializing_if = "Option::is_none")]
    pub issue_types: Option<std::collections::HashMap<String, models::IssueTypeDetails>>,
    /// The date-time that the draft workflow scheme was last modified. A modification is a change to the issue type-project mappings only. This property does not apply to non-draft workflows.
    #[serde(rename = "lastModified", skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    /// The user that last modified the draft workflow scheme. A modification is a change to the issue type-project mappings only. This property does not apply to non-draft workflows.
    #[serde(rename = "lastModifiedUser", skip_serializing_if = "Option::is_none")]
    pub last_modified_user: Option<Box<models::User>>,
    /// The name of the workflow scheme. The name must be unique. The maximum length is 255 characters. Required when creating a workflow scheme.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// For draft workflow schemes, this property is the name of the default workflow for the original workflow scheme. The default workflow has *All Unassigned Issue Types* assigned to it in Jira.
    #[serde(rename = "originalDefaultWorkflow", skip_serializing_if = "Option::is_none")]
    pub original_default_workflow: Option<String>,
    /// For draft workflow schemes, this property is the issue type to workflow mappings for the original workflow scheme, where each mapping is an issue type ID and workflow name pair. Note that an issue type can only be mapped to one workflow in a workflow scheme.
    #[serde(rename = "originalIssueTypeMappings", skip_serializing_if = "Option::is_none")]
    pub original_issue_type_mappings: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
    /// Whether to create or update a draft workflow scheme when updating an active workflow scheme. An active workflow scheme is a workflow scheme that is used by at least one project. The following examples show how this property works:   *  Update an active workflow scheme with `updateDraftIfNeeded` set to `true`: If a draft workflow scheme exists, it is updated. Otherwise, a draft workflow scheme is created.  *  Update an active workflow scheme with `updateDraftIfNeeded` set to `false`: An error is returned, as active workflow schemes cannot be updated.  *  Update an inactive workflow scheme with `updateDraftIfNeeded` set to `true`: The workflow scheme is updated, as inactive workflow schemes do not require drafts to update.  Defaults to `false`.
    #[serde(rename = "updateDraftIfNeeded", skip_serializing_if = "Option::is_none")]
    pub update_draft_if_needed: Option<bool>,
}

impl WorkflowScheme {
    /// Details about a workflow scheme.
    pub fn new() -> WorkflowScheme {
        WorkflowScheme {
            default_workflow: None,
            description: None,
            draft: None,
            id: None,
            issue_type_mappings: None,
            issue_types: None,
            last_modified: None,
            last_modified_user: None,
            name: None,
            original_default_workflow: None,
            original_issue_type_mappings: None,
            param_self: None,
            update_draft_if_needed: None,
        }
    }
}

