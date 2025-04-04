# \UsernavpropertiesApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_user_nav_property**](UsernavpropertiesApi.md#get_user_nav_property) | **GET** /rest/api/3/user/nav4-opt-property/{propertyKey} | Get user nav property
[**set_user_nav_property**](UsernavpropertiesApi.md#set_user_nav_property) | **PUT** /rest/api/3/user/nav4-opt-property/{propertyKey} | Set user nav property



## get_user_nav_property

> models::UserNavPropertyJsonBean get_user_nav_property(property_key, account_id)
Get user nav property

Returns the value of a user nav preference.  Note: This operation fetches the property key value directly from RbacClient.  **[Permissions](#permissions) required:**   *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg), to get a property from any user.  *  Access to Jira, to get a property from the calling user's record.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**property_key** | **String** | The key of the user's property. | [required] |
**account_id** | Option<**String**> | The account ID of the user, which uniquely identifies the user across all Atlassian products. For example, *5b10ac8d82e05b22cc7d4ef5*. |  |

### Return type

[**models::UserNavPropertyJsonBean**](UserNavPropertyJsonBean.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_user_nav_property

> serde_json::Value set_user_nav_property(property_key, body, account_id)
Set user nav property

Sets the value of a Nav4 preference. Use this resource to store Nav4 preference data against a user in the Identity service.  **[Permissions](#permissions) required:**   *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg), to set a property on any user.  *  Access to Jira, to set a property on the calling user's record.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**property_key** | **String** | The key of the nav property. The maximum length is 255 characters. | [required] |
**body** | Option<**serde_json::Value**> | The value of the property. The value has to be a boolean [JSON](https://tools.ietf.org/html/rfc4627) value. The maximum length of the property value is 32768 bytes. | [required] |
**account_id** | Option<**String**> | The account ID of the user, which uniquely identifies the user across all Atlassian products. For example, *5b10ac8d82e05b22cc7d4ef5*. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

