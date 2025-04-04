# \ProjectRolesApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_project_role**](ProjectRolesApi.md#create_project_role) | **POST** /rest/api/3/role | Create project role
[**delete_project_role**](ProjectRolesApi.md#delete_project_role) | **DELETE** /rest/api/3/role/{id} | Delete project role
[**fully_update_project_role**](ProjectRolesApi.md#fully_update_project_role) | **PUT** /rest/api/3/role/{id} | Fully update project role
[**get_all_project_roles**](ProjectRolesApi.md#get_all_project_roles) | **GET** /rest/api/3/role | Get all project roles
[**get_project_role**](ProjectRolesApi.md#get_project_role) | **GET** /rest/api/3/project/{projectIdOrKey}/role/{id} | Get project role for project
[**get_project_role_by_id**](ProjectRolesApi.md#get_project_role_by_id) | **GET** /rest/api/3/role/{id} | Get project role by ID
[**get_project_role_details**](ProjectRolesApi.md#get_project_role_details) | **GET** /rest/api/3/project/{projectIdOrKey}/roledetails | Get project role details
[**get_project_roles**](ProjectRolesApi.md#get_project_roles) | **GET** /rest/api/3/project/{projectIdOrKey}/role | Get project roles for project
[**partial_update_project_role**](ProjectRolesApi.md#partial_update_project_role) | **POST** /rest/api/3/role/{id} | Partial update project role



## create_project_role

> models::ProjectRole create_project_role(create_update_role_request_bean)
Create project role

Creates a new project role with no [default actors](#api-rest-api-3-resolution-get). You can use the [Add default actors to project role](#api-rest-api-3-role-id-actors-post) operation to add default actors to the project role after creating it.  *Note that although a new project role is available to all projects upon creation, any default actors that are associated with the project role are not added to projects that existed prior to the role being created.*<  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_update_role_request_bean** | [**CreateUpdateRoleRequestBean**](CreateUpdateRoleRequestBean.md) |  | [required] |

### Return type

[**models::ProjectRole**](ProjectRole.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_project_role

> delete_project_role(id, swap)
Delete project role

Deletes a project role. You must specify a replacement project role if you wish to delete a project role that is in use.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the project role to delete. Use [Get all project roles](#api-rest-api-3-role-get) to get a list of project role IDs. | [required] |
**swap** | Option<**i64**> | The ID of the project role that will replace the one being deleted. The swap will attempt to swap the role in schemes (notifications, permissions, issue security), workflows, worklogs and comments. |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fully_update_project_role

> models::ProjectRole fully_update_project_role(id, create_update_role_request_bean)
Fully update project role

Updates the project role's name and description. You must include both a name and a description in the request.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the project role. Use [Get all project roles](#api-rest-api-3-role-get) to get a list of project role IDs. | [required] |
**create_update_role_request_bean** | [**CreateUpdateRoleRequestBean**](CreateUpdateRoleRequestBean.md) |  | [required] |

### Return type

[**models::ProjectRole**](ProjectRole.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_project_roles

> Vec<models::ProjectRole> get_all_project_roles()
Get all project roles

Gets a list of all project roles, complete with project role details and default actors.  ### About project roles ###  [Project roles](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-roles/) are a flexible way to to associate users and groups with projects. In Jira Cloud, the list of project roles is shared globally with all projects, but each project can have a different set of actors associated with it (unlike groups, which have the same membership throughout all Jira applications).  Project roles are used in [permission schemes](#api-rest-api-3-permissionscheme-get), [email notification schemes](#api-rest-api-3-notificationscheme-get), [issue security levels](#api-rest-api-3-issuesecurityschemes-get), [comment visibility](#api-rest-api-3-comment-list-post), and workflow conditions.  #### Members and actors ####  In the Jira REST API, a member of a project role is called an *actor*. An *actor* is a group or user associated with a project role.  Actors may be set as [default members](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-roles/#Specifying-'default-members'-for-a-project-role) of the project role or set at the project level:   *  Default actors: Users and groups that are assigned to the project role for all newly created projects. The default actors can be removed at the project level later if desired.  *  Actors: Users and groups that are associated with a project role for a project, which may differ from the default actors. This enables you to assign a user to different roles in different projects.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ProjectRole>**](ProjectRole.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_role

> models::ProjectRole get_project_role(project_id_or_key, id, exclude_inactive_users)
Get project role for project

Returns a project role's details and actors associated with the project. The list of actors is sorted by display name.  To check whether a user belongs to a role based on their group memberships, use [Get user](#api-rest-api-3-user-get) with the `groups` expand parameter selected. Then check whether the user keys and groups match with the actors returned for the project.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** *Administer Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project or *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id_or_key** | **String** | The project ID or project key (case sensitive). | [required] |
**id** | **i64** | The ID of the project role. Use [Get all project roles](#api-rest-api-3-role-get) to get a list of project role IDs. | [required] |
**exclude_inactive_users** | Option<**bool**> | Exclude inactive users. |  |[default to false]

### Return type

[**models::ProjectRole**](ProjectRole.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_role_by_id

> models::ProjectRole get_project_role_by_id(id)
Get project role by ID

Gets the project role details and the default actors associated with the role. The list of default actors is sorted by display name.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the project role. Use [Get all project roles](#api-rest-api-3-role-get) to get a list of project role IDs. | [required] |

### Return type

[**models::ProjectRole**](ProjectRole.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_role_details

> Vec<models::ProjectRoleDetails> get_project_role_details(project_id_or_key, current_member, exclude_connect_addons)
Get project role details

Returns all [project roles](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-roles/) and the details for each role. Note that the list of project roles is common to all projects.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) or *Administer projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id_or_key** | **String** | The project ID or project key (case sensitive). | [required] |
**current_member** | Option<**bool**> | Whether the roles should be filtered to include only those the user is assigned to. |  |[default to false]
**exclude_connect_addons** | Option<**bool**> |  |  |[default to false]

### Return type

[**Vec<models::ProjectRoleDetails>**](ProjectRoleDetails.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_roles

> std::collections::HashMap<String, String> get_project_roles(project_id_or_key)
Get project roles for project

Returns a list of [project roles](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-roles/) for the project returning the name and self URL for each role.  Note that all project roles are shared with all projects in Jira Cloud. See [Get all project roles](#api-rest-api-3-role-get) for more information.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** *Administer Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for any project on the site or *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id_or_key** | **String** | The project ID or project key (case sensitive). | [required] |

### Return type

**std::collections::HashMap<String, String>**

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## partial_update_project_role

> models::ProjectRole partial_update_project_role(id, create_update_role_request_bean)
Partial update project role

Updates either the project role's name or its description.  You cannot update both the name and description at the same time using this operation. If you send a request with a name and a description only the name is updated.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the project role. Use [Get all project roles](#api-rest-api-3-role-get) to get a list of project role IDs. | [required] |
**create_update_role_request_bean** | [**CreateUpdateRoleRequestBean**](CreateUpdateRoleRequestBean.md) |  | [required] |

### Return type

[**models::ProjectRole**](ProjectRole.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

