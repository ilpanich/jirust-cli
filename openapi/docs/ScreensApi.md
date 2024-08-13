# \ScreensApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_field_to_default_screen**](ScreensApi.md#add_field_to_default_screen) | **POST** /rest/api/3/screens/addToDefault/{fieldId} | Add field to default screen
[**create_screen**](ScreensApi.md#create_screen) | **POST** /rest/api/3/screens | Create screen
[**delete_screen**](ScreensApi.md#delete_screen) | **DELETE** /rest/api/3/screens/{screenId} | Delete screen
[**get_available_screen_fields**](ScreensApi.md#get_available_screen_fields) | **GET** /rest/api/3/screens/{screenId}/availableFields | Get available screen fields
[**get_screens**](ScreensApi.md#get_screens) | **GET** /rest/api/3/screens | Get screens
[**get_screens_for_field**](ScreensApi.md#get_screens_for_field) | **GET** /rest/api/3/field/{fieldId}/screens | Get screens for a field
[**update_screen**](ScreensApi.md#update_screen) | **PUT** /rest/api/3/screens/{screenId} | Update screen



## add_field_to_default_screen

> serde_json::Value add_field_to_default_screen(field_id)
Add field to default screen

Adds a field to the default tab of the default screen.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **String** | The ID of the field. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_screen

> models::Screen create_screen(screen_details)
Create screen

Creates a screen with a default field tab.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**screen_details** | [**ScreenDetails**](ScreenDetails.md) |  | [required] |

### Return type

[**models::Screen**](Screen.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_screen

> delete_screen(screen_id)
Delete screen

Deletes a screen. A screen cannot be deleted if it is used in a screen scheme, workflow, or workflow draft.  Only screens used in classic projects can be deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**screen_id** | **i64** | The ID of the screen. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_available_screen_fields

> Vec<models::ScreenableField> get_available_screen_fields(screen_id)
Get available screen fields

Returns the fields that can be added to a tab on a screen.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**screen_id** | **i64** | The ID of the screen. | [required] |

### Return type

[**Vec<models::ScreenableField>**](ScreenableField.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_screens

> models::PageBeanScreen get_screens(start_at, max_results, id, query_string, scope, order_by)
Get screens

Returns a [paginated](#pagination) list of all screens or those specified by one or more screen IDs.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 100]
**id** | Option<[**Vec<i64>**](i64.md)> | The list of screen IDs. To include multiple IDs, provide an ampersand-separated list. For example, `id=10000&id=10001`. |  |
**query_string** | Option<**String**> | String used to perform a case-insensitive partial match with screen name. |  |[default to ]
**scope** | Option<[**Vec<String>**](String.md)> | The scope filter string. To filter by multiple scope, provide an ampersand-separated list. For example, `scope=GLOBAL&scope=PROJECT`. |  |
**order_by** | Option<**String**> | [Order](#ordering) the results by a field:   *  `id` Sorts by screen ID.  *  `name` Sorts by screen name. |  |

### Return type

[**models::PageBeanScreen**](PageBeanScreen.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_screens_for_field

> models::PageBeanScreenWithTab get_screens_for_field(field_id, start_at, max_results, expand)
Get screens for a field

Returns a [paginated](#pagination) list of the screens a field is used in.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **String** | The ID of the field to return screens for. | [required] |
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 100]
**expand** | Option<**String**> | Use [expand](#expansion) to include additional information about screens in the response. This parameter accepts `tab` which returns details about the screen tabs the field is used in. |  |

### Return type

[**models::PageBeanScreenWithTab**](PageBeanScreenWithTab.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_screen

> models::Screen update_screen(screen_id, update_screen_details)
Update screen

Updates a screen. Only screens used in classic projects can be updated.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**screen_id** | **i64** | The ID of the screen. | [required] |
**update_screen_details** | [**UpdateScreenDetails**](UpdateScreenDetails.md) |  | [required] |

### Return type

[**models::Screen**](Screen.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

