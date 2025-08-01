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

/// UiModificationContextDetails : The details of a UI modification's context, which define where to activate the UI modification.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UiModificationContextDetails {
    /// The ID of the UI modification context.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether a context is available. For example, when a project is deleted the context becomes unavailable.
    #[serde(rename = "isAvailable", skip_serializing_if = "Option::is_none")]
    pub is_available: Option<bool>,
    /// The issue type ID of the context. Null is treated as a wildcard, meaning the UI modification will be applied to all issue types. Each UI modification context can have a maximum of one wildcard.
    #[serde(rename = "issueTypeId", skip_serializing_if = "Option::is_none")]
    pub issue_type_id: Option<String>,
    /// The project ID of the context. Null is treated as a wildcard, meaning the UI modification will be applied to all projects. Each UI modification context can have a maximum of one wildcard.
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    /// The view type of the context. Only `GIC`(Global Issue Create), `IssueView` and `IssueTransition` are supported. Null is treated as a wildcard, meaning the UI modification will be applied to all view types. Each UI modification context can have a maximum of one wildcard.
    #[serde(rename = "viewType", skip_serializing_if = "Option::is_none")]
    pub view_type: Option<ViewType>,
}

impl UiModificationContextDetails {
    /// The details of a UI modification's context, which define where to activate the UI modification.
    pub fn new() -> UiModificationContextDetails {
        UiModificationContextDetails {
            id: None,
            is_available: None,
            issue_type_id: None,
            project_id: None,
            view_type: None,
        }
    }
}
/// The view type of the context. Only `GIC`(Global Issue Create), `IssueView` and `IssueTransition` are supported. Null is treated as a wildcard, meaning the UI modification will be applied to all view types. Each UI modification context can have a maximum of one wildcard.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ViewType {
    #[serde(rename = "GIC")]
    Gic,
    #[serde(rename = "IssueView")]
    IssueView,
    #[serde(rename = "IssueTransition")]
    IssueTransition,
}

impl Default for ViewType {
    fn default() -> ViewType {
        Self::Gic
    }
}

