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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateIssueSecuritySchemeRequestBean {
    /// The description of the security scheme scheme.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the security scheme scheme. Must be unique.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdateIssueSecuritySchemeRequestBean {
    pub fn new() -> UpdateIssueSecuritySchemeRequestBean {
        UpdateIssueSecuritySchemeRequestBean {
            description: None,
            name: None,
        }
    }
}

