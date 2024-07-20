# \ProjectAvatarsApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_project_avatar**](ProjectAvatarsApi.md#create_project_avatar) | **POST** /rest/api/3/project/{projectIdOrKey}/avatar2 | Load project avatar
[**delete_project_avatar**](ProjectAvatarsApi.md#delete_project_avatar) | **DELETE** /rest/api/3/project/{projectIdOrKey}/avatar/{id} | Delete project avatar
[**get_all_project_avatars**](ProjectAvatarsApi.md#get_all_project_avatars) | **GET** /rest/api/3/project/{projectIdOrKey}/avatars | Get all project avatars
[**update_project_avatar**](ProjectAvatarsApi.md#update_project_avatar) | **PUT** /rest/api/3/project/{projectIdOrKey}/avatar | Set project avatar



## create_project_avatar

> models::Avatar create_project_avatar(project_id_or_key, body, x, y, size)
Load project avatar

Loads an avatar for a project.  Specify the avatar's local file location in the body of the request. Also, include the following headers:   *  `X-Atlassian-Token: no-check` To prevent XSRF protection blocking the request, for more information see [Special Headers](#special-request-headers).  *  `Content-Type: image/image type` Valid image types are JPEG, GIF, or PNG.  For example:   `curl --request POST `  `--user email@example.com:<api_token> `  `--header 'X-Atlassian-Token: no-check' `  `--header 'Content-Type: image/< image_type>' `  `--data-binary \"<@/path/to/file/with/your/avatar>\" `  `--url 'https://your-domain.atlassian.net/rest/api/3/project/{projectIdOrKey}/avatar2'`  The avatar is cropped to a square. If no crop parameters are specified, the square originates at the top left of the image. The length of the square's sides is set to the smaller of the height or width of the image.  The cropped image is then used to create avatars of 16x16, 24x24, 32x32, and 48x48 in size.  After creating the avatar use [Set project avatar](#api-rest-api-3-project-projectIdOrKey-avatar-put) to set it as the project's displayed avatar.  **[Permissions](#permissions) required:** *Administer projects* [project permission](https://confluence.atlassian.com/x/yodKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id_or_key** | **String** | The ID or (case-sensitive) key of the project. | [required] |
**body** | Option<**serde_json::Value**> |  | [required] |
**x** | Option<**i32**> | The X coordinate of the top-left corner of the crop region. |  |[default to 0]
**y** | Option<**i32**> | The Y coordinate of the top-left corner of the crop region. |  |[default to 0]
**size** | Option<**i32**> | The length of each side of the crop region. |  |[default to 0]

### Return type

[**models::Avatar**](Avatar.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_project_avatar

> delete_project_avatar(project_id_or_key, id)
Delete project avatar

Deletes a custom avatar from a project. Note that system avatars cannot be deleted.  **[Permissions](#permissions) required:** *Administer projects* [project permission](https://confluence.atlassian.com/x/yodKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id_or_key** | **String** | The project ID or (case-sensitive) key. | [required] |
**id** | **i64** | The ID of the avatar. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_project_avatars

> models::ProjectAvatars get_all_project_avatars(project_id_or_key)
Get all project avatars

Returns all project avatars, grouped by system and custom avatars.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id_or_key** | **String** | The ID or (case-sensitive) key of the project. | [required] |

### Return type

[**models::ProjectAvatars**](ProjectAvatars.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_project_avatar

> serde_json::Value update_project_avatar(project_id_or_key, avatar)
Set project avatar

Sets the avatar displayed for a project.  Use [Load project avatar](#api-rest-api-3-project-projectIdOrKey-avatar2-post) to store avatars against the project, before using this operation to set the displayed avatar.  **[Permissions](#permissions) required:** *Administer projects* [project permission](https://confluence.atlassian.com/x/yodKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id_or_key** | **String** | The ID or (case-sensitive) key of the project. | [required] |
**avatar** | [**Avatar**](Avatar.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

