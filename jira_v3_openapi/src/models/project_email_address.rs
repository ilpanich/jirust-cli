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

/// ProjectEmailAddress : A project's sender email address.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectEmailAddress {
    /// The email address.
    #[serde(rename = "emailAddress", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// When using a custom domain, the status of the email address.
    #[serde(rename = "emailAddressStatus", skip_serializing_if = "Option::is_none")]
    pub email_address_status: Option<Vec<String>>,
}

impl ProjectEmailAddress {
    /// A project's sender email address.
    pub fn new() -> ProjectEmailAddress {
        ProjectEmailAddress {
            email_address: None,
            email_address_status: None,
        }
    }
}

