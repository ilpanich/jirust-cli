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

/// ProjectIssueCreateMetadata : Details of the issue creation metadata for a project.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectIssueCreateMetadata {
    /// List of the project's avatars, returning the avatar size and associated URL.
    #[serde(rename = "avatarUrls", skip_serializing_if = "Option::is_none")]
    pub avatar_urls: Option<Box<models::AvatarUrlsBean>>,
    /// Expand options that include additional project issue create metadata details in the response.
    #[serde(rename = "expand", skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    /// The ID of the project.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// List of the issue types supported by the project.
    #[serde(rename = "issuetypes", skip_serializing_if = "Option::is_none")]
    pub issuetypes: Option<Vec<models::IssueTypeIssueCreateMetadata>>,
    /// The key of the project.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The name of the project.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL of the project.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
}

impl ProjectIssueCreateMetadata {
    /// Details of the issue creation metadata for a project.
    pub fn new() -> ProjectIssueCreateMetadata {
        ProjectIssueCreateMetadata {
            avatar_urls: None,
            expand: None,
            id: None,
            issuetypes: None,
            key: None,
            name: None,
            param_self: None,
        }
    }
}

