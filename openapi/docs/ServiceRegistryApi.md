# \ServiceRegistryApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**service_registry_resource_period_services_get**](ServiceRegistryApi.md#service_registry_resource_period_services_get) | **GET** /rest/atlassian-connect/1/service-registry | Retrieve the attributes of service registries



## service_registry_resource_period_services_get

> Vec<models::ServiceRegistry> service_registry_resource_period_services_get(service_ids)
Retrieve the attributes of service registries

Retrieve the attributes of given service registries.  **[Permissions](#permissions) required:** Only Connect apps can make this request and the servicesIds belong to the tenant you are requesting

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_ids** | [**Vec<String>**](String.md) | The ID of the services (the strings starting with \"b:\" need to be decoded in Base64). | [required] |

### Return type

[**Vec<models::ServiceRegistry>**](ServiceRegistry.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

