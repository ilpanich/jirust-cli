# \AvatarsApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_avatar**](AvatarsApi.md#delete_avatar) | **DELETE** /rest/api/3/universal_avatar/type/{type}/owner/{owningObjectId}/avatar/{id} | Delete avatar
[**get_all_system_avatars**](AvatarsApi.md#get_all_system_avatars) | **GET** /rest/api/3/avatar/{type}/system | Get system avatars by type
[**get_avatar_image_by_id**](AvatarsApi.md#get_avatar_image_by_id) | **GET** /rest/api/3/universal_avatar/view/type/{type}/avatar/{id} | Get avatar image by ID
[**get_avatar_image_by_owner**](AvatarsApi.md#get_avatar_image_by_owner) | **GET** /rest/api/3/universal_avatar/view/type/{type}/owner/{entityId} | Get avatar image by owner
[**get_avatar_image_by_type**](AvatarsApi.md#get_avatar_image_by_type) | **GET** /rest/api/3/universal_avatar/view/type/{type} | Get avatar image by type
[**get_avatars**](AvatarsApi.md#get_avatars) | **GET** /rest/api/3/universal_avatar/type/{type}/owner/{entityId} | Get avatars
[**store_avatar**](AvatarsApi.md#store_avatar) | **POST** /rest/api/3/universal_avatar/type/{type}/owner/{entityId} | Load avatar



## delete_avatar

> delete_avatar(r#type, owning_object_id, id)
Delete avatar

Deletes an avatar from a project, issue type or priority.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | The avatar type. | [required] |
**owning_object_id** | **String** | The ID of the item the avatar is associated with. | [required] |
**id** | **i64** | The ID of the avatar. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_system_avatars

> models::SystemAvatars get_all_system_avatars(r#type)
Get system avatars by type

Returns a list of system avatar details by owner type, where the owner types are issue type, project, user or priority.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** None.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | The avatar type. | [required] |

### Return type

[**models::SystemAvatars**](SystemAvatars.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_avatar_image_by_id

> get_avatar_image_by_id(r#type, id, size, format)
Get avatar image by ID

Returns a project, issue type or priority avatar image by ID.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:**   *  For system avatars, none.  *  For custom project avatars, *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project the avatar belongs to.  *  For custom issue type avatars, *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for at least one project the issue type is used in.  *  For priority avatars, none.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | The icon type of the avatar. | [required] |
**id** | **i64** | The ID of the avatar. | [required] |
**size** | Option<**String**> | The size of the avatar image. If not provided the default size is returned. |  |
**format** | Option<**String**> | The format to return the avatar image in. If not provided the original content format is returned. |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json, image/png, image/svg+xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_avatar_image_by_owner

> get_avatar_image_by_owner(r#type, entity_id, size, format)
Get avatar image by owner

Returns the avatar image for a project, issue type or priority.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:**   *  For system avatars, none.  *  For custom project avatars, *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project the avatar belongs to.  *  For custom issue type avatars, *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for at least one project the issue type is used in.  *  For priority avatars, none.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | The icon type of the avatar. | [required] |
**entity_id** | **String** | The ID of the project or issue type the avatar belongs to. | [required] |
**size** | Option<**String**> | The size of the avatar image. If not provided the default size is returned. |  |
**format** | Option<**String**> | The format to return the avatar image in. If not provided the original content format is returned. |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json, image/png, image/svg+xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_avatar_image_by_type

> get_avatar_image_by_type(r#type, size, format)
Get avatar image by type

Returns the default project, issue type or priority avatar image.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** None.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | The icon type of the avatar. | [required] |
**size** | Option<**String**> | The size of the avatar image. If not provided the default size is returned. |  |
**format** | Option<**String**> | The format to return the avatar image in. If not provided the original content format is returned. |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json, image/png, image/svg+xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_avatars

> models::Avatars get_avatars(r#type, entity_id)
Get avatars

Returns the system and custom avatars for a project, issue type or priority.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:**   *  for custom project avatars, *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project the avatar belongs to.  *  for custom issue type avatars, *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for at least one project the issue type is used in.  *  for system avatars, none.  *  for priority avatars, none.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | The avatar type. | [required] |
**entity_id** | **String** | The ID of the item the avatar is associated with. | [required] |

### Return type

[**models::Avatars**](Avatars.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_avatar

> models::Avatar store_avatar(r#type, entity_id, size, body, x, y)
Load avatar

Loads a custom avatar for a project, issue type or priority.  Specify the avatar's local file location in the body of the request. Also, include the following headers:   *  `X-Atlassian-Token: no-check` To prevent XSRF protection blocking the request, for more information see [Special Headers](#special-request-headers).  *  `Content-Type: image/image type` Valid image types are JPEG, GIF, or PNG.  For example:   `curl --request POST `  `--user email@example.com:<api_token> `  `--header 'X-Atlassian-Token: no-check' `  `--header 'Content-Type: image/< image_type>' `  `--data-binary \"<@/path/to/file/with/your/avatar>\" `  `--url 'https://your-domain.atlassian.net/rest/api/3/universal_avatar/type/{type}/owner/{entityId}'`  The avatar is cropped to a square. If no crop parameters are specified, the square originates at the top left of the image. The length of the square's sides is set to the smaller of the height or width of the image.  The cropped image is then used to create avatars of 16x16, 24x24, 32x32, and 48x48 in size.  After creating the avatar use:   *  [Update issue type](#api-rest-api-3-issuetype-id-put) to set it as the issue type's displayed avatar.  *  [Set project avatar](#api-rest-api-3-project-projectIdOrKey-avatar-put) to set it as the project's displayed avatar.  *  [Update priority](#api-rest-api-3-priority-id-put) to set it as the priority's displayed avatar.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | The avatar type. | [required] |
**entity_id** | **String** | The ID of the item the avatar is associated with. | [required] |
**size** | **i32** | The length of each side of the crop region. | [required] |[default to 0]
**body** | Option<**serde_json::Value**> |  | [required] |
**x** | Option<**i32**> | The X coordinate of the top-left corner of the crop region. |  |[default to 0]
**y** | Option<**i32**> | The Y coordinate of the top-left corner of the crop region. |  |[default to 0]

### Return type

[**models::Avatar**](Avatar.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

