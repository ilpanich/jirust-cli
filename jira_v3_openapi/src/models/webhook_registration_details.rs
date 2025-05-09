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

/// WebhookRegistrationDetails : Details of webhooks to register.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookRegistrationDetails {
    /// The URL that specifies where to send the webhooks. This URL must use the same base URL as the Connect app. Only a single URL per app is allowed to be registered.
    #[serde(rename = "url")]
    pub url: String,
    /// A list of webhooks.
    #[serde(rename = "webhooks")]
    pub webhooks: Vec<models::WebhookDetails>,
}

impl WebhookRegistrationDetails {
    /// Details of webhooks to register.
    pub fn new(url: String, webhooks: Vec<models::WebhookDetails>) -> WebhookRegistrationDetails {
        WebhookRegistrationDetails {
            url,
            webhooks,
        }
    }
}

