# \FieldSchemesApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**associate_projects_to_field_association_schemes**](FieldSchemesApi.md#associate_projects_to_field_association_schemes) | **PUT** /rest/api/3/config/fieldschemes/projects | Associate projects to field schemes
[**create_field_association_scheme**](FieldSchemesApi.md#create_field_association_scheme) | **POST** /rest/api/3/config/fieldschemes | Create field scheme
[**delete_field_association_scheme**](FieldSchemesApi.md#delete_field_association_scheme) | **DELETE** /rest/api/3/config/fieldschemes/{id} | Delete a field scheme
[**get_field_association_scheme_by_id**](FieldSchemesApi.md#get_field_association_scheme_by_id) | **GET** /rest/api/3/config/fieldschemes/{id} | Get field scheme
[**get_field_association_scheme_item_parameters**](FieldSchemesApi.md#get_field_association_scheme_item_parameters) | **GET** /rest/api/3/config/fieldschemes/{id}/fields/{fieldId}/parameters | Get field parameters
[**get_field_association_schemes**](FieldSchemesApi.md#get_field_association_schemes) | **GET** /rest/api/3/config/fieldschemes | Get field schemes
[**get_projects_with_field_schemes**](FieldSchemesApi.md#get_projects_with_field_schemes) | **GET** /rest/api/3/config/fieldschemes/projects | Get projects with field schemes
[**remove_field_association_scheme_item_parameters**](FieldSchemesApi.md#remove_field_association_scheme_item_parameters) | **DELETE** /rest/api/3/config/fieldschemes/fields/parameters | Remove field parameters
[**remove_fields_associated_with_schemes**](FieldSchemesApi.md#remove_fields_associated_with_schemes) | **DELETE** /rest/api/3/config/fieldschemes/fields | Remove fields associated with field schemes
[**search_field_association_scheme_fields**](FieldSchemesApi.md#search_field_association_scheme_fields) | **GET** /rest/api/3/config/fieldschemes/{id}/fields | Search field scheme fields
[**search_field_association_scheme_projects**](FieldSchemesApi.md#search_field_association_scheme_projects) | **GET** /rest/api/3/config/fieldschemes/{id}/projects | Search field scheme projects
[**update_field_association_scheme**](FieldSchemesApi.md#update_field_association_scheme) | **PUT** /rest/api/3/config/fieldschemes/{id} | Update field scheme
[**update_field_association_scheme_item_parameters**](FieldSchemesApi.md#update_field_association_scheme_item_parameters) | **PUT** /rest/api/3/config/fieldschemes/fields/parameters | Update field parameters
[**update_fields_associated_with_schemes**](FieldSchemesApi.md#update_fields_associated_with_schemes) | **PUT** /rest/api/3/config/fieldschemes/fields | Update fields associated with field schemes



## associate_projects_to_field_association_schemes

> models::FieldSchemeToProjectsResponse associate_projects_to_field_association_schemes(request_body)
Associate projects to field schemes

Associate projects to field association schemes.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | [**std::collections::HashMap<String, models::FieldSchemeToProjectsRequest>**](FieldSchemeToProjectsRequest.md) |  | [required] |

### Return type

[**models::FieldSchemeToProjectsResponse**](FieldSchemeToProjectsResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_field_association_scheme

> models::CreateFieldAssociationSchemeResponse create_field_association_scheme(create_field_association_scheme_request)
Create field scheme

Endpoint for creating a new field association scheme.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_field_association_scheme_request** | [**CreateFieldAssociationSchemeRequest**](CreateFieldAssociationSchemeRequest.md) | The request containing the name and description of the field association scheme | [required] |

### Return type

[**models::CreateFieldAssociationSchemeResponse**](CreateFieldAssociationSchemeResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_field_association_scheme

> models::DeleteFieldAssociationSchemeResponse delete_field_association_scheme(id)
Delete a field scheme

Delete a specified field association scheme  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the field association scheme to delete. | [required] |

### Return type

[**models::DeleteFieldAssociationSchemeResponse**](DeleteFieldAssociationSchemeResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_field_association_scheme_by_id

> models::GetFieldAssociationSchemeByIdResponse get_field_association_scheme_by_id(id)
Get field scheme

Endpoint for fetching a field association scheme by its ID  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The scheme id to fetch | [required] |

### Return type

[**models::GetFieldAssociationSchemeByIdResponse**](GetFieldAssociationSchemeByIdResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_field_association_scheme_item_parameters

> models::GetFieldAssociationParametersResponse get_field_association_scheme_item_parameters(id, field_id)
Get field parameters

Retrieve field association parameters on a field association scheme  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | the ID of the field association scheme to retrieve parameters for | [required] |
**field_id** | **String** | the ID of the field | [required] |

### Return type

[**models::GetFieldAssociationParametersResponse**](GetFieldAssociationParametersResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_field_association_schemes

> models::PageBean2GetFieldAssociationSchemeResponse get_field_association_schemes(project_id, query, start_at, max_results)
Get field schemes

REST endpoint for retrieving a paginated list of field association schemes with optional filtering.  This endpoint allows clients to fetch field association schemes with optional filtering by project IDs and text queries. The response includes scheme details with navigation links and filter metadata when applicable.  Filtering Behavior:   *  When projectId or query parameters are provided, the response includes matchedFilters metadata showing which filters were applied.  *  When no filters are applied, matchedFilters is omitted from individual scheme objects  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | Option<[**Vec<i64>**](i64.md)> | (optional) List of project IDs to filter schemes by. If not provided, schemes from all projects are returned. |  |
**query** | Option<**String**> | (optional) Text filter for scheme name or description matching (case-insensitive). If not provided, no text filtering is applied. |  |
**start_at** | Option<**i64**> | Zero-based index of the first item to return (default: 0) |  |[default to 0]
**max_results** | Option<**i32**> | Maximum number of items to return per page (default: 50, max: 100) |  |[default to 50]

### Return type

[**models::PageBean2GetFieldAssociationSchemeResponse**](PageBean2GetFieldAssociationSchemeResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_projects_with_field_schemes

> models::PageBean2GetProjectsWithFieldSchemesResponse get_projects_with_field_schemes(project_id, start_at, max_results)
Get projects with field schemes

Get projects with field association schemes. This will be a temporary API but useful when transitioning from the legacy field configuration APIs to the new ones.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | [**Vec<i64>**](i64.md) | List of project ids to filter the results by. | [required] |
**start_at** | Option<**i64**> | The starting index of the returned projects. Base index: 0. |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of projects to return per page, maximum allowed value is 100. |  |[default to 50]

### Return type

[**models::PageBean2GetProjectsWithFieldSchemesResponse**](PageBean2GetProjectsWithFieldSchemesResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_field_association_scheme_item_parameters

> remove_field_association_scheme_item_parameters(request_body)
Remove field parameters

Remove field association parameters overrides for work types.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | [**std::collections::HashMap<String, Vec<models::ParameterRemovalDetails>>**](Vec.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_fields_associated_with_schemes

> models::MinimalFieldSchemeToFieldsResponse remove_fields_associated_with_schemes(request_body)
Remove fields associated with field schemes

Remove fields associated with field association schemes.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | [**std::collections::HashMap<String, models::RemoveFieldAssociationsRequestItem>**](RemoveFieldAssociationsRequestItem.md) | The request containing the schemes and fields to be removed. | [required] |

### Return type

[**models::MinimalFieldSchemeToFieldsResponse**](MinimalFieldSchemeToFieldsResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_field_association_scheme_fields

> models::PageBean2FieldAssociationSchemeFieldSearchResult search_field_association_scheme_fields(id, start_at, max_results, field_id)
Search field scheme fields

Search for fields belonging to a given field association scheme.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The scheme ID to search for child fields | [required] |
**start_at** | Option<**i64**> | The starting index of the returned fields. Base index: 0. |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of fields to return per page, maximum allowed value is 100. |  |[default to 50]
**field_id** | Option<[**Vec<String>**](String.md)> | The field IDs to filter by, if empty then all fields belonging to a field association scheme will be returned |  |

### Return type

[**models::PageBean2FieldAssociationSchemeFieldSearchResult**](PageBean2FieldAssociationSchemeFieldSearchResult.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_field_association_scheme_projects

> models::PageBean2FieldAssociationSchemeProjectSearchResult search_field_association_scheme_projects(id, start_at, max_results, project_id)
Search field scheme projects

REST Endpoint for searching for projects belonging to a given field association scheme  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The scheme id to search for associated projects | [required] |
**start_at** | Option<**i64**> | The starting index of the returned projects. Base index: 0. |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of projects to return per page, maximum allowed value is 100. |  |[default to 50]
**project_id** | Option<[**Vec<i64>**](i64.md)> | The project Ids to filter by, if empty then all projects belonging to a field association scheme will be returned |  |

### Return type

[**models::PageBean2FieldAssociationSchemeProjectSearchResult**](PageBean2FieldAssociationSchemeProjectSearchResult.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_field_association_scheme

> models::UpdateFieldAssociationSchemeResponse update_field_association_scheme(id, update_field_association_scheme_request)
Update field scheme

Endpoint for updating an existing field association scheme.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |
**update_field_association_scheme_request** | [**UpdateFieldAssociationSchemeRequest**](UpdateFieldAssociationSchemeRequest.md) | The request containing the desired updates to the field association scheme | [required] |

### Return type

[**models::UpdateFieldAssociationSchemeResponse**](UpdateFieldAssociationSchemeResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_field_association_scheme_item_parameters

> models::UpdateFieldSchemeParametersResponse update_field_association_scheme_item_parameters(request_body)
Update field parameters

Update field association item parameters in field association schemes.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | [**std::collections::HashMap<String, Vec<models::UpdateFieldSchemeParametersRequest>>**](Vec.md) | The request containing the field association scheme id and the parameters to update. | [required] |

### Return type

[**models::UpdateFieldSchemeParametersResponse**](UpdateFieldSchemeParametersResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_fields_associated_with_schemes

> models::FieldSchemeToFieldsResponse update_fields_associated_with_schemes(request_body)
Update fields associated with field schemes

Update fields associated with field association schemes.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | [**std::collections::HashMap<String, Vec<models::UpdateFieldAssociationsRequestItem>>**](Vec.md) | The request containing the schemes and work types to associate each field with. | [required] |

### Return type

[**models::FieldSchemeToFieldsResponse**](FieldSchemeToFieldsResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

