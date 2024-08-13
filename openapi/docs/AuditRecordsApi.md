# \AuditRecordsApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_audit_records**](AuditRecordsApi.md#get_audit_records) | **GET** /rest/api/3/auditing/record | Get audit records



## get_audit_records

> models::AuditRecords get_audit_records(offset, limit, filter, from, to)
Get audit records

Returns a list of audit records. The list can be filtered to include items:   *  where each item in `filter` has at least one match in any of these fields:           *  `summary`      *  `category`      *  `eventSource`      *  `objectItem.name` If the object is a user, account ID is available to filter.      *  `objectItem.parentName`      *  `objectItem.typeName`      *  `changedValues.changedFrom`      *  `changedValues.changedTo`      *  `remoteAddress`          For example, if `filter` contains *man ed*, an audit record containing `summary\": \"User added to group\"` and `\"category\": \"group management\"` is returned.  *  created on or after a date and time.  *  created or or before a date and time.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | The number of records to skip before returning the first result. |  |[default to 0]
**limit** | Option<**i32**> | The maximum number of results to return. |  |[default to 1000]
**filter** | Option<**String**> | The strings to match with audit field content, space separated. |  |
**from** | Option<**String**> | The date and time on or after which returned audit records must have been created. If `to` is provided `from` must be before `to` or no audit records are returned. |  |
**to** | Option<**String**> | The date and time on or before which returned audit results must have been created. If `from` is provided `to` must be after `from` or no audit records are returned. |  |

### Return type

[**models::AuditRecords**](AuditRecords.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

