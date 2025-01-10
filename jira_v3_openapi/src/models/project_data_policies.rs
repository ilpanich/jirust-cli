/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-4168ca0e5efc137ea4ff4f0cf29358bf64e26f02
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ProjectDataPolicies : Details about data policies for a list of projects.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectDataPolicies {
    /// List of projects with data policies.
    #[serde(rename = "projectDataPolicies", skip_serializing_if = "Option::is_none")]
    pub project_data_policies: Option<Vec<models::ProjectWithDataPolicy>>,
}

impl ProjectDataPolicies {
    /// Details about data policies for a list of projects.
    pub fn new() -> ProjectDataPolicies {
        ProjectDataPolicies {
            project_data_policies: None,
        }
    }
}

