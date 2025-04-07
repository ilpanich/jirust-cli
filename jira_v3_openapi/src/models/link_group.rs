/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-1461af1efd5cc75bf21cb8256a8b42f9bd4278be
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// LinkGroup : Details a link group, which defines issue operations.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkGroup {
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<models::LinkGroup>>,
    #[serde(rename = "header", skip_serializing_if = "Option::is_none")]
    pub header: Option<Box<models::SimpleLink>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<models::SimpleLink>>,
    #[serde(rename = "styleClass", skip_serializing_if = "Option::is_none")]
    pub style_class: Option<String>,
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}

impl LinkGroup {
    /// Details a link group, which defines issue operations.
    pub fn new() -> LinkGroup {
        LinkGroup {
            groups: None,
            header: None,
            id: None,
            links: None,
            style_class: None,
            weight: None,
        }
    }
}

