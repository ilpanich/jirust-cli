# \DynamicModulesApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**dynamic_modules_resource_period_get_modules_get**](DynamicModulesApi.md#dynamic_modules_resource_period_get_modules_get) | **GET** /rest/atlassian-connect/1/app/module/dynamic | Get modules
[**dynamic_modules_resource_period_register_modules_post**](DynamicModulesApi.md#dynamic_modules_resource_period_register_modules_post) | **POST** /rest/atlassian-connect/1/app/module/dynamic | Register modules
[**dynamic_modules_resource_period_remove_modules_delete**](DynamicModulesApi.md#dynamic_modules_resource_period_remove_modules_delete) | **DELETE** /rest/atlassian-connect/1/app/module/dynamic | Remove modules



## dynamic_modules_resource_period_get_modules_get

> models::ConnectModules dynamic_modules_resource_period_get_modules_get()
Get modules

Returns all modules registered dynamically by the calling app.  **[Permissions](#permissions) required:** Only Connect apps can make this request.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ConnectModules**](ConnectModules.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dynamic_modules_resource_period_register_modules_post

> dynamic_modules_resource_period_register_modules_post(connect_modules)
Register modules

Registers a list of modules.  **[Permissions](#permissions) required:** Only Connect apps can make this request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connect_modules** | [**ConnectModules**](ConnectModules.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dynamic_modules_resource_period_remove_modules_delete

> dynamic_modules_resource_period_remove_modules_delete(module_key)
Remove modules

Remove all or a list of modules registered by the calling app.  **[Permissions](#permissions) required:** Only Connect apps can make this request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**module_key** | Option<[**Vec<String>**](String.md)> | The key of the module to remove. To include multiple module keys, provide multiple copies of this parameter. For example, `moduleKey=dynamic-attachment-entity-property&moduleKey=dynamic-select-field`. Nonexistent keys are ignored. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

