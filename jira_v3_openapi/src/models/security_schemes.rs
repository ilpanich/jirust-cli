/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-4f9780d932de14e7c3dec0c9382f8855050fda22
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// SecuritySchemes : List of security schemes.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecuritySchemes {
    /// List of security schemes.
    #[serde(rename = "issueSecuritySchemes", skip_serializing_if = "Option::is_none")]
    pub issue_security_schemes: Option<Vec<models::SecurityScheme>>,
}

impl SecuritySchemes {
    /// List of security schemes.
    pub fn new() -> SecuritySchemes {
        SecuritySchemes {
            issue_security_schemes: None,
        }
    }
}

