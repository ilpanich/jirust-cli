# \PrioritySchemesApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_priority_scheme**](PrioritySchemesApi.md#create_priority_scheme) | **POST** /rest/api/3/priorityscheme | Create priority scheme
[**delete_priority_scheme**](PrioritySchemesApi.md#delete_priority_scheme) | **DELETE** /rest/api/3/priorityscheme/{schemeId} | Delete priority scheme
[**get_available_priorities_by_priority_scheme**](PrioritySchemesApi.md#get_available_priorities_by_priority_scheme) | **GET** /rest/api/3/priorityscheme/priorities/available | Get available priorities by priority scheme
[**get_priorities_by_priority_scheme**](PrioritySchemesApi.md#get_priorities_by_priority_scheme) | **GET** /rest/api/3/priorityscheme/{schemeId}/priorities | Get priorities by priority scheme
[**get_priority_schemes**](PrioritySchemesApi.md#get_priority_schemes) | **GET** /rest/api/3/priorityscheme | Get priority schemes
[**get_projects_by_priority_scheme**](PrioritySchemesApi.md#get_projects_by_priority_scheme) | **GET** /rest/api/3/priorityscheme/{schemeId}/projects | Get projects by priority scheme
[**suggested_priorities_for_mappings**](PrioritySchemesApi.md#suggested_priorities_for_mappings) | **POST** /rest/api/3/priorityscheme/mappings | Suggested priorities for mappings
[**update_priority_scheme**](PrioritySchemesApi.md#update_priority_scheme) | **PUT** /rest/api/3/priorityscheme/{schemeId} | Update priority scheme



## create_priority_scheme

> models::PrioritySchemeId create_priority_scheme(create_priority_scheme_details)
Create priority scheme

Creates a new priority scheme.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_priority_scheme_details** | [**CreatePrioritySchemeDetails**](CreatePrioritySchemeDetails.md) |  | [required] |

### Return type

[**models::PrioritySchemeId**](PrioritySchemeId.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_priority_scheme

> serde_json::Value delete_priority_scheme(scheme_id)
Delete priority scheme

Deletes a priority scheme.  This operation is only available for priority schemes without any associated projects. Any associated projects must be removed from the priority scheme before this operation can be performed.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scheme_id** | **i64** | The priority scheme ID. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_available_priorities_by_priority_scheme

> models::PageBeanPriorityWithSequence get_available_priorities_by_priority_scheme(scheme_id, start_at, max_results, query, exclude)
Get available priorities by priority scheme

Returns a [paginated](#pagination) list of priorities available for adding to a priority scheme.  **[Permissions](#permissions) required:** Permission to access Jira.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scheme_id** | **String** | The priority scheme ID. | [required] |
**start_at** | Option<**String**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**String**> | The maximum number of items to return per page. |  |[default to 50]
**query** | Option<**String**> | The string to query priorities on by name. |  |[default to ]
**exclude** | Option<[**Vec<String>**](String.md)> | A list of priority IDs to exclude from the results. |  |

### Return type

[**models::PageBeanPriorityWithSequence**](PageBeanPriorityWithSequence.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_priorities_by_priority_scheme

> models::PageBeanPriorityWithSequence get_priorities_by_priority_scheme(scheme_id, start_at, max_results)
Get priorities by priority scheme

Returns a [paginated](#pagination) list of priorities by scheme.  **[Permissions](#permissions) required:** Permission to access Jira.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scheme_id** | **String** | The priority scheme ID. | [required] |
**start_at** | Option<**String**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**String**> | The maximum number of items to return per page. |  |[default to 50]

### Return type

[**models::PageBeanPriorityWithSequence**](PageBeanPriorityWithSequence.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_priority_schemes

> models::PageBeanPrioritySchemeWithPaginatedPrioritiesAndProjects get_priority_schemes(start_at, max_results, priority_id, scheme_id, scheme_name, only_default, order_by, expand)
Get priority schemes

Returns a [paginated](#pagination) list of priority schemes.  **[Permissions](#permissions) required:** Permission to access Jira.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_at** | Option<**String**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**String**> | The maximum number of items to return per page. |  |[default to 50]
**priority_id** | Option<[**Vec<i64>**](i64.md)> | A set of priority IDs to filter by. To include multiple IDs, provide an ampersand-separated list. For example, `priorityId=10000&priorityId=10001`. |  |
**scheme_id** | Option<[**Vec<i64>**](i64.md)> | A set of priority scheme IDs. To include multiple IDs, provide an ampersand-separated list. For example, `schemeId=10000&schemeId=10001`. |  |
**scheme_name** | Option<**String**> | The name of scheme to search for. |  |[default to ]
**only_default** | Option<**bool**> | Whether only the default priority is returned. |  |[default to false]
**order_by** | Option<**String**> | The ordering to return the priority schemes by. |  |[default to +name]
**expand** | Option<**String**> | A comma separated list of additional information to return. \"priorities\" will return priorities associated with the priority scheme. \"projects\" will return projects associated with the priority scheme. `expand=priorities,projects`. |  |

### Return type

[**models::PageBeanPrioritySchemeWithPaginatedPrioritiesAndProjects**](PageBeanPrioritySchemeWithPaginatedPrioritiesAndProjects.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_projects_by_priority_scheme

> models::PageBeanProject get_projects_by_priority_scheme(scheme_id, start_at, max_results, project_id, query)
Get projects by priority scheme

Returns a [paginated](#pagination) list of projects by scheme.  **[Permissions](#permissions) required:** Permission to access Jira.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scheme_id** | **String** | The priority scheme ID. | [required] |
**start_at** | Option<**String**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**String**> | The maximum number of items to return per page. |  |[default to 50]
**project_id** | Option<[**Vec<i64>**](i64.md)> | The project IDs to filter by. For example, `projectId=10000&projectId=10001`. |  |
**query** | Option<**String**> | The string to query projects on by name. |  |[default to ]

### Return type

[**models::PageBeanProject**](PageBeanProject.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## suggested_priorities_for_mappings

> models::PageBeanPriorityWithSequence suggested_priorities_for_mappings(suggested_mappings_request_bean)
Suggested priorities for mappings

Returns a [paginated](#pagination) list of priorities that would require mapping, given a change in priorities or projects associated with a priority scheme.  **[Permissions](#permissions) required:** Permission to access Jira.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**suggested_mappings_request_bean** | [**SuggestedMappingsRequestBean**](SuggestedMappingsRequestBean.md) |  | [required] |

### Return type

[**models::PageBeanPriorityWithSequence**](PageBeanPriorityWithSequence.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_priority_scheme

> models::UpdatePrioritySchemeResponseBean update_priority_scheme(scheme_id, update_priority_scheme_request_bean)
Update priority scheme

Updates a priority scheme. This includes its details, the lists of priorities and projects in it  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scheme_id** | **i64** | The ID of the priority scheme. | [required] |
**update_priority_scheme_request_bean** | [**UpdatePrioritySchemeRequestBean**](UpdatePrioritySchemeRequestBean.md) |  | [required] |

### Return type

[**models::UpdatePrioritySchemeResponseBean**](UpdatePrioritySchemeResponseBean.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

