# \ProjectClassificationLevelsApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_default_project_classification**](ProjectClassificationLevelsApi.md#get_default_project_classification) | **GET** /rest/api/3/project/{projectIdOrKey}/classification-level/default | Get the default data classification level of a project
[**remove_default_project_classification**](ProjectClassificationLevelsApi.md#remove_default_project_classification) | **DELETE** /rest/api/3/project/{projectIdOrKey}/classification-level/default | Remove the default data classification level from a project
[**update_default_project_classification**](ProjectClassificationLevelsApi.md#update_default_project_classification) | **PUT** /rest/api/3/project/{projectIdOrKey}/classification-level/default | Update the default data classification level of a project



## get_default_project_classification

> serde_json::Value get_default_project_classification(project_id_or_key)
Get the default data classification level of a project

Returns the default data classification for a project.  **[Permissions](#permissions) required:**   *  *Browse Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project.  *  *Administer projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project.  *  *Administer jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id_or_key** | **String** | The project ID or project key (case-sensitive). | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_default_project_classification

> serde_json::Value remove_default_project_classification(project_id_or_key)
Remove the default data classification level from a project

Remove the default data classification level for a project.  **[Permissions](#permissions) required:**   *  *Administer projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project.  *  *Administer jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id_or_key** | **String** | The project ID or project key (case-sensitive). | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_default_project_classification

> serde_json::Value update_default_project_classification(project_id_or_key, update_default_project_classification_bean)
Update the default data classification level of a project

Updates the default data classification level for a project.  **[Permissions](#permissions) required:**   *  *Administer projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project.  *  *Administer jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id_or_key** | **String** | The project ID or project key (case-sensitive). | [required] |
**update_default_project_classification_bean** | [**UpdateDefaultProjectClassificationBean**](UpdateDefaultProjectClassificationBean.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

