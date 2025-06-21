# \IssueRedactionApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_redaction_status**](IssueRedactionApi.md#get_redaction_status) | **GET** /rest/api/3/redact/status/{jobId} | Get redaction status
[**redact**](IssueRedactionApi.md#redact) | **POST** /rest/api/3/redact | Redact



## get_redaction_status

> models::RedactionJobStatusResponse get_redaction_status(job_id)
Get redaction status

Retrieves the current status of a redaction job ID.  The jobStatus will be one of the following:   *  IN\\_PROGRESS - The redaction job is currently in progress  *  COMPLETED - The redaction job has completed successfully.  *  PENDING - The redaction job has not started yet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | Redaction job id | [required] |

### Return type

[**models::RedactionJobStatusResponse**](RedactionJobStatusResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## redact

> uuid::Uuid redact(bulk_redaction_request)
Redact

Submit a job to redact issue field data. This will trigger the redaction of the data in the specified fields asynchronously.  The redaction status can be polled using the job id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_redaction_request** | [**BulkRedactionRequest**](BulkRedactionRequest.md) | List of redaction requests | [required] |

### Return type

[**uuid::Uuid**](uuid::Uuid.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

