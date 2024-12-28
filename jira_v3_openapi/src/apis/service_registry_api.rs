/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-04504326a3bb85891591d523cc28ed61aefaa46b
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`service_registry_resource_period_services_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServiceRegistryResourcePeriodServicesGetError {
    Status400(),
    Status401(),
    Status403(),
    Status500(),
    Status501(),
    Status504(),
    UnknownValue(serde_json::Value),
}


/// Retrieve the attributes of given service registries.  **[Permissions](#permissions) required:** Only Connect apps can make this request and the servicesIds belong to the tenant you are requesting
pub async fn service_registry_resource_period_services_get(configuration: &configuration::Configuration, service_ids: Vec<String>) -> Result<Vec<models::ServiceRegistry>, Error<ServiceRegistryResourcePeriodServicesGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/atlassian-connect/1/service-registry", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = match "multi" {
        "multi" => local_var_req_builder.query(&service_ids.into_iter().map(|p| ("serviceIds".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
        _ => local_var_req_builder.query(&[("serviceIds", &service_ids.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
    };
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ServiceRegistryResourcePeriodServicesGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

