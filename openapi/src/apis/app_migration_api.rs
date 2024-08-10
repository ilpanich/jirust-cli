/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-48dd1e2d75e7eac65741e465279d2700f927137d
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`app_issue_field_value_update_resource_period_update_issue_fields_put`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppIssueFieldValueUpdateResourcePeriodUpdateIssueFieldsPutError {
    Status400(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`migration_resource_period_update_entity_properties_value_put`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MigrationResourcePeriodUpdateEntityPropertiesValuePutError {
    Status400(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`migration_resource_period_workflow_rule_search_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MigrationResourcePeriodWorkflowRuleSearchPostError {
    Status400(),
    Status403(),
    UnknownValue(serde_json::Value),
}


/// Updates the value of a custom field added by Connect apps on one or more issues. The values of up to 200 custom fields can be updated.  **[Permissions](#permissions) required:** Only Connect apps can make this request
pub async fn app_issue_field_value_update_resource_period_update_issue_fields_put(configuration: &configuration::Configuration, atlassian_transfer_id: &str, connect_custom_field_values: models::ConnectCustomFieldValues) -> Result<serde_json::Value, Error<AppIssueFieldValueUpdateResourcePeriodUpdateIssueFieldsPutError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/atlassian-connect/1/migration/field", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Atlassian-Transfer-Id", atlassian_transfer_id.to_string());
    local_var_req_builder = local_var_req_builder.json(&connect_custom_field_values);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AppIssueFieldValueUpdateResourcePeriodUpdateIssueFieldsPutError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates the values of multiple entity properties for an object, up to 50 updates per request. This operation is for use by Connect apps during app migration.
pub async fn migration_resource_period_update_entity_properties_value_put(configuration: &configuration::Configuration, atlassian_transfer_id: &str, entity_type: &str, entity_property_details: Vec<models::EntityPropertyDetails>) -> Result<(), Error<MigrationResourcePeriodUpdateEntityPropertiesValuePutError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/atlassian-connect/1/migration/properties/{entityType}", local_var_configuration.base_path, entityType=crate::apis::urlencode(entity_type));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Atlassian-Transfer-Id", atlassian_transfer_id.to_string());
    local_var_req_builder = local_var_req_builder.json(&entity_property_details);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<MigrationResourcePeriodUpdateEntityPropertiesValuePutError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns configurations for workflow transition rules migrated from server to cloud and owned by the calling Connect app.
pub async fn migration_resource_period_workflow_rule_search_post(configuration: &configuration::Configuration, atlassian_transfer_id: &str, workflow_rules_search: models::WorkflowRulesSearch) -> Result<models::WorkflowRulesSearchDetails, Error<MigrationResourcePeriodWorkflowRuleSearchPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/atlassian-connect/1/migration/workflow/rule/search", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Atlassian-Transfer-Id", atlassian_transfer_id.to_string());
    local_var_req_builder = local_var_req_builder.json(&workflow_rules_search);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<MigrationResourcePeriodWorkflowRuleSearchPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

