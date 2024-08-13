/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-48dd1e2d75e7eac65741e465279d2700f927137d
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// StatusScope : The scope of the status.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusScope {
    #[serde(rename = "project", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub project: Option<Option<Box<models::ProjectId>>>,
    /// The scope of the status. `GLOBAL` for company-managed projects and `PROJECT` for team-managed projects.
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl StatusScope {
    /// The scope of the status.
    pub fn new(r#type: Type) -> StatusScope {
        StatusScope {
            project: None,
            r#type,
        }
    }
}
/// The scope of the status. `GLOBAL` for company-managed projects and `PROJECT` for team-managed projects.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "PROJECT")]
    Project,
    #[serde(rename = "GLOBAL")]
    Global,
}

impl Default for Type {
    fn default() -> Type {
        Self::Project
    }
}
