# \IssueNotificationSchemesApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_notifications**](IssueNotificationSchemesApi.md#add_notifications) | **PUT** /rest/api/3/notificationscheme/{id}/notification | Add notifications to notification scheme
[**create_notification_scheme**](IssueNotificationSchemesApi.md#create_notification_scheme) | **POST** /rest/api/3/notificationscheme | Create notification scheme
[**delete_notification_scheme**](IssueNotificationSchemesApi.md#delete_notification_scheme) | **DELETE** /rest/api/3/notificationscheme/{notificationSchemeId} | Delete notification scheme
[**get_notification_scheme**](IssueNotificationSchemesApi.md#get_notification_scheme) | **GET** /rest/api/3/notificationscheme/{id} | Get notification scheme
[**get_notification_scheme_to_project_mappings**](IssueNotificationSchemesApi.md#get_notification_scheme_to_project_mappings) | **GET** /rest/api/3/notificationscheme/project | Get projects using notification schemes paginated
[**get_notification_schemes**](IssueNotificationSchemesApi.md#get_notification_schemes) | **GET** /rest/api/3/notificationscheme | Get notification schemes paginated
[**remove_notification_from_notification_scheme**](IssueNotificationSchemesApi.md#remove_notification_from_notification_scheme) | **DELETE** /rest/api/3/notificationscheme/{notificationSchemeId}/notification/{notificationId} | Remove notification from notification scheme
[**update_notification_scheme**](IssueNotificationSchemesApi.md#update_notification_scheme) | **PUT** /rest/api/3/notificationscheme/{id} | Update notification scheme



## add_notifications

> serde_json::Value add_notifications(id, add_notifications_details)
Add notifications to notification scheme

Adds notifications to a notification scheme. You can add up to 1000 notifications per request.  *Deprecated: The notification type `EmailAddress` is no longer supported in Cloud. Refer to the [changelog](https://developer.atlassian.com/cloud/jira/platform/changelog/#CHANGE-1031) for more details.*  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the notification scheme. | [required] |
**add_notifications_details** | [**AddNotificationsDetails**](AddNotificationsDetails.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_notification_scheme

> models::NotificationSchemeId create_notification_scheme(create_notification_scheme_details)
Create notification scheme

Creates a notification scheme with notifications. You can create up to 1000 notifications per request.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_notification_scheme_details** | [**CreateNotificationSchemeDetails**](CreateNotificationSchemeDetails.md) |  | [required] |

### Return type

[**models::NotificationSchemeId**](NotificationSchemeId.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_notification_scheme

> serde_json::Value delete_notification_scheme(notification_scheme_id)
Delete notification scheme

Deletes a notification scheme.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_scheme_id** | **String** | The ID of the notification scheme. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notification_scheme

> models::NotificationScheme get_notification_scheme(id, expand)
Get notification scheme

Returns a [notification scheme](https://confluence.atlassian.com/x/8YdKLg), including the list of events and the recipients who will receive notifications for those events.  **[Permissions](#permissions) required:** Permission to access Jira, however, the user must have permission to administer at least one project associated with the notification scheme.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the notification scheme. Use [Get notification schemes paginated](#api-rest-api-3-notificationscheme-get) to get a list of notification scheme IDs. | [required] |
**expand** | Option<**String**> | Use [expand](#expansion) to include additional information in the response. This parameter accepts a comma-separated list. Expand options include:   *  `all` Returns all expandable information  *  `field` Returns information about any custom fields assigned to receive an event  *  `group` Returns information about any groups assigned to receive an event  *  `notificationSchemeEvents` Returns a list of event associations. This list is returned for all expandable information  *  `projectRole` Returns information about any project roles assigned to receive an event  *  `user` Returns information about any users assigned to receive an event |  |

### Return type

[**models::NotificationScheme**](NotificationScheme.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notification_scheme_to_project_mappings

> models::PageBeanNotificationSchemeAndProjectMappingJsonBean get_notification_scheme_to_project_mappings(start_at, max_results, notification_scheme_id, project_id)
Get projects using notification schemes paginated

Returns a [paginated](#pagination) mapping of project that have notification scheme assigned. You can provide either one or multiple notification scheme IDs or project IDs to filter by. If you don't provide any, this will return a list of all mappings. Note that only company-managed (classic) projects are supported. This is because team-managed projects don't have a concept of a default notification scheme. The mappings are ordered by projectId.  **[Permissions](#permissions) required:** Permission to access Jira.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_at** | Option<**String**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**String**> | The maximum number of items to return per page. |  |[default to 50]
**notification_scheme_id** | Option<[**Vec<String>**](String.md)> | The list of notifications scheme IDs to be filtered out |  |
**project_id** | Option<[**Vec<String>**](String.md)> | The list of project IDs to be filtered out |  |

### Return type

[**models::PageBeanNotificationSchemeAndProjectMappingJsonBean**](PageBeanNotificationSchemeAndProjectMappingJsonBean.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notification_schemes

> models::PageBeanNotificationScheme get_notification_schemes(start_at, max_results, id, project_id, only_default, expand)
Get notification schemes paginated

Returns a [paginated](#pagination) list of [notification schemes](https://confluence.atlassian.com/x/8YdKLg) ordered by the display name.  *Note that you should allow for events without recipients to appear in responses.*  **[Permissions](#permissions) required:** Permission to access Jira, however, the user must have permission to administer at least one project associated with a notification scheme for it to be returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_at** | Option<**String**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**String**> | The maximum number of items to return per page. |  |[default to 50]
**id** | Option<[**Vec<String>**](String.md)> | The list of notification schemes IDs to be filtered by |  |
**project_id** | Option<[**Vec<String>**](String.md)> | The list of projects IDs to be filtered by |  |
**only_default** | Option<**bool**> | When set to true, returns only the default notification scheme. If you provide project IDs not associated with the default, returns an empty page. The default value is false. |  |[default to false]
**expand** | Option<**String**> | Use [expand](#expansion) to include additional information in the response. This parameter accepts a comma-separated list. Expand options include:   *  `all` Returns all expandable information  *  `field` Returns information about any custom fields assigned to receive an event  *  `group` Returns information about any groups assigned to receive an event  *  `notificationSchemeEvents` Returns a list of event associations. This list is returned for all expandable information  *  `projectRole` Returns information about any project roles assigned to receive an event  *  `user` Returns information about any users assigned to receive an event |  |

### Return type

[**models::PageBeanNotificationScheme**](PageBeanNotificationScheme.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_notification_from_notification_scheme

> serde_json::Value remove_notification_from_notification_scheme(notification_scheme_id, notification_id)
Remove notification from notification scheme

Removes a notification from a notification scheme.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_scheme_id** | **String** | The ID of the notification scheme. | [required] |
**notification_id** | **String** | The ID of the notification. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_notification_scheme

> serde_json::Value update_notification_scheme(id, update_notification_scheme_details)
Update notification scheme

Updates a notification scheme.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the notification scheme. | [required] |
**update_notification_scheme_details** | [**UpdateNotificationSchemeDetails**](UpdateNotificationSchemeDetails.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

