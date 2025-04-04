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

/// FieldAssociationsRequest : Details of field associations with projects.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldAssociationsRequest {
    /// Contexts to associate/unassociate the fields with.
    #[serde(rename = "associationContexts")]
    pub association_contexts: Vec<models::AssociationContextObject>,
    /// Fields to associate/unassociate with projects.
    #[serde(rename = "fields")]
    pub fields: Vec<models::FieldIdentifierObject>,
}

impl FieldAssociationsRequest {
    /// Details of field associations with projects.
    pub fn new(association_contexts: Vec<models::AssociationContextObject>, fields: Vec<models::FieldIdentifierObject>) -> FieldAssociationsRequest {
        FieldAssociationsRequest {
            association_contexts,
            fields,
        }
    }
}

