# \ProjectTypesApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_accessible_project_type_by_key**](ProjectTypesApi.md#get_accessible_project_type_by_key) | **GET** /rest/api/3/project/type/{projectTypeKey}/accessible | Get accessible project type by key
[**get_all_accessible_project_types**](ProjectTypesApi.md#get_all_accessible_project_types) | **GET** /rest/api/3/project/type/accessible | Get licensed project types
[**get_all_project_types**](ProjectTypesApi.md#get_all_project_types) | **GET** /rest/api/3/project/type | Get all project types
[**get_project_type_by_key**](ProjectTypesApi.md#get_project_type_by_key) | **GET** /rest/api/3/project/type/{projectTypeKey} | Get project type by key



## get_accessible_project_type_by_key

> models::ProjectType get_accessible_project_type_by_key(project_type_key)
Get accessible project type by key

Returns a [project type](https://confluence.atlassian.com/x/Var1Nw) if it is accessible to the user.  **[Permissions](#permissions) required:** Permission to access Jira.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_type_key** | **String** | The key of the project type. | [required] |

### Return type

[**models::ProjectType**](ProjectType.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_accessible_project_types

> Vec<models::ProjectType> get_all_accessible_project_types()
Get licensed project types

Returns all [project types](https://confluence.atlassian.com/x/Var1Nw) with a valid license.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ProjectType>**](ProjectType.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_project_types

> Vec<models::ProjectType> get_all_project_types()
Get all project types

Returns all [project types](https://confluence.atlassian.com/x/Var1Nw), whether or not the instance has a valid license for each type.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** None.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ProjectType>**](ProjectType.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_type_by_key

> models::ProjectType get_project_type_by_key(project_type_key)
Get project type by key

Returns a [project type](https://confluence.atlassian.com/x/Var1Nw).  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** None.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_type_key** | **String** | The key of the project type. | [required] |

### Return type

[**models::ProjectType**](ProjectType.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

