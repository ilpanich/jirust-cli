# \ProjectComponentsApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_component**](ProjectComponentsApi.md#create_component) | **POST** /rest/api/3/component | Create component
[**delete_component**](ProjectComponentsApi.md#delete_component) | **DELETE** /rest/api/3/component/{id} | Delete component
[**find_components_for_projects**](ProjectComponentsApi.md#find_components_for_projects) | **GET** /rest/api/3/component | Find components for projects
[**get_component**](ProjectComponentsApi.md#get_component) | **GET** /rest/api/3/component/{id} | Get component
[**get_component_related_issues**](ProjectComponentsApi.md#get_component_related_issues) | **GET** /rest/api/3/component/{id}/relatedIssueCounts | Get component issues count
[**get_project_components**](ProjectComponentsApi.md#get_project_components) | **GET** /rest/api/3/project/{projectIdOrKey}/components | Get project components
[**get_project_components_paginated**](ProjectComponentsApi.md#get_project_components_paginated) | **GET** /rest/api/3/project/{projectIdOrKey}/component | Get project components paginated
[**update_component**](ProjectComponentsApi.md#update_component) | **PUT** /rest/api/3/component/{id} | Update component



## create_component

> models::ProjectComponent create_component(project_component)
Create component

Creates a component. Use components to provide containers for issues within a project. Use components to provide containers for issues within a project.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** *Administer projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project in which the component is created or *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_component** | [**ProjectComponent**](ProjectComponent.md) |  | [required] |

### Return type

[**models::ProjectComponent**](ProjectComponent.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_component

> delete_component(id, move_issues_to)
Delete component

Deletes a component.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** *Administer projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the component or *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the component. | [required] |
**move_issues_to** | Option<**String**> | The ID of the component to replace the deleted component. If this value is null no replacement is made. |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_components_for_projects

> models::PageBean2ComponentJsonBean find_components_for_projects(project_ids_or_keys, start_at, max_results, order_by, query)
Find components for projects

Returns a [paginated](#pagination) list of all components in a project, including global (Compass) components when applicable.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** *Browse Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_ids_or_keys** | Option<[**Vec<String>**](String.md)> | The project IDs and/or project keys (case sensitive). |  |
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 50]
**order_by** | Option<**String**> | [Order](#ordering) the results by a field:   *  `description` Sorts by the component description.  *  `name` Sorts by component name. |  |
**query** | Option<**String**> | Filter the results using a literal string. Components with a matching `name` or `description` are returned (case insensitive). |  |

### Return type

[**models::PageBean2ComponentJsonBean**](PageBean2ComponentJsonBean.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_component

> models::ProjectComponent get_component(id)
Get component

Returns a component.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for project containing the component.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the component. | [required] |

### Return type

[**models::ProjectComponent**](ProjectComponent.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_component_related_issues

> models::ComponentIssuesCount get_component_related_issues(id)
Get component issues count

Returns the counts of issues assigned to the component.  This operation can be accessed anonymously.  **Deprecation notice:** The required OAuth 2.0 scopes will be updated on June 15, 2024.   *  **Classic**: `read:jira-work`  *  **Granular**: `read:field:jira`, `read:project.component:jira`  **[Permissions](#permissions) required:** None.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the component. | [required] |

### Return type

[**models::ComponentIssuesCount**](ComponentIssuesCount.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_components

> Vec<models::ProjectComponent> get_project_components(project_id_or_key, component_source)
Get project components

Returns all components in a project. See the [Get project components paginated](#api-rest-api-3-project-projectIdOrKey-component-get) resource if you want to get a full list of components with pagination.  If your project uses Compass components, this API will return a paginated list of Compass components that are linked to issues in that project.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** *Browse Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id_or_key** | **String** | The project ID or project key (case sensitive). | [required] |
**component_source** | Option<**String**> | The source of the components to return. Can be `jira` (default), `compass` or `auto`. When `auto` is specified, the API will return connected Compass components if the project is opted into Compass, otherwise it will return Jira components. Defaults to `jira`. |  |[default to jira]

### Return type

[**Vec<models::ProjectComponent>**](ProjectComponent.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_components_paginated

> models::PageBeanComponentWithIssueCount get_project_components_paginated(project_id_or_key, start_at, max_results, order_by, component_source, query)
Get project components paginated

Returns a [paginated](#pagination) list of all components in a project. See the [Get project components](#api-rest-api-3-project-projectIdOrKey-components-get) resource if you want to get a full list of versions without pagination.  If your project uses Compass components, this API will return a list of Compass components that are linked to issues in that project.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** *Browse Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id_or_key** | **String** | The project ID or project key (case sensitive). | [required] |
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 50]
**order_by** | Option<**String**> | [Order](#ordering) the results by a field:   *  `description` Sorts by the component description.  *  `issueCount` Sorts by the count of issues associated with the component.  *  `lead` Sorts by the user key of the component's project lead.  *  `name` Sorts by component name. |  |
**component_source** | Option<**String**> | The source of the components to return. Can be `jira` (default), `compass` or `auto`. When `auto` is specified, the API will return connected Compass components if the project is opted into Compass, otherwise it will return Jira components. Defaults to `jira`. |  |[default to jira]
**query** | Option<**String**> | Filter the results using a literal string. Components with a matching `name` or `description` are returned (case insensitive). |  |

### Return type

[**models::PageBeanComponentWithIssueCount**](PageBeanComponentWithIssueCount.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_component

> models::ProjectComponent update_component(id, project_component)
Update component

Updates a component. Any fields included in the request are overwritten. If `leadAccountId` is an empty string (\"\") the component lead is removed.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** *Administer projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the component or *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the component. | [required] |
**project_component** | [**ProjectComponent**](ProjectComponent.md) |  | [required] |

### Return type

[**models::ProjectComponent**](ProjectComponent.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

