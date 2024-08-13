# \ProjectPermissionSchemesApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**assign_permission_scheme**](ProjectPermissionSchemesApi.md#assign_permission_scheme) | **PUT** /rest/api/3/project/{projectKeyOrId}/permissionscheme | Assign permission scheme
[**get_assigned_permission_scheme**](ProjectPermissionSchemesApi.md#get_assigned_permission_scheme) | **GET** /rest/api/3/project/{projectKeyOrId}/permissionscheme | Get assigned permission scheme
[**get_project_issue_security_scheme**](ProjectPermissionSchemesApi.md#get_project_issue_security_scheme) | **GET** /rest/api/3/project/{projectKeyOrId}/issuesecuritylevelscheme | Get project issue security scheme
[**get_security_levels_for_project**](ProjectPermissionSchemesApi.md#get_security_levels_for_project) | **GET** /rest/api/3/project/{projectKeyOrId}/securitylevel | Get project issue security levels



## assign_permission_scheme

> models::PermissionScheme assign_permission_scheme(project_key_or_id, id_bean, expand)
Assign permission scheme

Assigns a permission scheme with a project. See [Managing project permissions](https://confluence.atlassian.com/x/yodKLg) for more information about permission schemes.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_key_or_id** | **String** | The project ID or project key (case sensitive). | [required] |
**id_bean** | [**IdBean**](IdBean.md) |  | [required] |
**expand** | Option<**String**> | Use [expand](#expansion) to include additional information in the response. This parameter accepts a comma-separated list. Note that permissions are included when you specify any value. Expand options include:   *  `all` Returns all expandable information.  *  `field` Returns information about the custom field granted the permission.  *  `group` Returns information about the group that is granted the permission.  *  `permissions` Returns all permission grants for each permission scheme.  *  `projectRole` Returns information about the project role granted the permission.  *  `user` Returns information about the user who is granted the permission. |  |

### Return type

[**models::PermissionScheme**](PermissionScheme.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_assigned_permission_scheme

> models::PermissionScheme get_assigned_permission_scheme(project_key_or_id, expand)
Get assigned permission scheme

Gets the [permission scheme](https://confluence.atlassian.com/x/yodKLg) associated with the project.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) or *Administer projects* [project permission](https://confluence.atlassian.com/x/yodKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_key_or_id** | **String** | The project ID or project key (case sensitive). | [required] |
**expand** | Option<**String**> | Use [expand](#expansion) to include additional information in the response. This parameter accepts a comma-separated list. Note that permissions are included when you specify any value. Expand options include:   *  `all` Returns all expandable information.  *  `field` Returns information about the custom field granted the permission.  *  `group` Returns information about the group that is granted the permission.  *  `permissions` Returns all permission grants for each permission scheme.  *  `projectRole` Returns information about the project role granted the permission.  *  `user` Returns information about the user who is granted the permission. |  |

### Return type

[**models::PermissionScheme**](PermissionScheme.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_issue_security_scheme

> models::SecurityScheme get_project_issue_security_scheme(project_key_or_id)
Get project issue security scheme

Returns the [issue security scheme](https://confluence.atlassian.com/x/J4lKLg) associated with the project.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) or the *Administer Projects* [project permission](https://confluence.atlassian.com/x/yodKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_key_or_id** | **String** | The project ID or project key (case sensitive). | [required] |

### Return type

[**models::SecurityScheme**](SecurityScheme.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_security_levels_for_project

> models::ProjectIssueSecurityLevels get_security_levels_for_project(project_key_or_id)
Get project issue security levels

Returns all [issue security](https://confluence.atlassian.com/x/J4lKLg) levels for the project that the user has access to.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** *Browse projects* [global permission](https://confluence.atlassian.com/x/x4dKLg) for the project, however, issue security levels are only returned for authenticated user with *Set Issue Security* [global permission](https://confluence.atlassian.com/x/x4dKLg) for the project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_key_or_id** | **String** | The project ID or project key (case sensitive). | [required] |

### Return type

[**models::ProjectIssueSecurityLevels**](ProjectIssueSecurityLevels.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

