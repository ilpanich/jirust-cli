# \ProjectEmailApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_project_email**](ProjectEmailApi.md#get_project_email) | **GET** /rest/api/3/project/{projectId}/email | Get project's sender email
[**update_project_email**](ProjectEmailApi.md#update_project_email) | **PUT** /rest/api/3/project/{projectId}/email | Set project's sender email



## get_project_email

> models::ProjectEmailAddress get_project_email(project_id)
Get project's sender email

Returns the [project's sender email address](https://confluence.atlassian.com/x/dolKLg).  **[Permissions](#permissions) required:** *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **i64** | The project ID. | [required] |

### Return type

[**models::ProjectEmailAddress**](ProjectEmailAddress.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_project_email

> serde_json::Value update_project_email(project_id, project_email_address)
Set project's sender email

Sets the [project's sender email address](https://confluence.atlassian.com/x/dolKLg).  If `emailAddress` is an empty string, the default email address is restored.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) or *Administer Projects* [project permission.](https://confluence.atlassian.com/x/yodKLg)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **i64** | The project ID. | [required] |
**project_email_address** | [**ProjectEmailAddress**](ProjectEmailAddress.md) | The project's sender email address to be set. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

