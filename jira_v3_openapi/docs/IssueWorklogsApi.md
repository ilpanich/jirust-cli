# \IssueWorklogsApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_worklog**](IssueWorklogsApi.md#add_worklog) | **POST** /rest/api/3/issue/{issueIdOrKey}/worklog | Add worklog
[**bulk_delete_worklogs**](IssueWorklogsApi.md#bulk_delete_worklogs) | **DELETE** /rest/api/3/issue/{issueIdOrKey}/worklog | Bulk delete worklogs
[**bulk_move_worklogs**](IssueWorklogsApi.md#bulk_move_worklogs) | **POST** /rest/api/3/issue/{issueIdOrKey}/worklog/move | Bulk move worklogs
[**delete_worklog**](IssueWorklogsApi.md#delete_worklog) | **DELETE** /rest/api/3/issue/{issueIdOrKey}/worklog/{id} | Delete worklog
[**get_ids_of_worklogs_deleted_since**](IssueWorklogsApi.md#get_ids_of_worklogs_deleted_since) | **GET** /rest/api/3/worklog/deleted | Get IDs of deleted worklogs
[**get_ids_of_worklogs_modified_since**](IssueWorklogsApi.md#get_ids_of_worklogs_modified_since) | **GET** /rest/api/3/worklog/updated | Get IDs of updated worklogs
[**get_issue_worklog**](IssueWorklogsApi.md#get_issue_worklog) | **GET** /rest/api/3/issue/{issueIdOrKey}/worklog | Get issue worklogs
[**get_worklog**](IssueWorklogsApi.md#get_worklog) | **GET** /rest/api/3/issue/{issueIdOrKey}/worklog/{id} | Get worklog
[**get_worklogs_for_ids**](IssueWorklogsApi.md#get_worklogs_for_ids) | **POST** /rest/api/3/worklog/list | Get worklogs
[**update_worklog**](IssueWorklogsApi.md#update_worklog) | **PUT** /rest/api/3/issue/{issueIdOrKey}/worklog/{id} | Update worklog



## add_worklog

> models::Worklog add_worklog(issue_id_or_key, worklog, notify_users, adjust_estimate, new_estimate, reduce_by, expand, override_editable_flag)
Add worklog

Adds a worklog to an issue.  Time tracking must be enabled in Jira, otherwise this operation returns an error. For more information, see [Configuring time tracking](https://confluence.atlassian.com/x/qoXKM).  This operation can be accessed anonymously.  **[Permissions](#permissions) required:**   *  *Browse projects* and *Work on issues* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue is in.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id_or_key** | **String** | The ID or key the issue. | [required] |
**worklog** | [**Worklog**](Worklog.md) |  | [required] |
**notify_users** | Option<**bool**> | Whether users watching the issue are notified by email. |  |[default to true]
**adjust_estimate** | Option<**String**> | Defines how to update the issue's time estimate, the options are:   *  `new` Sets the estimate to a specific value, defined in `newEstimate`.  *  `leave` Leaves the estimate unchanged.  *  `manual` Reduces the estimate by amount specified in `reduceBy`.  *  `auto` Reduces the estimate by the value of `timeSpent` in the worklog. |  |[default to auto]
**new_estimate** | Option<**String**> | The value to set as the issue's remaining time estimate, as days (\\#d), hours (\\#h), or minutes (\\#m or \\#). For example, *2d*. Required when `adjustEstimate` is `new`. |  |
**reduce_by** | Option<**String**> | The amount to reduce the issue's remaining estimate by, as days (\\#d), hours (\\#h), or minutes (\\#m). For example, *2d*. Required when `adjustEstimate` is `manual`. |  |
**expand** | Option<**String**> | Use [expand](#expansion) to include additional information about work logs in the response. This parameter accepts `properties`, which returns worklog properties. |  |[default to ]
**override_editable_flag** | Option<**bool**> | Whether the worklog entry should be added to the issue even if the issue is not editable, because jira.issue.editable set to false or missing. For example, the issue is closed. Connect and Forge app users with *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) can use this flag. |  |[default to false]

### Return type

[**models::Worklog**](Worklog.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_delete_worklogs

> bulk_delete_worklogs(issue_id_or_key, worklog_ids_request_bean, adjust_estimate, override_editable_flag)
Bulk delete worklogs

Deletes a list of worklogs from an issue. This is an experimental API with limitations:   *  You can't delete more than 5000 worklogs at once.  *  No notifications will be sent for deleted worklogs.  Time tracking must be enabled in Jira, otherwise this operation returns an error. For more information, see [Configuring time tracking](https://confluence.atlassian.com/x/qoXKM).  **[Permissions](#permissions) required:**   *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the issue.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.  *  *Delete all worklogs*[ project permission](https://confluence.atlassian.com/x/yodKLg) to delete any worklog.  *  If any worklog has visibility restrictions, belongs to the group or has the role visibility is restricted to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id_or_key** | **String** | The ID or key of the issue. | [required] |
**worklog_ids_request_bean** | [**WorklogIdsRequestBean**](WorklogIdsRequestBean.md) | A JSON object containing a list of worklog IDs. | [required] |
**adjust_estimate** | Option<**String**> | Defines how to update the issue's time estimate, the options are:   *  `leave` Leaves the estimate unchanged.  *  `auto` Reduces the estimate by the aggregate value of `timeSpent` across all worklogs being deleted. |  |[default to auto]
**override_editable_flag** | Option<**bool**> | Whether the work log entries should be removed to the issue even if the issue is not editable, because jira.issue.editable set to false or missing. For example, the issue is closed. Connect and Forge app users with admin permission can use this flag. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_move_worklogs

> bulk_move_worklogs(issue_id_or_key, worklogs_move_request_bean, adjust_estimate, override_editable_flag)
Bulk move worklogs

Moves a list of worklogs from one issue to another. This is an experimental API with several limitations:   *  You can't move more than 5000 worklogs at once.  *  You can't move worklogs containing an attachment.  *  You can't move worklogs restricted by project roles.  *  No notifications will be sent for moved worklogs.  *  No webhooks or events will be sent for moved worklogs.  *  No issue history will be recorded for moved worklogs.  Time tracking must be enabled in Jira, otherwise this operation returns an error. For more information, see [Configuring time tracking](https://confluence.atlassian.com/x/qoXKM).  **[Permissions](#permissions) required:**   *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the projects containing the source and destination issues.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.  *  *Delete all worklogs*[ and *Edit all worklogs*](https://confluence.atlassian.com/x/yodKLg)[project permission](https://confluence.atlassian.com/x/yodKLg)  *  If the worklog has visibility restrictions, belongs to the group or has the role visibility is restricted to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id_or_key** | **String** |  | [required] |
**worklogs_move_request_bean** | [**WorklogsMoveRequestBean**](WorklogsMoveRequestBean.md) | A JSON object containing a list of worklog IDs and the ID or key of the destination issue. | [required] |
**adjust_estimate** | Option<**String**> | Defines how to update the issues' time estimate, the options are:   *  `leave` Leaves the estimate unchanged.  *  `auto` Reduces the estimate by the aggregate value of `timeSpent` across all worklogs being moved in the source issue, and increases it in the destination issue. |  |[default to auto]
**override_editable_flag** | Option<**bool**> | Whether the work log entry should be moved to and from the issues even if the issues are not editable, because jira.issue.editable set to false or missing. For example, the issue is closed. Connect and Forge app users with admin permission can use this flag. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_worklog

> delete_worklog(issue_id_or_key, id, notify_users, adjust_estimate, new_estimate, increase_by, override_editable_flag)
Delete worklog

Deletes a worklog from an issue.  Time tracking must be enabled in Jira, otherwise this operation returns an error. For more information, see [Configuring time tracking](https://confluence.atlassian.com/x/qoXKM).  This operation can be accessed anonymously.  **[Permissions](#permissions) required:**   *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue is in.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.  *  *Delete all worklogs*[ project permission](https://confluence.atlassian.com/x/yodKLg) to delete any worklog or *Delete own worklogs* to delete worklogs created by the user,  *  If the worklog has visibility restrictions, belongs to the group or has the role visibility is restricted to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id_or_key** | **String** | The ID or key of the issue. | [required] |
**id** | **String** | The ID of the worklog. | [required] |
**notify_users** | Option<**bool**> | Whether users watching the issue are notified by email. |  |[default to true]
**adjust_estimate** | Option<**String**> | Defines how to update the issue's time estimate, the options are:   *  `new` Sets the estimate to a specific value, defined in `newEstimate`.  *  `leave` Leaves the estimate unchanged.  *  `manual` Increases the estimate by amount specified in `increaseBy`.  *  `auto` Reduces the estimate by the value of `timeSpent` in the worklog. |  |[default to auto]
**new_estimate** | Option<**String**> | The value to set as the issue's remaining time estimate, as days (\\#d), hours (\\#h), or minutes (\\#m or \\#). For example, *2d*. Required when `adjustEstimate` is `new`. |  |
**increase_by** | Option<**String**> | The amount to increase the issue's remaining estimate by, as days (\\#d), hours (\\#h), or minutes (\\#m or \\#). For example, *2d*. Required when `adjustEstimate` is `manual`. |  |
**override_editable_flag** | Option<**bool**> | Whether the work log entry should be added to the issue even if the issue is not editable, because jira.issue.editable set to false or missing. For example, the issue is closed. Connect and Forge app users with admin permission can use this flag. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ids_of_worklogs_deleted_since

> models::ChangedWorklogs get_ids_of_worklogs_deleted_since(since)
Get IDs of deleted worklogs

Returns a list of IDs and delete timestamps for worklogs deleted after a date and time.  This resource is paginated, with a limit of 1000 worklogs per page. Each page lists worklogs from oldest to youngest. If the number of items in the date range exceeds 1000, `until` indicates the timestamp of the youngest item on the page. Also, `nextPage` provides the URL for the next page of worklogs. The `lastPage` parameter is set to true on the last page of worklogs.  This resource does not return worklogs deleted during the minute preceding the request.  **[Permissions](#permissions) required:** Permission to access Jira.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**since** | Option<**i64**> | The date and time, as a UNIX timestamp in milliseconds, after which deleted worklogs are returned. |  |[default to 0]

### Return type

[**models::ChangedWorklogs**](ChangedWorklogs.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ids_of_worklogs_modified_since

> models::ChangedWorklogs get_ids_of_worklogs_modified_since(since, expand)
Get IDs of updated worklogs

Returns a list of IDs and update timestamps for worklogs updated after a date and time.  This resource is paginated, with a limit of 1000 worklogs per page. Each page lists worklogs from oldest to youngest. If the number of items in the date range exceeds 1000, `until` indicates the timestamp of the youngest item on the page. Also, `nextPage` provides the URL for the next page of worklogs. The `lastPage` parameter is set to true on the last page of worklogs.  This resource does not return worklogs updated during the minute preceding the request.  **[Permissions](#permissions) required:** Permission to access Jira, however, worklogs are only returned where either of the following is true:   *  the worklog is set as *Viewable by All Users*.  *  the user is a member of a project role or group with permission to view the worklog.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**since** | Option<**i64**> | The date and time, as a UNIX timestamp in milliseconds, after which updated worklogs are returned. |  |[default to 0]
**expand** | Option<**String**> | Use [expand](#expansion) to include additional information about worklogs in the response. This parameter accepts `properties` that returns the properties of each worklog. |  |[default to ]

### Return type

[**models::ChangedWorklogs**](ChangedWorklogs.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_issue_worklog

> models::PageOfWorklogs get_issue_worklog(issue_id_or_key, start_at, max_results, started_after, started_before, expand)
Get issue worklogs

Returns worklogs for an issue (ordered by created time), starting from the oldest worklog or from the worklog started on or after a date and time.  Time tracking must be enabled in Jira, otherwise this operation returns an error. For more information, see [Configuring time tracking](https://confluence.atlassian.com/x/qoXKM).  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** Workloads are only returned where the user has:   *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue is in.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.  *  If the worklog has visibility restrictions, belongs to the group or has the role visibility is restricted to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id_or_key** | **String** | The ID or key of the issue. | [required] |
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 5000]
**started_after** | Option<**i64**> | The worklog start date and time, as a UNIX timestamp in milliseconds, after which worklogs are returned. |  |
**started_before** | Option<**i64**> | The worklog start date and time, as a UNIX timestamp in milliseconds, before which worklogs are returned. |  |
**expand** | Option<**String**> | Use [expand](#expansion) to include additional information about worklogs in the response. This parameter accepts`properties`, which returns worklog properties. |  |[default to ]

### Return type

[**models::PageOfWorklogs**](PageOfWorklogs.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_worklog

> models::Worklog get_worklog(issue_id_or_key, id, expand)
Get worklog

Returns a worklog.  Time tracking must be enabled in Jira, otherwise this operation returns an error. For more information, see [Configuring time tracking](https://confluence.atlassian.com/x/qoXKM).  This operation can be accessed anonymously.  **[Permissions](#permissions) required:**   *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue is in.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.  *  If the worklog has visibility restrictions, belongs to the group or has the role visibility is restricted to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id_or_key** | **String** | The ID or key of the issue. | [required] |
**id** | **String** | The ID of the worklog. | [required] |
**expand** | Option<**String**> | Use [expand](#expansion) to include additional information about work logs in the response. This parameter accepts  `properties`, which returns worklog properties. |  |[default to ]

### Return type

[**models::Worklog**](Worklog.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_worklogs_for_ids

> Vec<models::Worklog> get_worklogs_for_ids(worklog_ids_request_bean, expand)
Get worklogs

Returns worklog details for a list of worklog IDs.  The returned list of worklogs is limited to 1000 items.  **[Permissions](#permissions) required:** Permission to access Jira, however, worklogs are only returned where either of the following is true:   *  the worklog is set as *Viewable by All Users*.  *  the user is a member of a project role or group with permission to view the worklog.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**worklog_ids_request_bean** | [**WorklogIdsRequestBean**](WorklogIdsRequestBean.md) | A JSON object containing a list of worklog IDs. | [required] |
**expand** | Option<**String**> | Use [expand](#expansion) to include additional information about worklogs in the response. This parameter accepts `properties` that returns the properties of each worklog. |  |[default to ]

### Return type

[**Vec<models::Worklog>**](Worklog.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_worklog

> models::Worklog update_worklog(issue_id_or_key, id, worklog, notify_users, adjust_estimate, new_estimate, expand, override_editable_flag)
Update worklog

Updates a worklog.  Time tracking must be enabled in Jira, otherwise this operation returns an error. For more information, see [Configuring time tracking](https://confluence.atlassian.com/x/qoXKM).  This operation can be accessed anonymously.  **[Permissions](#permissions) required:**   *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue is in.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.  *  *Edit all worklogs*[ project permission](https://confluence.atlassian.com/x/yodKLg) to update any worklog or *Edit own worklogs* to update worklogs created by the user.  *  If the worklog has visibility restrictions, belongs to the group or has the role visibility is restricted to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_id_or_key** | **String** | The ID or key the issue. | [required] |
**id** | **String** | The ID of the worklog. | [required] |
**worklog** | [**Worklog**](Worklog.md) |  | [required] |
**notify_users** | Option<**bool**> | Whether users watching the issue are notified by email. |  |[default to true]
**adjust_estimate** | Option<**String**> | Defines how to update the issue's time estimate, the options are:   *  `new` Sets the estimate to a specific value, defined in `newEstimate`.  *  `leave` Leaves the estimate unchanged.  *  `auto` Updates the estimate by the difference between the original and updated value of `timeSpent` or `timeSpentSeconds`. |  |[default to auto]
**new_estimate** | Option<**String**> | The value to set as the issue's remaining time estimate, as days (\\#d), hours (\\#h), or minutes (\\#m or \\#). For example, *2d*. Required when `adjustEstimate` is `new`. |  |
**expand** | Option<**String**> | Use [expand](#expansion) to include additional information about worklogs in the response. This parameter accepts `properties`, which returns worklog properties. |  |[default to ]
**override_editable_flag** | Option<**bool**> | Whether the worklog should be added to the issue even if the issue is not editable. For example, because the issue is closed. Connect and Forge app users with *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) can use this flag. |  |[default to false]

### Return type

[**models::Worklog**](Worklog.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

