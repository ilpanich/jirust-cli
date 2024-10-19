/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-d0630ad8e2b33a2fc7347459a3397d94eb3a0393
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// UpdatePrioritySchemeRequestBean : Details of a priority scheme.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdatePrioritySchemeRequestBean {
    /// The default priority of the scheme.
    #[serde(rename = "defaultPriorityId", skip_serializing_if = "Option::is_none")]
    pub default_priority_id: Option<i64>,
    /// The description of the priority scheme.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Instructions to migrate issues.
    #[serde(rename = "mappings", skip_serializing_if = "Option::is_none")]
    pub mappings: Option<Box<models::PriorityMapping>>,
    /// The name of the priority scheme. Must be unique.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The priorities in the scheme.
    #[serde(rename = "priorities", skip_serializing_if = "Option::is_none")]
    pub priorities: Option<models::UpdatePrioritiesInSchemeRequestBean>,
    /// The projects in the scheme.
    #[serde(rename = "projects", skip_serializing_if = "Option::is_none")]
    pub projects: Option<models::UpdateProjectsInSchemeRequestBean>,
}

impl UpdatePrioritySchemeRequestBean {
    /// Details of a priority scheme.
    pub fn new() -> UpdatePrioritySchemeRequestBean {
        UpdatePrioritySchemeRequestBean {
            default_priority_id: None,
            description: None,
            mappings: None,
            name: None,
            priorities: None,
            projects: None,
        }
    }
}

