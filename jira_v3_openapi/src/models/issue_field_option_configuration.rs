/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-2776b5c6be42695cc76ed18bb9006304d509a541
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// IssueFieldOptionConfiguration : Details of the projects the option is available in.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueFieldOptionConfiguration {
    /// DEPRECATED
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashSet<Attributes>>,
    /// Defines the projects that the option is available in. If the scope is not defined, then the option is available in all projects.
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<Box<models::IssueFieldOptionScopeBean>>,
}

impl IssueFieldOptionConfiguration {
    /// Details of the projects the option is available in.
    pub fn new() -> IssueFieldOptionConfiguration {
        IssueFieldOptionConfiguration {
            attributes: None,
            scope: None,
        }
    }
}
/// DEPRECATED
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Attributes {
    #[serde(rename = "notSelectable")]
    NotSelectable,
    #[serde(rename = "defaultValue")]
    DefaultValue,
}

impl Default for Attributes {
    fn default() -> Attributes {
        Self::NotSelectable
    }
}

