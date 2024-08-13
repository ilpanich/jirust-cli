# \ProjectRoleActorsApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_actor_users**](ProjectRoleActorsApi.md#add_actor_users) | **POST** /rest/api/3/project/{projectIdOrKey}/role/{id} | Add actors to project role
[**add_project_role_actors_to_role**](ProjectRoleActorsApi.md#add_project_role_actors_to_role) | **POST** /rest/api/3/role/{id}/actors | Add default actors to project role
[**delete_actor**](ProjectRoleActorsApi.md#delete_actor) | **DELETE** /rest/api/3/project/{projectIdOrKey}/role/{id} | Delete actors from project role
[**delete_project_role_actors_from_role**](ProjectRoleActorsApi.md#delete_project_role_actors_from_role) | **DELETE** /rest/api/3/role/{id}/actors | Delete default actors from project role
[**get_project_role_actors_for_role**](ProjectRoleActorsApi.md#get_project_role_actors_for_role) | **GET** /rest/api/3/role/{id}/actors | Get default actors for project role
[**set_actors**](ProjectRoleActorsApi.md#set_actors) | **PUT** /rest/api/3/project/{projectIdOrKey}/role/{id} | Set actors for project role



## add_actor_users

> models::ProjectRole add_actor_users(project_id_or_key, id, actors_map)
Add actors to project role

Adds actors to a project role for the project.  To replace all actors for the project, use [Set actors for project role](#api-rest-api-3-project-projectIdOrKey-role-id-put).  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** *Administer Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project or *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id_or_key** | **String** | The project ID or project key (case sensitive). | [required] |
**id** | **i64** | The ID of the project role. Use [Get all project roles](#api-rest-api-3-role-get) to get a list of project role IDs. | [required] |
**actors_map** | [**ActorsMap**](ActorsMap.md) | The groups or users to associate with the project role for this project. Provide the user account ID, group name, or group ID. As a group's name can change, use of group ID is recommended. | [required] |

### Return type

[**models::ProjectRole**](ProjectRole.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_project_role_actors_to_role

> models::ProjectRole add_project_role_actors_to_role(id, actor_input_bean)
Add default actors to project role

Adds [default actors](#api-rest-api-3-resolution-get) to a role. You may add groups or users, but you cannot add groups and users in the same request.  Changing a project role's default actors does not affect project role members for projects already created.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the project role. Use [Get all project roles](#api-rest-api-3-role-get) to get a list of project role IDs. | [required] |
**actor_input_bean** | [**ActorInputBean**](ActorInputBean.md) |  | [required] |

### Return type

[**models::ProjectRole**](ProjectRole.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_actor

> delete_actor(project_id_or_key, id, user, group, group_id)
Delete actors from project role

Deletes actors from a project role for the project.  To remove default actors from the project role, use [Delete default actors from project role](#api-rest-api-3-role-id-actors-delete).  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** *Administer Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project or *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id_or_key** | **String** | The project ID or project key (case sensitive). | [required] |
**id** | **i64** | The ID of the project role. Use [Get all project roles](#api-rest-api-3-role-get) to get a list of project role IDs. | [required] |
**user** | Option<**String**> | The user account ID of the user to remove from the project role. |  |
**group** | Option<**String**> | The name of the group to remove from the project role. This parameter cannot be used with the `groupId` parameter. As a group's name can change, use of `groupId` is recommended. |  |
**group_id** | Option<**String**> | The ID of the group to remove from the project role. This parameter cannot be used with the `group` parameter. |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_project_role_actors_from_role

> models::ProjectRole delete_project_role_actors_from_role(id, user, group_id, group)
Delete default actors from project role

Deletes the [default actors](#api-rest-api-3-resolution-get) from a project role. You may delete a group or user, but you cannot delete a group and a user in the same request.  Changing a project role's default actors does not affect project role members for projects already created.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the project role. Use [Get all project roles](#api-rest-api-3-role-get) to get a list of project role IDs. | [required] |
**user** | Option<**String**> | The user account ID of the user to remove as a default actor. |  |
**group_id** | Option<**String**> | The group ID of the group to be removed as a default actor. This parameter cannot be used with the `group` parameter. |  |
**group** | Option<**String**> | The group name of the group to be removed as a default actor.This parameter cannot be used with the `groupId` parameter. As a group's name can change, use of `groupId` is recommended. |  |

### Return type

[**models::ProjectRole**](ProjectRole.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_role_actors_for_role

> models::ProjectRole get_project_role_actors_for_role(id)
Get default actors for project role

Returns the [default actors](#api-rest-api-3-resolution-get) for the project role.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

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


## set_actors

> models::ProjectRole set_actors(project_id_or_key, id, project_role_actors_update_bean)
Set actors for project role

Sets the actors for a project role for a project, replacing all existing actors.  To add actors to the project without overwriting the existing list, use [Add actors to project role](#api-rest-api-3-project-projectIdOrKey-role-id-post).  **[Permissions](#permissions) required:** *Administer Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project or *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id_or_key** | **String** | The project ID or project key (case sensitive). | [required] |
**id** | **i64** | The ID of the project role. Use [Get all project roles](#api-rest-api-3-role-get) to get a list of project role IDs. | [required] |
**project_role_actors_update_bean** | [**ProjectRoleActorsUpdateBean**](ProjectRoleActorsUpdateBean.md) | The groups or users to associate with the project role for this project. Provide the user account ID, group name, or group ID. As a group's name can change, use of group ID is recommended. | [required] |

### Return type

[**models::ProjectRole**](ProjectRole.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

