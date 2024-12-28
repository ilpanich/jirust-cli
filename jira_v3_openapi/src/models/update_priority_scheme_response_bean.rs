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

/// UpdatePrioritySchemeResponseBean : Details of the updated priority scheme.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdatePrioritySchemeResponseBean {
    #[serde(rename = "priorityScheme", skip_serializing_if = "Option::is_none")]
    pub priority_scheme: Option<models::PrioritySchemeWithPaginatedPrioritiesAndProjects>,
    /// The in-progress issue migration task.
    #[serde(rename = "task", skip_serializing_if = "Option::is_none")]
    pub task: Option<Box<models::TaskProgressBeanJsonNode>>,
}

impl UpdatePrioritySchemeResponseBean {
    /// Details of the updated priority scheme.
    pub fn new() -> UpdatePrioritySchemeResponseBean {
        UpdatePrioritySchemeResponseBean {
            priority_scheme: None,
            task: None,
        }
    }
}

