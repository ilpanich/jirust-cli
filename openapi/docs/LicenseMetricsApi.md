# \LicenseMetricsApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_approximate_application_license_count**](LicenseMetricsApi.md#get_approximate_application_license_count) | **GET** /rest/api/3/license/approximateLicenseCount/product/{applicationKey} | Get approximate application license count
[**get_approximate_license_count**](LicenseMetricsApi.md#get_approximate_license_count) | **GET** /rest/api/3/license/approximateLicenseCount | Get approximate license count
[**get_license**](LicenseMetricsApi.md#get_license) | **GET** /rest/api/3/instance/license | Get license



## get_approximate_application_license_count

> models::LicenseMetric get_approximate_application_license_count(application_key)
Get approximate application license count

Returns the total approximate number of user accounts for a single Jira license. Note that this information is cached with a 7-day lifecycle and could be stale at the time of call.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_key** | **String** | The ID of the application, represents a specific version of Jira. | [required] |

### Return type

[**models::LicenseMetric**](LicenseMetric.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_approximate_license_count

> models::LicenseMetric get_approximate_license_count()
Get approximate license count

Returns the approximate number of user accounts across all Jira licenses. Note that this information is cached with a 7-day lifecycle and could be stale at the time of call.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::LicenseMetric**](LicenseMetric.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_license

> models::License get_license()
Get license

Returns licensing information about the Jira instance.  **[Permissions](#permissions) required:** None.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::License**](License.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

