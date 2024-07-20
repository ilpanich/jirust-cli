# \ProjectCategoriesApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_project_category**](ProjectCategoriesApi.md#create_project_category) | **POST** /rest/api/3/projectCategory | Create project category
[**get_all_project_categories**](ProjectCategoriesApi.md#get_all_project_categories) | **GET** /rest/api/3/projectCategory | Get all project categories
[**get_project_category_by_id**](ProjectCategoriesApi.md#get_project_category_by_id) | **GET** /rest/api/3/projectCategory/{id} | Get project category by ID
[**remove_project_category**](ProjectCategoriesApi.md#remove_project_category) | **DELETE** /rest/api/3/projectCategory/{id} | Delete project category
[**update_project_category**](ProjectCategoriesApi.md#update_project_category) | **PUT** /rest/api/3/projectCategory/{id} | Update project category



## create_project_category

> models::ProjectCategory create_project_category(project_category)
Create project category

Creates a project category.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_category** | [**ProjectCategory**](ProjectCategory.md) |  | [required] |

### Return type

[**models::ProjectCategory**](ProjectCategory.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_project_categories

> Vec<models::ProjectCategory> get_all_project_categories()
Get all project categories

Returns all project categories.  **[Permissions](#permissions) required:** Permission to access Jira.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ProjectCategory>**](ProjectCategory.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_category_by_id

> models::ProjectCategory get_project_category_by_id(id)
Get project category by ID

Returns a project category.  **[Permissions](#permissions) required:** Permission to access Jira.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the project category. | [required] |

### Return type

[**models::ProjectCategory**](ProjectCategory.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_project_category

> remove_project_category(id)
Delete project category

Deletes a project category.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | ID of the project category to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_project_category

> models::UpdatedProjectCategory update_project_category(id, project_category)
Update project category

Updates a project category.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |
**project_category** | [**ProjectCategory**](ProjectCategory.md) |  | [required] |

### Return type

[**models::UpdatedProjectCategory**](UpdatedProjectCategory.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

