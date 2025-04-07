/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-1461af1efd5cc75bf21cb8256a8b42f9bd4278be
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

#![cfg(feature = "issues_api")]

use crate::models;
use serde::{Deserialize, Serialize};

/// IssueBean : Details about an issue.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueBean {
    /// Details of changelogs associated with the issue.
    #[serde(rename = "changelog", skip_serializing_if = "Option::is_none")]
    pub changelog: Option<Box<models::PageOfChangelogs>>,
    /// The metadata for the fields on the issue that can be amended.
    #[serde(rename = "editmeta", skip_serializing_if = "Option::is_none")]
    pub editmeta: Option<Box<models::IssueUpdateMetadata>>,
    /// Expand options that include additional issue details in the response.
    #[serde(rename = "expand", skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "fieldsToInclude", skip_serializing_if = "Option::is_none")]
    pub fields_to_include: Option<Box<models::IncludedFields>>,
    /// The ID of the issue.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The key of the issue.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The ID and name of each field present on the issue.
    #[serde(rename = "names", skip_serializing_if = "Option::is_none")]
    pub names: Option<std::collections::HashMap<String, String>>,
    /// The operations that can be performed on the issue.
    #[serde(rename = "operations", skip_serializing_if = "Option::is_none")]
    pub operations: Option<models::Operations>,
    /// Details of the issue properties identified in the request.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// The rendered value of each field present on the issue.
    #[serde(rename = "renderedFields", skip_serializing_if = "Option::is_none")]
    pub rendered_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// The schema describing each field present on the issue.
    #[serde(rename = "schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<std::collections::HashMap<String, models::JsonTypeBean>>,
    /// The URL of the issue details.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
    /// The transitions that can be performed on the issue.
    #[serde(rename = "transitions", skip_serializing_if = "Option::is_none")]
    pub transitions: Option<Vec<models::IssueTransition>>,
    /// The versions of each field on the issue.
    #[serde(
        rename = "versionedRepresentations",
        skip_serializing_if = "Option::is_none"
    )]
    pub versioned_representations: Option<
        std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>,
    >,
}

impl IssueBean {
    /// Details about an issue.
    pub fn new() -> IssueBean {
        IssueBean {
            changelog: None,
            editmeta: None,
            expand: None,
            fields: None,
            fields_to_include: None,
            id: None,
            key: None,
            names: None,
            operations: None,
            properties: None,
            rendered_fields: None,
            schema: None,
            param_self: None,
            transitions: None,
            versioned_representations: None,
        }
    }
}
