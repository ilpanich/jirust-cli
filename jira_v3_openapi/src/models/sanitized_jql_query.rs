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

/// SanitizedJqlQuery : Details of the sanitized JQL query.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SanitizedJqlQuery {
    /// The account ID of the user for whom sanitization was performed.
    #[serde(rename = "accountId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<Option<String>>,
    /// The list of errors.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Box<models::ErrorCollection>>,
    /// The initial query.
    #[serde(rename = "initialQuery", skip_serializing_if = "Option::is_none")]
    pub initial_query: Option<String>,
    /// The sanitized query, if there were no errors.
    #[serde(rename = "sanitizedQuery", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sanitized_query: Option<Option<String>>,
}

impl SanitizedJqlQuery {
    /// Details of the sanitized JQL query.
    pub fn new() -> SanitizedJqlQuery {
        SanitizedJqlQuery {
            account_id: None,
            errors: None,
            initial_query: None,
            sanitized_query: None,
        }
    }
}

