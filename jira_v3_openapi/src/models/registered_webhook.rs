/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-d0630ad8e2b33a2fc7347459a3397d94eb3a0393
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// RegisteredWebhook : ID of a registered webhook or error messages explaining why a webhook wasn't registered.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegisteredWebhook {
    /// The ID of the webhook. Returned if the webhook is created.
    #[serde(rename = "createdWebhookId", skip_serializing_if = "Option::is_none")]
    pub created_webhook_id: Option<i64>,
    /// Error messages specifying why the webhook creation failed.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
}

impl RegisteredWebhook {
    /// ID of a registered webhook or error messages explaining why a webhook wasn't registered.
    pub fn new() -> RegisteredWebhook {
        RegisteredWebhook {
            created_webhook_id: None,
            errors: None,
        }
    }
}

