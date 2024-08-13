# \ScreenTabFieldsApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_screen_tab_field**](ScreenTabFieldsApi.md#add_screen_tab_field) | **POST** /rest/api/3/screens/{screenId}/tabs/{tabId}/fields | Add screen tab field
[**get_all_screen_tab_fields**](ScreenTabFieldsApi.md#get_all_screen_tab_fields) | **GET** /rest/api/3/screens/{screenId}/tabs/{tabId}/fields | Get all screen tab fields
[**move_screen_tab_field**](ScreenTabFieldsApi.md#move_screen_tab_field) | **POST** /rest/api/3/screens/{screenId}/tabs/{tabId}/fields/{id}/move | Move screen tab field
[**remove_screen_tab_field**](ScreenTabFieldsApi.md#remove_screen_tab_field) | **DELETE** /rest/api/3/screens/{screenId}/tabs/{tabId}/fields/{id} | Remove screen tab field



## add_screen_tab_field

> models::ScreenableField add_screen_tab_field(screen_id, tab_id, add_field_bean)
Add screen tab field

Adds a field to a screen tab.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**screen_id** | **i64** | The ID of the screen. | [required] |
**tab_id** | **i64** | The ID of the screen tab. | [required] |
**add_field_bean** | [**AddFieldBean**](AddFieldBean.md) |  | [required] |

### Return type

[**models::ScreenableField**](ScreenableField.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_screen_tab_fields

> Vec<models::ScreenableField> get_all_screen_tab_fields(screen_id, tab_id, project_key)
Get all screen tab fields

Returns all fields for a screen tab.  **[Permissions](#permissions) required:**   *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).  *  *Administer projects* [project permission](https://confluence.atlassian.com/x/yodKLg) when the project key is specified, providing that the screen is associated with the project through a Screen Scheme and Issue Type Screen Scheme.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**screen_id** | **i64** | The ID of the screen. | [required] |
**tab_id** | **i64** | The ID of the screen tab. | [required] |
**project_key** | Option<**String**> | The key of the project. |  |

### Return type

[**Vec<models::ScreenableField>**](ScreenableField.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## move_screen_tab_field

> serde_json::Value move_screen_tab_field(screen_id, tab_id, id, move_field_bean)
Move screen tab field

Moves a screen tab field.  If `after` and `position` are provided in the request, `position` is ignored.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**screen_id** | **i64** | The ID of the screen. | [required] |
**tab_id** | **i64** | The ID of the screen tab. | [required] |
**id** | **String** | The ID of the field. | [required] |
**move_field_bean** | [**MoveFieldBean**](MoveFieldBean.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_screen_tab_field

> remove_screen_tab_field(screen_id, tab_id, id)
Remove screen tab field

Removes a field from a screen tab.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**screen_id** | **i64** | The ID of the screen. | [required] |
**tab_id** | **i64** | The ID of the screen tab. | [required] |
**id** | **String** | The ID of the field. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

