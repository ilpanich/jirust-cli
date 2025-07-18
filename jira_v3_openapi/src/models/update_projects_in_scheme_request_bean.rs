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

/// UpdateProjectsInSchemeRequestBean : Update projects in a scheme
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateProjectsInSchemeRequestBean {
    /// Projects to add to a scheme
    #[serde(rename = "add", skip_serializing_if = "Option::is_none")]
    pub add: Option<Box<models::PrioritySchemeChangesWithoutMappings>>,
    /// Projects to remove from a scheme
    #[serde(rename = "remove", skip_serializing_if = "Option::is_none")]
    pub remove: Option<Box<models::PrioritySchemeChangesWithoutMappings>>,
}

impl UpdateProjectsInSchemeRequestBean {
    /// Update projects in a scheme
    pub fn new() -> UpdateProjectsInSchemeRequestBean {
        UpdateProjectsInSchemeRequestBean {
            add: None,
            remove: None,
        }
    }
}

