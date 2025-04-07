# \AppPropertiesApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**addon_properties_resource_period_delete_addon_property_delete**](AppPropertiesApi.md#addon_properties_resource_period_delete_addon_property_delete) | **DELETE** /rest/atlassian-connect/1/addons/{addonKey}/properties/{propertyKey} | Delete app property
[**addon_properties_resource_period_get_addon_properties_get**](AppPropertiesApi.md#addon_properties_resource_period_get_addon_properties_get) | **GET** /rest/atlassian-connect/1/addons/{addonKey}/properties | Get app properties
[**addon_properties_resource_period_get_addon_property_get**](AppPropertiesApi.md#addon_properties_resource_period_get_addon_property_get) | **GET** /rest/atlassian-connect/1/addons/{addonKey}/properties/{propertyKey} | Get app property
[**addon_properties_resource_period_put_addon_property_put**](AppPropertiesApi.md#addon_properties_resource_period_put_addon_property_put) | **PUT** /rest/atlassian-connect/1/addons/{addonKey}/properties/{propertyKey} | Set app property
[**delete_forge_app_property**](AppPropertiesApi.md#delete_forge_app_property) | **DELETE** /rest/forge/1/app/properties/{propertyKey} | Delete app property (Forge)
[**put_forge_app_property**](AppPropertiesApi.md#put_forge_app_property) | **PUT** /rest/forge/1/app/properties/{propertyKey} | Set app property (Forge)



## addon_properties_resource_period_delete_addon_property_delete

> addon_properties_resource_period_delete_addon_property_delete(addon_key, property_key)
Delete app property

Deletes an app's property.  **[Permissions](#permissions) required:** Only a Connect app whose key matches `addonKey` can make this request. Additionally, Forge apps can access Connect app properties (stored against the same `app.connect.key`).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**addon_key** | **String** | The key of the app, as defined in its descriptor. | [required] |
**property_key** | **String** | The key of the property. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## addon_properties_resource_period_get_addon_properties_get

> models::PropertyKeys addon_properties_resource_period_get_addon_properties_get(addon_key)
Get app properties

Gets all the properties of an app.  **[Permissions](#permissions) required:** Only a Connect app whose key matches `addonKey` can make this request. Additionally, Forge apps can access Connect app properties (stored against the same `app.connect.key`).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**addon_key** | **String** | The key of the app, as defined in its descriptor. | [required] |

### Return type

[**models::PropertyKeys**](PropertyKeys.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## addon_properties_resource_period_get_addon_property_get

> models::EntityProperty addon_properties_resource_period_get_addon_property_get(addon_key, property_key)
Get app property

Returns the key and value of an app's property.  **[Permissions](#permissions) required:** Only a Connect app whose key matches `addonKey` can make this request. Additionally, Forge apps can access Connect app properties (stored against the same `app.connect.key`).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**addon_key** | **String** | The key of the app, as defined in its descriptor. | [required] |
**property_key** | **String** | The key of the property. | [required] |

### Return type

[**models::EntityProperty**](EntityProperty.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## addon_properties_resource_period_put_addon_property_put

> models::OperationMessage addon_properties_resource_period_put_addon_property_put(addon_key, property_key, body)
Set app property

Sets the value of an app's property. Use this resource to store custom data for your app.  The value of the request body must be a [valid](http://tools.ietf.org/html/rfc4627), non-empty JSON blob. The maximum length is 32768 characters.  **[Permissions](#permissions) required:** Only a Connect app whose key matches `addonKey` can make this request. Additionally, Forge apps can access Connect app properties (stored against the same `app.connect.key`).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**addon_key** | **String** | The key of the app, as defined in its descriptor. | [required] |
**property_key** | **String** | The key of the property. | [required] |
**body** | Option<**serde_json::Value**> |  | [required] |

### Return type

[**models::OperationMessage**](OperationMessage.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_forge_app_property

> delete_forge_app_property(property_key)
Delete app property (Forge)

Deletes a Forge app's property.  **[Permissions](#permissions) required:** Only Forge apps can make this request.  The new `write:app-data:jira` OAuth scope is 100% optional now, and not using it won't break your app. However, we recommend adding it to your app's scope list because we will eventually make it mandatory.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**property_key** | **String** | The key of the property. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_forge_app_property

> models::OperationMessage put_forge_app_property(property_key, body)
Set app property (Forge)

Sets the value of a Forge app's property. These values can be retrieved in [Jira expressions](/cloud/jira/platform/jira-expressions/) through the `app` [context variable](/cloud/jira/platform/jira-expressions/#context-variables). They are also available in [entity property display conditions](/platform/forge/manifest-reference/display-conditions/entity-property-conditions/).  For other use cases, use the [Storage API](/platform/forge/runtime-reference/storage-api/).  The value of the request body must be a [valid](http://tools.ietf.org/html/rfc4627), non-empty JSON blob. The maximum length is 32768 characters.  **[Permissions](#permissions) required:** Only Forge apps can make this request.  The new `write:app-data:jira` OAuth scope is 100% optional now, and not using it won't break your app. However, we recommend adding it to your app's scope list because we will eventually make it mandatory.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**property_key** | **String** | The key of the property. | [required] |
**body** | Option<**serde_json::Value**> |  | [required] |

### Return type

[**models::OperationMessage**](OperationMessage.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

