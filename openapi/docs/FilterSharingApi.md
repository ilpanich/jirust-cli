# \FilterSharingApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_share_permission**](FilterSharingApi.md#add_share_permission) | **POST** /rest/api/3/filter/{id}/permission | Add share permission
[**delete_share_permission**](FilterSharingApi.md#delete_share_permission) | **DELETE** /rest/api/3/filter/{id}/permission/{permissionId} | Delete share permission
[**get_default_share_scope**](FilterSharingApi.md#get_default_share_scope) | **GET** /rest/api/3/filter/defaultShareScope | Get default share scope
[**get_share_permission**](FilterSharingApi.md#get_share_permission) | **GET** /rest/api/3/filter/{id}/permission/{permissionId} | Get share permission
[**get_share_permissions**](FilterSharingApi.md#get_share_permissions) | **GET** /rest/api/3/filter/{id}/permission | Get share permissions
[**set_default_share_scope**](FilterSharingApi.md#set_default_share_scope) | **PUT** /rest/api/3/filter/defaultShareScope | Set default share scope



## add_share_permission

> Vec<models::SharePermission> add_share_permission(id, share_permission_input_bean)
Add share permission

Add a share permissions to a filter. If you add a global share permission (one for all logged-in users or the public) it will overwrite all share permissions for the filter.  Be aware that this operation uses different objects for updating share permissions compared to [Update filter](#api-rest-api-3-filter-id-put).  **[Permissions](#permissions) required:** *Share dashboards and filters* [global permission](https://confluence.atlassian.com/x/x4dKLg) and the user must own the filter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the filter. | [required] |
**share_permission_input_bean** | [**SharePermissionInputBean**](SharePermissionInputBean.md) |  | [required] |

### Return type

[**Vec<models::SharePermission>**](SharePermission.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_share_permission

> delete_share_permission(id, permission_id)
Delete share permission

Deletes a share permission from a filter.  **[Permissions](#permissions) required:** Permission to access Jira and the user must own the filter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the filter. | [required] |
**permission_id** | **i64** | The ID of the share permission. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_default_share_scope

> models::DefaultShareScope get_default_share_scope()
Get default share scope

Returns the default sharing settings for new filters and dashboards for a user.  **[Permissions](#permissions) required:** Permission to access Jira.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::DefaultShareScope**](DefaultShareScope.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_share_permission

> models::SharePermission get_share_permission(id, permission_id)
Get share permission

Returns a share permission for a filter. A filter can be shared with groups, projects, all logged-in users, or the public. Sharing with all logged-in users or the public is known as a global share permission.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** None, however, a share permission is only returned for:   *  filters owned by the user.  *  filters shared with a group that the user is a member of.  *  filters shared with a private project that the user has *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for.  *  filters shared with a public project.  *  filters shared with the public.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the filter. | [required] |
**permission_id** | **i64** | The ID of the share permission. | [required] |

### Return type

[**models::SharePermission**](SharePermission.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_share_permissions

> Vec<models::SharePermission> get_share_permissions(id)
Get share permissions

Returns the share permissions for a filter. A filter can be shared with groups, projects, all logged-in users, or the public. Sharing with all logged-in users or the public is known as a global share permission.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** None, however, share permissions are only returned for:   *  filters owned by the user.  *  filters shared with a group that the user is a member of.  *  filters shared with a private project that the user has *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for.  *  filters shared with a public project.  *  filters shared with the public.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the filter. | [required] |

### Return type

[**Vec<models::SharePermission>**](SharePermission.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_default_share_scope

> models::DefaultShareScope set_default_share_scope(default_share_scope)
Set default share scope

Sets the default sharing for new filters and dashboards for a user.  **[Permissions](#permissions) required:** Permission to access Jira.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**default_share_scope** | [**DefaultShareScope**](DefaultShareScope.md) |  | [required] |

### Return type

[**models::DefaultShareScope**](DefaultShareScope.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

