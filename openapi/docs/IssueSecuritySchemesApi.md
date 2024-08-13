# \IssueSecuritySchemesApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_security_level**](IssueSecuritySchemesApi.md#add_security_level) | **PUT** /rest/api/3/issuesecurityschemes/{schemeId}/level | Add issue security levels
[**add_security_level_members**](IssueSecuritySchemesApi.md#add_security_level_members) | **PUT** /rest/api/3/issuesecurityschemes/{schemeId}/level/{levelId}/member | Add issue security level members
[**associate_schemes_to_projects**](IssueSecuritySchemesApi.md#associate_schemes_to_projects) | **PUT** /rest/api/3/issuesecurityschemes/project | Associate security scheme to project
[**create_issue_security_scheme**](IssueSecuritySchemesApi.md#create_issue_security_scheme) | **POST** /rest/api/3/issuesecurityschemes | Create issue security scheme
[**delete_security_scheme**](IssueSecuritySchemesApi.md#delete_security_scheme) | **DELETE** /rest/api/3/issuesecurityschemes/{schemeId} | Delete issue security scheme
[**get_issue_security_scheme**](IssueSecuritySchemesApi.md#get_issue_security_scheme) | **GET** /rest/api/3/issuesecurityschemes/{id} | Get issue security scheme
[**get_issue_security_schemes**](IssueSecuritySchemesApi.md#get_issue_security_schemes) | **GET** /rest/api/3/issuesecurityschemes | Get issue security schemes
[**get_security_level_members**](IssueSecuritySchemesApi.md#get_security_level_members) | **GET** /rest/api/3/issuesecurityschemes/level/member | Get issue security level members
[**get_security_levels**](IssueSecuritySchemesApi.md#get_security_levels) | **GET** /rest/api/3/issuesecurityschemes/level | Get issue security levels
[**remove_level**](IssueSecuritySchemesApi.md#remove_level) | **DELETE** /rest/api/3/issuesecurityschemes/{schemeId}/level/{levelId} | Remove issue security level
[**remove_member_from_security_level**](IssueSecuritySchemesApi.md#remove_member_from_security_level) | **DELETE** /rest/api/3/issuesecurityschemes/{schemeId}/level/{levelId}/member/{memberId} | Remove member from issue security level
[**search_projects_using_security_schemes**](IssueSecuritySchemesApi.md#search_projects_using_security_schemes) | **GET** /rest/api/3/issuesecurityschemes/project | Get projects using issue security schemes
[**search_security_schemes**](IssueSecuritySchemesApi.md#search_security_schemes) | **GET** /rest/api/3/issuesecurityschemes/search | Search issue security schemes
[**set_default_levels**](IssueSecuritySchemesApi.md#set_default_levels) | **PUT** /rest/api/3/issuesecurityschemes/level/default | Set default issue security levels
[**update_issue_security_scheme**](IssueSecuritySchemesApi.md#update_issue_security_scheme) | **PUT** /rest/api/3/issuesecurityschemes/{id} | Update issue security scheme
[**update_security_level**](IssueSecuritySchemesApi.md#update_security_level) | **PUT** /rest/api/3/issuesecurityschemes/{schemeId}/level/{levelId} | Update issue security level



## add_security_level

> serde_json::Value add_security_level(scheme_id, add_security_scheme_levels_request_bean)
Add issue security levels

Adds levels and levels' members to the issue security scheme. You can add up to 100 levels per request.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scheme_id** | **String** | The ID of the issue security scheme. | [required] |
**add_security_scheme_levels_request_bean** | [**AddSecuritySchemeLevelsRequestBean**](AddSecuritySchemeLevelsRequestBean.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_security_level_members

> serde_json::Value add_security_level_members(scheme_id, level_id, security_scheme_members_request)
Add issue security level members

Adds members to the issue security level. You can add up to 100 members per request.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scheme_id** | **String** | The ID of the issue security scheme. | [required] |
**level_id** | **String** | The ID of the issue security level. | [required] |
**security_scheme_members_request** | [**SecuritySchemeMembersRequest**](SecuritySchemeMembersRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## associate_schemes_to_projects

> associate_schemes_to_projects(associate_security_scheme_with_project_details)
Associate security scheme to project

Associates an issue security scheme with a project and remaps security levels of issues to the new levels, if provided.  This operation is [asynchronous](#async). Follow the `location` link in the response to determine the status of the task and use [Get task](#api-rest-api-3-task-taskId-get) to obtain subsequent updates.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**associate_security_scheme_with_project_details** | [**AssociateSecuritySchemeWithProjectDetails**](AssociateSecuritySchemeWithProjectDetails.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_issue_security_scheme

> models::SecuritySchemeId create_issue_security_scheme(create_issue_security_scheme_details)
Create issue security scheme

Creates a security scheme with security scheme levels and levels' members. You can create up to 100 security scheme levels and security scheme levels' members per request.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_issue_security_scheme_details** | [**CreateIssueSecuritySchemeDetails**](CreateIssueSecuritySchemeDetails.md) |  | [required] |

### Return type

[**models::SecuritySchemeId**](SecuritySchemeId.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_security_scheme

> serde_json::Value delete_security_scheme(scheme_id)
Delete issue security scheme

Deletes an issue security scheme.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scheme_id** | **String** | The ID of the issue security scheme. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_issue_security_scheme

> models::SecurityScheme get_issue_security_scheme(id)
Get issue security scheme

Returns an issue security scheme along with its security levels.  **[Permissions](#permissions) required:**   *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).  *  *Administer Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for a project that uses the requested issue security scheme.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the issue security scheme. Use the [Get issue security schemes](#api-rest-api-3-issuesecurityschemes-get) operation to get a list of issue security scheme IDs. | [required] |

### Return type

[**models::SecurityScheme**](SecurityScheme.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_issue_security_schemes

> models::SecuritySchemes get_issue_security_schemes()
Get issue security schemes

Returns all [issue security schemes](https://confluence.atlassian.com/x/J4lKLg).  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SecuritySchemes**](SecuritySchemes.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_security_level_members

> models::PageBeanSecurityLevelMember get_security_level_members(start_at, max_results, id, scheme_id, level_id, expand)
Get issue security level members

Returns a [paginated](#pagination) list of issue security level members.  Only issue security level members in the context of classic projects are returned.  Filtering using parameters is inclusive: if you specify both security scheme IDs and level IDs, the result will include all issue security level members from the specified schemes and levels.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_at** | Option<**String**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**String**> | The maximum number of items to return per page. |  |[default to 50]
**id** | Option<[**Vec<String>**](String.md)> | The list of issue security level member IDs. To include multiple issue security level members separate IDs with an ampersand: `id=10000&id=10001`. |  |
**scheme_id** | Option<[**Vec<String>**](String.md)> | The list of issue security scheme IDs. To include multiple issue security schemes separate IDs with an ampersand: `schemeId=10000&schemeId=10001`. |  |
**level_id** | Option<[**Vec<String>**](String.md)> | The list of issue security level IDs. To include multiple issue security levels separate IDs with an ampersand: `levelId=10000&levelId=10001`. |  |
**expand** | Option<**String**> | Use expand to include additional information in the response. This parameter accepts a comma-separated list. Expand options include:   *  `all` Returns all expandable information  *  `field` Returns information about the custom field granted the permission  *  `group` Returns information about the group that is granted the permission  *  `projectRole` Returns information about the project role granted the permission  *  `user` Returns information about the user who is granted the permission |  |

### Return type

[**models::PageBeanSecurityLevelMember**](PageBeanSecurityLevelMember.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_security_levels

> models::PageBeanSecurityLevel get_security_levels(start_at, max_results, id, scheme_id, only_default)
Get issue security levels

Returns a [paginated](#pagination) list of issue security levels.  Only issue security levels in the context of classic projects are returned.  Filtering using IDs is inclusive: if you specify both security scheme IDs and level IDs, the result will include both specified issue security levels and all issue security levels from the specified schemes.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_at** | Option<**String**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**String**> | The maximum number of items to return per page. |  |[default to 50]
**id** | Option<[**Vec<String>**](String.md)> | The list of issue security scheme level IDs. To include multiple issue security levels, separate IDs with an ampersand: `id=10000&id=10001`. |  |
**scheme_id** | Option<[**Vec<String>**](String.md)> | The list of issue security scheme IDs. To include multiple issue security schemes, separate IDs with an ampersand: `schemeId=10000&schemeId=10001`. |  |
**only_default** | Option<**bool**> | When set to true, returns multiple default levels for each security scheme containing a default. If you provide scheme and level IDs not associated with the default, returns an empty page. The default value is false. |  |[default to false]

### Return type

[**models::PageBeanSecurityLevel**](PageBeanSecurityLevel.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_level

> remove_level(scheme_id, level_id, replace_with)
Remove issue security level

Deletes an issue security level.  This operation is [asynchronous](#async). Follow the `location` link in the response to determine the status of the task and use [Get task](#api-rest-api-3-task-taskId-get) to obtain subsequent updates.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scheme_id** | **String** | The ID of the issue security scheme. | [required] |
**level_id** | **String** | The ID of the issue security level to remove. | [required] |
**replace_with** | Option<**String**> | The ID of the issue security level that will replace the currently selected level. |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_member_from_security_level

> serde_json::Value remove_member_from_security_level(scheme_id, level_id, member_id)
Remove member from issue security level

Removes an issue security level member from an issue security scheme.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scheme_id** | **String** | The ID of the issue security scheme. | [required] |
**level_id** | **String** | The ID of the issue security level. | [required] |
**member_id** | **String** | The ID of the issue security level member to be removed. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_projects_using_security_schemes

> models::PageBeanIssueSecuritySchemeToProjectMapping search_projects_using_security_schemes(start_at, max_results, issue_security_scheme_id, project_id)
Get projects using issue security schemes

Returns a [paginated](#pagination) mapping of projects that are using security schemes. You can provide either one or multiple security scheme IDs or project IDs to filter by. If you don't provide any, this will return a list of all mappings. Only issue security schemes in the context of classic projects are supported. **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_at** | Option<**String**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**String**> | The maximum number of items to return per page. |  |[default to 50]
**issue_security_scheme_id** | Option<[**Vec<String>**](String.md)> | The list of security scheme IDs to be filtered out. |  |
**project_id** | Option<[**Vec<String>**](String.md)> | The list of project IDs to be filtered out. |  |

### Return type

[**models::PageBeanIssueSecuritySchemeToProjectMapping**](PageBeanIssueSecuritySchemeToProjectMapping.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_security_schemes

> models::PageBeanSecuritySchemeWithProjects search_security_schemes(start_at, max_results, id, project_id)
Search issue security schemes

Returns a [paginated](#pagination) list of issue security schemes.   If you specify the project ID parameter, the result will contain issue security schemes and related project IDs you filter by. Use \\{@link IssueSecuritySchemeResource\\#searchProjectsUsingSecuritySchemes(String, String, Set, Set)\\} to obtain all projects related to scheme.  Only issue security schemes in the context of classic projects are returned.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_at** | Option<**String**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**String**> | The maximum number of items to return per page. |  |[default to 50]
**id** | Option<[**Vec<String>**](String.md)> | The list of issue security scheme IDs. To include multiple issue security scheme IDs, separate IDs with an ampersand: `id=10000&id=10001`. |  |
**project_id** | Option<[**Vec<String>**](String.md)> | The list of project IDs. To include multiple project IDs, separate IDs with an ampersand: `projectId=10000&projectId=10001`. |  |

### Return type

[**models::PageBeanSecuritySchemeWithProjects**](PageBeanSecuritySchemeWithProjects.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_default_levels

> serde_json::Value set_default_levels(set_default_levels_request)
Set default issue security levels

Sets default issue security levels for schemes.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**set_default_levels_request** | [**SetDefaultLevelsRequest**](SetDefaultLevelsRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_issue_security_scheme

> serde_json::Value update_issue_security_scheme(id, update_issue_security_scheme_request_bean)
Update issue security scheme

Updates the issue security scheme.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the issue security scheme. | [required] |
**update_issue_security_scheme_request_bean** | [**UpdateIssueSecuritySchemeRequestBean**](UpdateIssueSecuritySchemeRequestBean.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_security_level

> serde_json::Value update_security_level(scheme_id, level_id, update_issue_security_level_details)
Update issue security level

Updates the issue security level.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scheme_id** | **String** | The ID of the issue security scheme level belongs to. | [required] |
**level_id** | **String** | The ID of the issue security level to update. | [required] |
**update_issue_security_level_details** | [**UpdateIssueSecurityLevelDetails**](UpdateIssueSecurityLevelDetails.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

