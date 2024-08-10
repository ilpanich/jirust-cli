# \ProjectKeyAndNameValidationApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_valid_project_key**](ProjectKeyAndNameValidationApi.md#get_valid_project_key) | **GET** /rest/api/3/projectvalidate/validProjectKey | Get valid project key
[**get_valid_project_name**](ProjectKeyAndNameValidationApi.md#get_valid_project_name) | **GET** /rest/api/3/projectvalidate/validProjectName | Get valid project name
[**validate_project_key**](ProjectKeyAndNameValidationApi.md#validate_project_key) | **GET** /rest/api/3/projectvalidate/key | Validate project key



## get_valid_project_key

> String get_valid_project_key(key)
Get valid project key

Validates a project key and, if the key is invalid or in use, generates a valid random string for the project key.  **[Permissions](#permissions) required:** None.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | Option<**String**> | The project key. |  |

### Return type

**String**

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_valid_project_name

> String get_valid_project_name(name)
Get valid project name

Checks that a project name isn't in use. If the name isn't in use, the passed string is returned. If the name is in use, this operation attempts to generate a valid project name based on the one supplied, usually by adding a sequence number. If a valid project name cannot be generated, a 404 response is returned.  **[Permissions](#permissions) required:** None.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The project name. | [required] |

### Return type

**String**

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_project_key

> models::ErrorCollection validate_project_key(key)
Validate project key

Validates a project key by confirming the key is a valid string and not in use.  **[Permissions](#permissions) required:** None.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | Option<**String**> | The project key. |  |

### Return type

[**models::ErrorCollection**](ErrorCollection.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

