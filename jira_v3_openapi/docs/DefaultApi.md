# \DefaultApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_worklogs_by_issue_id_and_worklog_id**](DefaultApi.md#get_worklogs_by_issue_id_and_worklog_id) | **POST** /rest/internal/api/latest/worklog/bulk | Get worklogs by issue id and worklog id



## get_worklogs_by_issue_id_and_worklog_id

> models::BulkWorklogKeyResponseBean get_worklogs_by_issue_id_and_worklog_id(bulk_worklog_key_request_bean)
Get worklogs by issue id and worklog id

Returns worklog details for a list of issue ID and worklog ID pairs.  This is an internal API for bulk fetching worklogs by their issue and worklog IDs. Worklogs that don't exist will be filtered out from the response.  The returned list of worklogs is limited to 1000 items.  **[Permissions](#permissions) required:** This is an internal service-to-service API that requires ASAP authentication. No user permission checks are performed as this bypasses normal user context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_worklog_key_request_bean** | [**BulkWorklogKeyRequestBean**](BulkWorklogKeyRequestBean.md) | A JSON object containing a list of issue ID and worklog ID pairs. | [required] |

### Return type

[**models::BulkWorklogKeyResponseBean**](BulkWorklogKeyResponseBean.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

