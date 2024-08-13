# \IssueTypeScreenSchemesApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**append_mappings_for_issue_type_screen_scheme**](IssueTypeScreenSchemesApi.md#append_mappings_for_issue_type_screen_scheme) | **PUT** /rest/api/3/issuetypescreenscheme/{issueTypeScreenSchemeId}/mapping | Append mappings to issue type screen scheme
[**assign_issue_type_screen_scheme_to_project**](IssueTypeScreenSchemesApi.md#assign_issue_type_screen_scheme_to_project) | **PUT** /rest/api/3/issuetypescreenscheme/project | Assign issue type screen scheme to project
[**create_issue_type_screen_scheme**](IssueTypeScreenSchemesApi.md#create_issue_type_screen_scheme) | **POST** /rest/api/3/issuetypescreenscheme | Create issue type screen scheme
[**delete_issue_type_screen_scheme**](IssueTypeScreenSchemesApi.md#delete_issue_type_screen_scheme) | **DELETE** /rest/api/3/issuetypescreenscheme/{issueTypeScreenSchemeId} | Delete issue type screen scheme
[**get_issue_type_screen_scheme_mappings**](IssueTypeScreenSchemesApi.md#get_issue_type_screen_scheme_mappings) | **GET** /rest/api/3/issuetypescreenscheme/mapping | Get issue type screen scheme items
[**get_issue_type_screen_scheme_project_associations**](IssueTypeScreenSchemesApi.md#get_issue_type_screen_scheme_project_associations) | **GET** /rest/api/3/issuetypescreenscheme/project | Get issue type screen schemes for projects
[**get_issue_type_screen_schemes**](IssueTypeScreenSchemesApi.md#get_issue_type_screen_schemes) | **GET** /rest/api/3/issuetypescreenscheme | Get issue type screen schemes
[**get_projects_for_issue_type_screen_scheme**](IssueTypeScreenSchemesApi.md#get_projects_for_issue_type_screen_scheme) | **GET** /rest/api/3/issuetypescreenscheme/{issueTypeScreenSchemeId}/project | Get issue type screen scheme projects
[**remove_mappings_from_issue_type_screen_scheme**](IssueTypeScreenSchemesApi.md#remove_mappings_from_issue_type_screen_scheme) | **POST** /rest/api/3/issuetypescreenscheme/{issueTypeScreenSchemeId}/mapping/remove | Remove mappings from issue type screen scheme
[**update_default_screen_scheme**](IssueTypeScreenSchemesApi.md#update_default_screen_scheme) | **PUT** /rest/api/3/issuetypescreenscheme/{issueTypeScreenSchemeId}/mapping/default | Update issue type screen scheme default screen scheme
[**update_issue_type_screen_scheme**](IssueTypeScreenSchemesApi.md#update_issue_type_screen_scheme) | **PUT** /rest/api/3/issuetypescreenscheme/{issueTypeScreenSchemeId} | Update issue type screen scheme



## append_mappings_for_issue_type_screen_scheme

> serde_json::Value append_mappings_for_issue_type_screen_scheme(issue_type_screen_scheme_id, issue_type_screen_scheme_mapping_details)
Append mappings to issue type screen scheme

Appends issue type to screen scheme mappings to an issue type screen scheme.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_type_screen_scheme_id** | **String** | The ID of the issue type screen scheme. | [required] |
**issue_type_screen_scheme_mapping_details** | [**IssueTypeScreenSchemeMappingDetails**](IssueTypeScreenSchemeMappingDetails.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assign_issue_type_screen_scheme_to_project

> serde_json::Value assign_issue_type_screen_scheme_to_project(issue_type_screen_scheme_project_association)
Assign issue type screen scheme to project

Assigns an issue type screen scheme to a project.  Issue type screen schemes can only be assigned to classic projects.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_type_screen_scheme_project_association** | [**IssueTypeScreenSchemeProjectAssociation**](IssueTypeScreenSchemeProjectAssociation.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_issue_type_screen_scheme

> models::IssueTypeScreenSchemeId create_issue_type_screen_scheme(issue_type_screen_scheme_details)
Create issue type screen scheme

Creates an issue type screen scheme.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_type_screen_scheme_details** | [**IssueTypeScreenSchemeDetails**](IssueTypeScreenSchemeDetails.md) | An issue type screen scheme bean. | [required] |

### Return type

[**models::IssueTypeScreenSchemeId**](IssueTypeScreenSchemeId.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_issue_type_screen_scheme

> serde_json::Value delete_issue_type_screen_scheme(issue_type_screen_scheme_id)
Delete issue type screen scheme

Deletes an issue type screen scheme.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_type_screen_scheme_id** | **String** | The ID of the issue type screen scheme. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_issue_type_screen_scheme_mappings

> models::PageBeanIssueTypeScreenSchemeItem get_issue_type_screen_scheme_mappings(start_at, max_results, issue_type_screen_scheme_id)
Get issue type screen scheme items

Returns a [paginated](#pagination) list of issue type screen scheme items.  Only issue type screen schemes used in classic projects are returned.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 50]
**issue_type_screen_scheme_id** | Option<[**Vec<i64>**](i64.md)> | The list of issue type screen scheme IDs. To include multiple issue type screen schemes, separate IDs with ampersand: `issueTypeScreenSchemeId=10000&issueTypeScreenSchemeId=10001`. |  |

### Return type

[**models::PageBeanIssueTypeScreenSchemeItem**](PageBeanIssueTypeScreenSchemeItem.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_issue_type_screen_scheme_project_associations

> models::PageBeanIssueTypeScreenSchemesProjects get_issue_type_screen_scheme_project_associations(project_id, start_at, max_results)
Get issue type screen schemes for projects

Returns a [paginated](#pagination) list of issue type screen schemes and, for each issue type screen scheme, a list of the projects that use it.  Only issue type screen schemes used in classic projects are returned.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | [**Vec<i64>**](i64.md) | The list of project IDs. To include multiple projects, separate IDs with ampersand: `projectId=10000&projectId=10001`. | [required] |
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 50]

### Return type

[**models::PageBeanIssueTypeScreenSchemesProjects**](PageBeanIssueTypeScreenSchemesProjects.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_issue_type_screen_schemes

> models::PageBeanIssueTypeScreenScheme get_issue_type_screen_schemes(start_at, max_results, id, query_string, order_by, expand)
Get issue type screen schemes

Returns a [paginated](#pagination) list of issue type screen schemes.  Only issue type screen schemes used in classic projects are returned.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 50]
**id** | Option<[**Vec<i64>**](i64.md)> | The list of issue type screen scheme IDs. To include multiple IDs, provide an ampersand-separated list. For example, `id=10000&id=10001`. |  |
**query_string** | Option<**String**> | String used to perform a case-insensitive partial match with issue type screen scheme name. |  |[default to ]
**order_by** | Option<**String**> | [Order](#ordering) the results by a field:   *  `name` Sorts by issue type screen scheme name.  *  `id` Sorts by issue type screen scheme ID. |  |[default to id]
**expand** | Option<**String**> | Use [expand](#expansion) to include additional information in the response. This parameter accepts `projects` that, for each issue type screen schemes, returns information about the projects the issue type screen scheme is assigned to. |  |[default to ]

### Return type

[**models::PageBeanIssueTypeScreenScheme**](PageBeanIssueTypeScreenScheme.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_projects_for_issue_type_screen_scheme

> models::PageBeanProjectDetails get_projects_for_issue_type_screen_scheme(issue_type_screen_scheme_id, start_at, max_results, query)
Get issue type screen scheme projects

Returns a [paginated](#pagination) list of projects associated with an issue type screen scheme.  Only company-managed projects associated with an issue type screen scheme are returned.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_type_screen_scheme_id** | **i64** | The ID of the issue type screen scheme. | [required] |
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 50]
**query** | Option<**String**> |  |  |[default to ]

### Return type

[**models::PageBeanProjectDetails**](PageBeanProjectDetails.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_mappings_from_issue_type_screen_scheme

> serde_json::Value remove_mappings_from_issue_type_screen_scheme(issue_type_screen_scheme_id, issue_type_ids)
Remove mappings from issue type screen scheme

Removes issue type to screen scheme mappings from an issue type screen scheme.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_type_screen_scheme_id** | **String** | The ID of the issue type screen scheme. | [required] |
**issue_type_ids** | [**IssueTypeIds**](IssueTypeIds.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_default_screen_scheme

> serde_json::Value update_default_screen_scheme(issue_type_screen_scheme_id, update_default_screen_scheme)
Update issue type screen scheme default screen scheme

Updates the default screen scheme of an issue type screen scheme. The default screen scheme is used for all unmapped issue types.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_type_screen_scheme_id** | **String** | The ID of the issue type screen scheme. | [required] |
**update_default_screen_scheme** | [**UpdateDefaultScreenScheme**](UpdateDefaultScreenScheme.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_issue_type_screen_scheme

> serde_json::Value update_issue_type_screen_scheme(issue_type_screen_scheme_id, issue_type_screen_scheme_update_details)
Update issue type screen scheme

Updates an issue type screen scheme.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_type_screen_scheme_id** | **String** | The ID of the issue type screen scheme. | [required] |
**issue_type_screen_scheme_update_details** | [**IssueTypeScreenSchemeUpdateDetails**](IssueTypeScreenSchemeUpdateDetails.md) | The issue type screen scheme update details. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

