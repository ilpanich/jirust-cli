# \WebhooksApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_webhook_by_id**](WebhooksApi.md#delete_webhook_by_id) | **DELETE** /rest/api/3/webhook | Delete webhooks by ID
[**get_dynamic_webhooks_for_app**](WebhooksApi.md#get_dynamic_webhooks_for_app) | **GET** /rest/api/3/webhook | Get dynamic webhooks for app
[**get_failed_webhooks**](WebhooksApi.md#get_failed_webhooks) | **GET** /rest/api/3/webhook/failed | Get failed webhooks
[**refresh_webhooks**](WebhooksApi.md#refresh_webhooks) | **PUT** /rest/api/3/webhook/refresh | Extend webhook life
[**register_dynamic_webhooks**](WebhooksApi.md#register_dynamic_webhooks) | **POST** /rest/api/3/webhook | Register dynamic webhooks



## delete_webhook_by_id

> delete_webhook_by_id(container_for_webhook_ids)
Delete webhooks by ID

Removes webhooks by ID. Only webhooks registered by the calling app are removed. If webhooks created by other apps are specified, they are ignored.  **[Permissions](#permissions) required:** Only [Connect](https://developer.atlassian.com/cloud/jira/platform/#connect-apps) and [OAuth 2.0](https://developer.atlassian.com/cloud/jira/platform/oauth-2-3lo-apps) apps can use this operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_for_webhook_ids** | [**ContainerForWebhookIds**](ContainerForWebhookIds.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dynamic_webhooks_for_app

> models::PageBeanWebhook get_dynamic_webhooks_for_app(start_at, max_results)
Get dynamic webhooks for app

Returns a [paginated](#pagination) list of the webhooks registered by the calling app.  **[Permissions](#permissions) required:** Only [Connect](https://developer.atlassian.com/cloud/jira/platform/#connect-apps) and [OAuth 2.0](https://developer.atlassian.com/cloud/jira/platform/oauth-2-3lo-apps) apps can use this operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 100]

### Return type

[**models::PageBeanWebhook**](PageBeanWebhook.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_failed_webhooks

> models::FailedWebhooks get_failed_webhooks(max_results, after)
Get failed webhooks

Returns webhooks that have recently failed to be delivered to the requesting app after the maximum number of retries.  After 72 hours the failure may no longer be returned by this operation.  The oldest failure is returned first.  This method uses a cursor-based pagination. To request the next page use the failure time of the last webhook on the list as the `failedAfter` value or use the URL provided in `next`.  **[Permissions](#permissions) required:** Only [Connect apps](https://developer.atlassian.com/cloud/jira/platform/index/#connect-apps) can use this operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**max_results** | Option<**i32**> | The maximum number of webhooks to return per page. If obeying the maxResults directive would result in records with the same failure time being split across pages, the directive is ignored and all records with the same failure time included on the page. |  |
**after** | Option<**i64**> | The time after which any webhook failure must have occurred for the record to be returned, expressed as milliseconds since the UNIX epoch. |  |

### Return type

[**models::FailedWebhooks**](FailedWebhooks.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_webhooks

> models::WebhooksExpirationDate refresh_webhooks(container_for_webhook_ids)
Extend webhook life

Extends the life of webhook. Webhooks registered through the REST API expire after 30 days. Call this operation to keep them alive.  Unrecognized webhook IDs (those that are not found or belong to other apps) are ignored.  **[Permissions](#permissions) required:** Only [Connect](https://developer.atlassian.com/cloud/jira/platform/#connect-apps) and [OAuth 2.0](https://developer.atlassian.com/cloud/jira/platform/oauth-2-3lo-apps) apps can use this operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_for_webhook_ids** | [**ContainerForWebhookIds**](ContainerForWebhookIds.md) |  | [required] |

### Return type

[**models::WebhooksExpirationDate**](WebhooksExpirationDate.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_dynamic_webhooks

> models::ContainerForRegisteredWebhooks register_dynamic_webhooks(webhook_registration_details)
Register dynamic webhooks

Registers webhooks.  **NOTE:** for non-public OAuth apps, webhooks are delivered only if there is a match between the app owner and the user who registered a dynamic webhook.  **[Permissions](#permissions) required:** Only [Connect](https://developer.atlassian.com/cloud/jira/platform/#connect-apps) and [OAuth 2.0](https://developer.atlassian.com/cloud/jira/platform/oauth-2-3lo-apps) apps can use this operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_registration_details** | [**WebhookRegistrationDetails**](WebhookRegistrationDetails.md) |  | [required] |

### Return type

[**models::ContainerForRegisteredWebhooks**](ContainerForRegisteredWebhooks.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

