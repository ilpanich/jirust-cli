# \ScreenSchemesApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_screen_scheme**](ScreenSchemesApi.md#create_screen_scheme) | **POST** /rest/api/3/screenscheme | Create screen scheme
[**delete_screen_scheme**](ScreenSchemesApi.md#delete_screen_scheme) | **DELETE** /rest/api/3/screenscheme/{screenSchemeId} | Delete screen scheme
[**get_screen_schemes**](ScreenSchemesApi.md#get_screen_schemes) | **GET** /rest/api/3/screenscheme | Get screen schemes
[**update_screen_scheme**](ScreenSchemesApi.md#update_screen_scheme) | **PUT** /rest/api/3/screenscheme/{screenSchemeId} | Update screen scheme



## create_screen_scheme

> models::ScreenSchemeId create_screen_scheme(screen_scheme_details)
Create screen scheme

Creates a screen scheme.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**screen_scheme_details** | [**ScreenSchemeDetails**](ScreenSchemeDetails.md) |  | [required] |

### Return type

[**models::ScreenSchemeId**](ScreenSchemeId.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_screen_scheme

> delete_screen_scheme(screen_scheme_id)
Delete screen scheme

Deletes a screen scheme. A screen scheme cannot be deleted if it is used in an issue type screen scheme.  Only screens schemes used in classic projects can be deleted.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**screen_scheme_id** | **String** | The ID of the screen scheme. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_screen_schemes

> models::PageBeanScreenScheme get_screen_schemes(start_at, max_results, id, expand, query_string, order_by)
Get screen schemes

Returns a [paginated](#pagination) list of screen schemes.  Only screen schemes used in classic projects are returned.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 25]
**id** | Option<[**Vec<i64>**](i64.md)> | The list of screen scheme IDs. To include multiple IDs, provide an ampersand-separated list. For example, `id=10000&id=10001`. |  |
**expand** | Option<**String**> | Use [expand](#expansion) include additional information in the response. This parameter accepts `issueTypeScreenSchemes` that, for each screen schemes, returns information about the issue type screen scheme the screen scheme is assigned to. |  |[default to ]
**query_string** | Option<**String**> | String used to perform a case-insensitive partial match with screen scheme name. |  |[default to ]
**order_by** | Option<**String**> | [Order](#ordering) the results by a field:   *  `id` Sorts by screen scheme ID.  *  `name` Sorts by screen scheme name. |  |

### Return type

[**models::PageBeanScreenScheme**](PageBeanScreenScheme.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_screen_scheme

> serde_json::Value update_screen_scheme(screen_scheme_id, update_screen_scheme_details)
Update screen scheme

Updates a screen scheme. Only screen schemes used in classic projects can be updated.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**screen_scheme_id** | **String** | The ID of the screen scheme. | [required] |
**update_screen_scheme_details** | [**UpdateScreenSchemeDetails**](UpdateScreenSchemeDetails.md) | The screen scheme update details. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

