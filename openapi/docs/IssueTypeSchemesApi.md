# \IssueTypeSchemesApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_issue_types_to_issue_type_scheme**](IssueTypeSchemesApi.md#add_issue_types_to_issue_type_scheme) | **PUT** /rest/api/3/issuetypescheme/{issueTypeSchemeId}/issuetype | Add issue types to issue type scheme
[**assign_issue_type_scheme_to_project**](IssueTypeSchemesApi.md#assign_issue_type_scheme_to_project) | **PUT** /rest/api/3/issuetypescheme/project | Assign issue type scheme to project
[**create_issue_type_scheme**](IssueTypeSchemesApi.md#create_issue_type_scheme) | **POST** /rest/api/3/issuetypescheme | Create issue type scheme
[**delete_issue_type_scheme**](IssueTypeSchemesApi.md#delete_issue_type_scheme) | **DELETE** /rest/api/3/issuetypescheme/{issueTypeSchemeId} | Delete issue type scheme
[**get_all_issue_type_schemes**](IssueTypeSchemesApi.md#get_all_issue_type_schemes) | **GET** /rest/api/3/issuetypescheme | Get all issue type schemes
[**get_issue_type_scheme_for_projects**](IssueTypeSchemesApi.md#get_issue_type_scheme_for_projects) | **GET** /rest/api/3/issuetypescheme/project | Get issue type schemes for projects
[**get_issue_type_schemes_mapping**](IssueTypeSchemesApi.md#get_issue_type_schemes_mapping) | **GET** /rest/api/3/issuetypescheme/mapping | Get issue type scheme items
[**remove_issue_type_from_issue_type_scheme**](IssueTypeSchemesApi.md#remove_issue_type_from_issue_type_scheme) | **DELETE** /rest/api/3/issuetypescheme/{issueTypeSchemeId}/issuetype/{issueTypeId} | Remove issue type from issue type scheme
[**reorder_issue_types_in_issue_type_scheme**](IssueTypeSchemesApi.md#reorder_issue_types_in_issue_type_scheme) | **PUT** /rest/api/3/issuetypescheme/{issueTypeSchemeId}/issuetype/move | Change order of issue types
[**update_issue_type_scheme**](IssueTypeSchemesApi.md#update_issue_type_scheme) | **PUT** /rest/api/3/issuetypescheme/{issueTypeSchemeId} | Update issue type scheme



## add_issue_types_to_issue_type_scheme

> serde_json::Value add_issue_types_to_issue_type_scheme(issue_type_scheme_id, issue_type_ids)
Add issue types to issue type scheme

Adds issue types to an issue type scheme.  The added issue types are appended to the issue types list.  If any of the issue types exist in the issue type scheme, the operation fails and no issue types are added.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_type_scheme_id** | **i64** | The ID of the issue type scheme. | [required] |
**issue_type_ids** | [**IssueTypeIds**](IssueTypeIds.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assign_issue_type_scheme_to_project

> serde_json::Value assign_issue_type_scheme_to_project(issue_type_scheme_project_association)
Assign issue type scheme to project

Assigns an issue type scheme to a project.  If any issues in the project are assigned issue types not present in the new scheme, the operation will fail. To complete the assignment those issues must be updated to use issue types in the new scheme.  Issue type schemes can only be assigned to classic projects.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_type_scheme_project_association** | [**IssueTypeSchemeProjectAssociation**](IssueTypeSchemeProjectAssociation.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_issue_type_scheme

> models::IssueTypeSchemeId create_issue_type_scheme(issue_type_scheme_details)
Create issue type scheme

Creates an issue type scheme.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_type_scheme_details** | [**IssueTypeSchemeDetails**](IssueTypeSchemeDetails.md) |  | [required] |

### Return type

[**models::IssueTypeSchemeId**](IssueTypeSchemeID.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_issue_type_scheme

> serde_json::Value delete_issue_type_scheme(issue_type_scheme_id)
Delete issue type scheme

Deletes an issue type scheme.  Only issue type schemes used in classic projects can be deleted.  Any projects assigned to the scheme are reassigned to the default issue type scheme.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_type_scheme_id** | **i64** | The ID of the issue type scheme. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_issue_type_schemes

> models::PageBeanIssueTypeScheme get_all_issue_type_schemes(start_at, max_results, id, order_by, expand, query_string)
Get all issue type schemes

Returns a [paginated](#pagination) list of issue type schemes.  Only issue type schemes used in classic projects are returned.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 50]
**id** | Option<[**Vec<i64>**](i64.md)> | The list of issue type schemes IDs. To include multiple IDs, provide an ampersand-separated list. For example, `id=10000&id=10001`. |  |
**order_by** | Option<**String**> | [Order](#ordering) the results by a field:   *  `name` Sorts by issue type scheme name.  *  `id` Sorts by issue type scheme ID. |  |[default to id]
**expand** | Option<**String**> | Use [expand](#expansion) to include additional information in the response. This parameter accepts a comma-separated list. Expand options include:   *  `projects` For each issue type schemes, returns information about the projects the issue type scheme is assigned to.  *  `issueTypes` For each issue type schemes, returns information about the issueTypes the issue type scheme have. |  |[default to ]
**query_string** | Option<**String**> | String used to perform a case-insensitive partial match with issue type scheme name. |  |[default to ]

### Return type

[**models::PageBeanIssueTypeScheme**](PageBeanIssueTypeScheme.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_issue_type_scheme_for_projects

> models::PageBeanIssueTypeSchemeProjects get_issue_type_scheme_for_projects(project_id, start_at, max_results)
Get issue type schemes for projects

Returns a [paginated](#pagination) list of issue type schemes and, for each issue type scheme, a list of the projects that use it.  Only issue type schemes used in classic projects are returned.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | [**Vec<i64>**](i64.md) | The list of project IDs. To include multiple project IDs, provide an ampersand-separated list. For example, `projectId=10000&projectId=10001`. | [required] |
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 50]

### Return type

[**models::PageBeanIssueTypeSchemeProjects**](PageBeanIssueTypeSchemeProjects.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_issue_type_schemes_mapping

> models::PageBeanIssueTypeSchemeMapping get_issue_type_schemes_mapping(start_at, max_results, issue_type_scheme_id)
Get issue type scheme items

Returns a [paginated](#pagination) list of issue type scheme items.  Only issue type scheme items used in classic projects are returned.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 50]
**issue_type_scheme_id** | Option<[**Vec<i64>**](i64.md)> | The list of issue type scheme IDs. To include multiple IDs, provide an ampersand-separated list. For example, `issueTypeSchemeId=10000&issueTypeSchemeId=10001`. |  |

### Return type

[**models::PageBeanIssueTypeSchemeMapping**](PageBeanIssueTypeSchemeMapping.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_issue_type_from_issue_type_scheme

> serde_json::Value remove_issue_type_from_issue_type_scheme(issue_type_scheme_id, issue_type_id)
Remove issue type from issue type scheme

Removes an issue type from an issue type scheme.  This operation cannot remove:   *  any issue type used by issues.  *  any issue types from the default issue type scheme.  *  the last standard issue type from an issue type scheme.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_type_scheme_id** | **i64** | The ID of the issue type scheme. | [required] |
**issue_type_id** | **i64** | The ID of the issue type. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reorder_issue_types_in_issue_type_scheme

> serde_json::Value reorder_issue_types_in_issue_type_scheme(issue_type_scheme_id, order_of_issue_types)
Change order of issue types

Changes the order of issue types in an issue type scheme.  The request body parameters must meet the following requirements:   *  all of the issue types must belong to the issue type scheme.  *  either `after` or `position` must be provided.  *  the issue type in `after` must not be in the issue type list.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_type_scheme_id** | **i64** | The ID of the issue type scheme. | [required] |
**order_of_issue_types** | [**OrderOfIssueTypes**](OrderOfIssueTypes.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_issue_type_scheme

> serde_json::Value update_issue_type_scheme(issue_type_scheme_id, issue_type_scheme_update_details)
Update issue type scheme

Updates an issue type scheme.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_type_scheme_id** | **i64** | The ID of the issue type scheme. | [required] |
**issue_type_scheme_update_details** | [**IssueTypeSchemeUpdateDetails**](IssueTypeSchemeUpdateDetails.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

