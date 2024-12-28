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
pub struct JiraRichTextField {
    #[serde(rename = "fieldId")]
    pub field_id: String,
    #[serde(rename = "richText")]
    pub rich_text: Box<models::JiraRichTextInput>,
}

impl JiraRichTextField {
    pub fn new(field_id: String, rich_text: models::JiraRichTextInput) -> JiraRichTextField {
        JiraRichTextField {
            field_id,
            rich_text: Box::new(rich_text),
        }
    }
}

