# \IssueCommentPropertiesApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_comment_property**](IssueCommentPropertiesApi.md#delete_comment_property) | **DELETE** /rest/api/3/comment/{commentId}/properties/{propertyKey} | Delete comment property
[**get_comment_property**](IssueCommentPropertiesApi.md#get_comment_property) | **GET** /rest/api/3/comment/{commentId}/properties/{propertyKey} | Get comment property
[**get_comment_property_keys**](IssueCommentPropertiesApi.md#get_comment_property_keys) | **GET** /rest/api/3/comment/{commentId}/properties | Get comment property keys
[**set_comment_property**](IssueCommentPropertiesApi.md#set_comment_property) | **PUT** /rest/api/3/comment/{commentId}/properties/{propertyKey} | Set comment property



## delete_comment_property

> delete_comment_property(comment_id, property_key)
Delete comment property

Deletes a comment property.  **[Permissions](#permissions) required:** either of:   *  *Edit All Comments* [project permission](https://confluence.atlassian.com/x/yodKLg) to delete a property from any comment.  *  *Edit Own Comments* [project permission](https://confluence.atlassian.com/x/yodKLg) to delete a property from a comment created by the user.  Also, when the visibility of a comment is restricted to a role or group the user must be a member of that role or group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **String** | The ID of the comment. | [required] |
**property_key** | **String** | The key of the property. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_comment_property

> models::EntityProperty get_comment_property(comment_id, property_key)
Get comment property

Returns the value of a comment property.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:**   *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.  *  If the comment has visibility restrictions, belongs to the group or has the role visibility is restricted to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **String** | The ID of the comment. | [required] |
**property_key** | **String** | The key of the property. | [required] |

### Return type

[**models::EntityProperty**](EntityProperty.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_comment_property_keys

> models::PropertyKeys get_comment_property_keys(comment_id)
Get comment property keys

Returns the keys of all the properties of a comment.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:**   *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.  *  If the comment has visibility restrictions, belongs to the group or has the role visibility is restricted to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **String** | The ID of the comment. | [required] |

### Return type

[**models::PropertyKeys**](PropertyKeys.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_comment_property

> serde_json::Value set_comment_property(comment_id, property_key, body)
Set comment property

Creates or updates the value of a property for a comment. Use this resource to store custom data against a comment.  The value of the request body must be a [valid](http://tools.ietf.org/html/rfc4627), non-empty JSON blob. The maximum length is 32768 characters.  **[Permissions](#permissions) required:** either of:   *  *Edit All Comments* [project permission](https://confluence.atlassian.com/x/yodKLg) to create or update the value of a property on any comment.  *  *Edit Own Comments* [project permission](https://confluence.atlassian.com/x/yodKLg) to create or update the value of a property on a comment created by the user.  Also, when the visibility of a comment is restricted to a role or group the user must be a member of that role or group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **String** | The ID of the comment. | [required] |
**property_key** | **String** | The key of the property. The maximum length is 255 characters. | [required] |
**body** | Option<**serde_json::Value**> | The value of the property. The value has to be a valid, non-empty [JSON](https://tools.ietf.org/html/rfc4627) value. The maximum length of the property value is 32768 bytes. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

