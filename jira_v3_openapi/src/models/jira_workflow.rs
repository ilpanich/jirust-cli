/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-af24ef23962debd9cc35cf020799e57ab332dd33
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// JiraWorkflow : Details of a workflow.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JiraWorkflow {
    /// The description of the workflow.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the workflow.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Indicates if the workflow can be edited.
    #[serde(rename = "isEditable", skip_serializing_if = "Option::is_none")]
    pub is_editable: Option<bool>,
    /// The name of the workflow.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<Box<models::WorkflowScope>>,
    #[serde(rename = "startPointLayout", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub start_point_layout: Option<Option<Box<models::WorkflowLayout>>>,
    /// The statuses referenced in this workflow.
    #[serde(rename = "statuses", skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<models::WorkflowReferenceStatus>>,
    /// If there is a current [asynchronous task](#async-operations) operation for this workflow.
    #[serde(rename = "taskId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub task_id: Option<Option<String>>,
    /// The transitions of the workflow. Note that a transition can have either the deprecated `to`/`from` fields or the `toStatusReference`/`links` fields, but never both nor a combination.
    #[serde(rename = "transitions", skip_serializing_if = "Option::is_none")]
    pub transitions: Option<Vec<models::WorkflowTransitions>>,
    /// Use the optional `workflows.usages` expand to get additional information about the projects and issue types associated with the requested workflows.
    #[serde(rename = "usages", skip_serializing_if = "Option::is_none")]
    pub usages: Option<Vec<models::ProjectIssueTypes>>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<Box<models::DocumentVersion>>,
}

impl JiraWorkflow {
    /// Details of a workflow.
    pub fn new() -> JiraWorkflow {
        JiraWorkflow {
            description: None,
            id: None,
            is_editable: None,
            name: None,
            scope: None,
            start_point_layout: None,
            statuses: None,
            task_id: None,
            transitions: None,
            usages: None,
            version: None,
        }
    }
}

