# \DashboardsApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_gadget**](DashboardsApi.md#add_gadget) | **POST** /rest/api/3/dashboard/{dashboardId}/gadget | Add gadget to dashboard
[**bulk_edit_dashboards**](DashboardsApi.md#bulk_edit_dashboards) | **PUT** /rest/api/3/dashboard/bulk/edit | Bulk edit dashboards
[**copy_dashboard**](DashboardsApi.md#copy_dashboard) | **POST** /rest/api/3/dashboard/{id}/copy | Copy dashboard
[**create_dashboard**](DashboardsApi.md#create_dashboard) | **POST** /rest/api/3/dashboard | Create dashboard
[**delete_dashboard**](DashboardsApi.md#delete_dashboard) | **DELETE** /rest/api/3/dashboard/{id} | Delete dashboard
[**delete_dashboard_item_property**](DashboardsApi.md#delete_dashboard_item_property) | **DELETE** /rest/api/3/dashboard/{dashboardId}/items/{itemId}/properties/{propertyKey} | Delete dashboard item property
[**get_all_available_dashboard_gadgets**](DashboardsApi.md#get_all_available_dashboard_gadgets) | **GET** /rest/api/3/dashboard/gadgets | Get available gadgets
[**get_all_dashboards**](DashboardsApi.md#get_all_dashboards) | **GET** /rest/api/3/dashboard | Get all dashboards
[**get_all_gadgets**](DashboardsApi.md#get_all_gadgets) | **GET** /rest/api/3/dashboard/{dashboardId}/gadget | Get gadgets
[**get_dashboard**](DashboardsApi.md#get_dashboard) | **GET** /rest/api/3/dashboard/{id} | Get dashboard
[**get_dashboard_item_property**](DashboardsApi.md#get_dashboard_item_property) | **GET** /rest/api/3/dashboard/{dashboardId}/items/{itemId}/properties/{propertyKey} | Get dashboard item property
[**get_dashboard_item_property_keys**](DashboardsApi.md#get_dashboard_item_property_keys) | **GET** /rest/api/3/dashboard/{dashboardId}/items/{itemId}/properties | Get dashboard item property keys
[**get_dashboards_paginated**](DashboardsApi.md#get_dashboards_paginated) | **GET** /rest/api/3/dashboard/search | Search for dashboards
[**remove_gadget**](DashboardsApi.md#remove_gadget) | **DELETE** /rest/api/3/dashboard/{dashboardId}/gadget/{gadgetId} | Remove gadget from dashboard
[**set_dashboard_item_property**](DashboardsApi.md#set_dashboard_item_property) | **PUT** /rest/api/3/dashboard/{dashboardId}/items/{itemId}/properties/{propertyKey} | Set dashboard item property
[**update_dashboard**](DashboardsApi.md#update_dashboard) | **PUT** /rest/api/3/dashboard/{id} | Update dashboard
[**update_gadget**](DashboardsApi.md#update_gadget) | **PUT** /rest/api/3/dashboard/{dashboardId}/gadget/{gadgetId} | Update gadget on dashboard



## add_gadget

> models::DashboardGadget add_gadget(dashboard_id, dashboard_gadget_settings)
Add gadget to dashboard

Adds a gadget to a dashboard.  **[Permissions](#permissions) required:** None.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | **i64** | The ID of the dashboard. | [required] |
**dashboard_gadget_settings** | [**DashboardGadgetSettings**](DashboardGadgetSettings.md) |  | [required] |

### Return type

[**models::DashboardGadget**](DashboardGadget.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_edit_dashboards

> models::BulkEditShareableEntityResponse bulk_edit_dashboards(bulk_edit_shareable_entity_request)
Bulk edit dashboards

Bulk edit dashboards. Maximum number of dashboards to be edited at the same time is 100.  **[Permissions](#permissions) required:** None  The dashboards to be updated must be owned by the user, or the user must be an administrator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_edit_shareable_entity_request** | [**BulkEditShareableEntityRequest**](BulkEditShareableEntityRequest.md) | The details of dashboards being updated in bulk. | [required] |

### Return type

[**models::BulkEditShareableEntityResponse**](BulkEditShareableEntityResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## copy_dashboard

> models::Dashboard copy_dashboard(id, dashboard_details, extend_admin_permissions)
Copy dashboard

Copies a dashboard. Any values provided in the `dashboard` parameter replace those in the copied dashboard.  **[Permissions](#permissions) required:** None  The dashboard to be copied must be owned by or shared with the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**dashboard_details** | [**DashboardDetails**](DashboardDetails.md) | Dashboard details. | [required] |
**extend_admin_permissions** | Option<**bool**> | Whether admin level permissions are used. It should only be true if the user has *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) |  |[default to false]

### Return type

[**models::Dashboard**](Dashboard.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dashboard

> models::Dashboard create_dashboard(dashboard_details, extend_admin_permissions)
Create dashboard

Creates a dashboard.  **[Permissions](#permissions) required:** None.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_details** | [**DashboardDetails**](DashboardDetails.md) | Dashboard details. | [required] |
**extend_admin_permissions** | Option<**bool**> | Whether admin level permissions are used. It should only be true if the user has *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) |  |[default to false]

### Return type

[**models::Dashboard**](Dashboard.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dashboard

> delete_dashboard(id)
Delete dashboard

Deletes a dashboard.  **[Permissions](#permissions) required:** None  The dashboard to be deleted must be owned by the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the dashboard. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dashboard_item_property

> serde_json::Value delete_dashboard_item_property(dashboard_id, item_id, property_key)
Delete dashboard item property

Deletes a dashboard item property.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** The user must be the owner of the dashboard. Note, users with the *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) are considered owners of the System dashboard.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | **String** | The ID of the dashboard. | [required] |
**item_id** | **String** | The ID of the dashboard item. | [required] |
**property_key** | **String** | The key of the dashboard item property. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_available_dashboard_gadgets

> models::AvailableDashboardGadgetsResponse get_all_available_dashboard_gadgets()
Get available gadgets

Gets a list of all available gadgets that can be added to all dashboards.  **[Permissions](#permissions) required:** None.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AvailableDashboardGadgetsResponse**](AvailableDashboardGadgetsResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_dashboards

> models::PageOfDashboards get_all_dashboards(filter, start_at, max_results)
Get all dashboards

Returns a list of dashboards owned by or shared with the user. The list may be filtered to include only favorite or owned dashboards.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** None.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | The filter applied to the list of dashboards. Valid values are:   *  `favourite` Returns dashboards the user has marked as favorite.  *  `my` Returns dashboards owned by the user. |  |
**start_at** | Option<**i32**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 20]

### Return type

[**models::PageOfDashboards**](PageOfDashboards.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_gadgets

> models::DashboardGadgetResponse get_all_gadgets(dashboard_id, module_key, uri, gadget_id)
Get gadgets

Returns a list of dashboard gadgets on a dashboard.  This operation returns:   *  Gadgets from a list of IDs, when `id` is set.  *  Gadgets with a module key, when `moduleKey` is set.  *  Gadgets from a list of URIs, when `uri` is set.  *  All gadgets, when no other parameters are set.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** None.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | **i64** | The ID of the dashboard. | [required] |
**module_key** | Option<[**Vec<String>**](String.md)> | The list of gadgets module keys. To include multiple module keys, separate module keys with ampersand: `moduleKey=key:one&moduleKey=key:two`. |  |
**uri** | Option<[**Vec<String>**](String.md)> | The list of gadgets URIs. To include multiple URIs, separate URIs with ampersand: `uri=/rest/example/uri/1&uri=/rest/example/uri/2`. |  |
**gadget_id** | Option<[**Vec<i64>**](i64.md)> | The list of gadgets IDs. To include multiple IDs, separate IDs with ampersand: `gadgetId=10000&gadgetId=10001`. |  |

### Return type

[**models::DashboardGadgetResponse**](DashboardGadgetResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dashboard

> models::Dashboard get_dashboard(id)
Get dashboard

Returns a dashboard.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** None.  However, to get a dashboard, the dashboard must be shared with the user or the user must own it. Note, users with the *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) are considered owners of the System dashboard. The System dashboard is considered to be shared with all other users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the dashboard. | [required] |

### Return type

[**models::Dashboard**](Dashboard.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dashboard_item_property

> models::EntityProperty get_dashboard_item_property(dashboard_id, item_id, property_key)
Get dashboard item property

Returns the key and value of a dashboard item property.  A dashboard item enables an app to add user-specific information to a user dashboard. Dashboard items are exposed to users as gadgets that users can add to their dashboards. For more information on how users do this, see [Adding and customizing gadgets](https://confluence.atlassian.com/x/7AeiLQ).  When an app creates a dashboard item it registers a callback to receive the dashboard item ID. The callback fires whenever the item is rendered or, where the item is configurable, the user edits the item. The app then uses this resource to store the item's content or configuration details. For more information on working with dashboard items, see [ Building a dashboard item for a JIRA Connect add-on](https://developer.atlassian.com/server/jira/platform/guide-building-a-dashboard-item-for-a-jira-connect-add-on-33746254/) and the [Dashboard Item](https://developer.atlassian.com/cloud/jira/platform/modules/dashboard-item/) documentation.  There is no resource to set or get dashboard items.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** The user must be the owner of the dashboard or have the dashboard shared with them. Note, users with the *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) are considered owners of the System dashboard. The System dashboard is considered to be shared with all other users, and is accessible to anonymous users when Jira\\\\u2019s anonymous access is permitted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | **String** | The ID of the dashboard. | [required] |
**item_id** | **String** | The ID of the dashboard item. | [required] |
**property_key** | **String** | The key of the dashboard item property. | [required] |

### Return type

[**models::EntityProperty**](EntityProperty.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dashboard_item_property_keys

> models::PropertyKeys get_dashboard_item_property_keys(dashboard_id, item_id)
Get dashboard item property keys

Returns the keys of all properties for a dashboard item.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** The user must be the owner of the dashboard or have the dashboard shared with them. Note, users with the *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) are considered owners of the System dashboard. The System dashboard is considered to be shared with all other users, and is accessible to anonymous users when Jira\\\\u2019s anonymous access is permitted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | **String** | The ID of the dashboard. | [required] |
**item_id** | **String** | The ID of the dashboard item. | [required] |

### Return type

[**models::PropertyKeys**](PropertyKeys.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dashboards_paginated

> models::PageBeanDashboard get_dashboards_paginated(dashboard_name, account_id, owner, groupname, group_id, project_id, order_by, start_at, max_results, status, expand)
Search for dashboards

Returns a [paginated](#pagination) list of dashboards. This operation is similar to [Get dashboards](#api-rest-api-3-dashboard-get) except that the results can be refined to include dashboards that have specific attributes. For example, dashboards with a particular name. When multiple attributes are specified only filters matching all attributes are returned.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** The following dashboards that match the query parameters are returned:   *  Dashboards owned by the user. Not returned for anonymous users.  *  Dashboards shared with a group that the user is a member of. Not returned for anonymous users.  *  Dashboards shared with a private project that the user can browse. Not returned for anonymous users.  *  Dashboards shared with a public project.  *  Dashboards shared with the public.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_name** | Option<**String**> | String used to perform a case-insensitive partial match with `name`. |  |
**account_id** | Option<**String**> | User account ID used to return dashboards with the matching `owner.accountId`. This parameter cannot be used with the `owner` parameter. |  |
**owner** | Option<**String**> | This parameter is deprecated because of privacy changes. Use `accountId` instead. See the [migration guide](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details. User name used to return dashboards with the matching `owner.name`. This parameter cannot be used with the `accountId` parameter. |  |
**groupname** | Option<**String**> | As a group's name can change, use of `groupId` is recommended. Group name used to return dashboards that are shared with a group that matches `sharePermissions.group.name`. This parameter cannot be used with the `groupId` parameter. |  |
**group_id** | Option<**String**> | Group ID used to return dashboards that are shared with a group that matches `sharePermissions.group.groupId`. This parameter cannot be used with the `groupname` parameter. |  |
**project_id** | Option<**i64**> | Project ID used to returns dashboards that are shared with a project that matches `sharePermissions.project.id`. |  |
**order_by** | Option<**String**> | [Order](#ordering) the results by a field:   *  `description` Sorts by dashboard description. Note that this sort works independently of whether the expand to display the description field is in use.  *  `favourite_count` Sorts by dashboard popularity.  *  `id` Sorts by dashboard ID.  *  `is_favourite` Sorts by whether the dashboard is marked as a favorite.  *  `name` Sorts by dashboard name.  *  `owner` Sorts by dashboard owner name. |  |[default to name]
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 50]
**status** | Option<**String**> | The status to filter by. It may be active, archived or deleted. |  |[default to active]
**expand** | Option<**String**> | Use [expand](#expansion) to include additional information about dashboard in the response. This parameter accepts a comma-separated list. Expand options include:   *  `description` Returns the description of the dashboard.  *  `owner` Returns the owner of the dashboard.  *  `viewUrl` Returns the URL that is used to view the dashboard.  *  `favourite` Returns `isFavourite`, an indicator of whether the user has set the dashboard as a favorite.  *  `favouritedCount` Returns `popularity`, a count of how many users have set this dashboard as a favorite.  *  `sharePermissions` Returns details of the share permissions defined for the dashboard.  *  `editPermissions` Returns details of the edit permissions defined for the dashboard.  *  `isWritable` Returns whether the current user has permission to edit the dashboard. |  |

### Return type

[**models::PageBeanDashboard**](PageBeanDashboard.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_gadget

> serde_json::Value remove_gadget(dashboard_id, gadget_id)
Remove gadget from dashboard

Removes a dashboard gadget from a dashboard.  When a gadget is removed from a dashboard, other gadgets in the same column are moved up to fill the emptied position.  **[Permissions](#permissions) required:** None.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | **i64** | The ID of the dashboard. | [required] |
**gadget_id** | **i64** | The ID of the gadget. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_dashboard_item_property

> serde_json::Value set_dashboard_item_property(dashboard_id, item_id, property_key, body)
Set dashboard item property

Sets the value of a dashboard item property. Use this resource in apps to store custom data against a dashboard item.  A dashboard item enables an app to add user-specific information to a user dashboard. Dashboard items are exposed to users as gadgets that users can add to their dashboards. For more information on how users do this, see [Adding and customizing gadgets](https://confluence.atlassian.com/x/7AeiLQ).  When an app creates a dashboard item it registers a callback to receive the dashboard item ID. The callback fires whenever the item is rendered or, where the item is configurable, the user edits the item. The app then uses this resource to store the item's content or configuration details. For more information on working with dashboard items, see [ Building a dashboard item for a JIRA Connect add-on](https://developer.atlassian.com/server/jira/platform/guide-building-a-dashboard-item-for-a-jira-connect-add-on-33746254/) and the [Dashboard Item](https://developer.atlassian.com/cloud/jira/platform/modules/dashboard-item/) documentation.  There is no resource to set or get dashboard items.  The value of the request body must be a [valid](http://tools.ietf.org/html/rfc4627), non-empty JSON blob. The maximum length is 32768 characters.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** The user must be the owner of the dashboard. Note, users with the *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) are considered owners of the System dashboard.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | **String** | The ID of the dashboard. | [required] |
**item_id** | **String** | The ID of the dashboard item. | [required] |
**property_key** | **String** | The key of the dashboard item property. The maximum length is 255 characters. For dashboard items with a spec URI and no complete module key, if the provided propertyKey is equal to \"config\", the request body's JSON must be an object with all keys and values as strings. | [required] |
**body** | Option<**serde_json::Value**> | The value of the property. The value has to be a valid, non-empty [JSON](https://tools.ietf.org/html/rfc4627) value. The maximum length of the property value is 32768 bytes. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dashboard

> models::Dashboard update_dashboard(id, dashboard_details, extend_admin_permissions)
Update dashboard

Updates a dashboard, replacing all the dashboard details with those provided.  **[Permissions](#permissions) required:** None  The dashboard to be updated must be owned by the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the dashboard to update. | [required] |
**dashboard_details** | [**DashboardDetails**](DashboardDetails.md) | Replacement dashboard details. | [required] |
**extend_admin_permissions** | Option<**bool**> | Whether admin level permissions are used. It should only be true if the user has *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) |  |[default to false]

### Return type

[**models::Dashboard**](Dashboard.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_gadget

> serde_json::Value update_gadget(dashboard_id, gadget_id, dashboard_gadget_update_request)
Update gadget on dashboard

Changes the title, position, and color of the gadget on a dashboard.  **[Permissions](#permissions) required:** None.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | **i64** | The ID of the dashboard. | [required] |
**gadget_id** | **i64** | The ID of the gadget. | [required] |
**dashboard_gadget_update_request** | [**DashboardGadgetUpdateRequest**](DashboardGadgetUpdateRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

