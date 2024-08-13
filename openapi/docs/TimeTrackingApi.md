# \TimeTrackingApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_available_time_tracking_implementations**](TimeTrackingApi.md#get_available_time_tracking_implementations) | **GET** /rest/api/3/configuration/timetracking/list | Get all time tracking providers
[**get_selected_time_tracking_implementation**](TimeTrackingApi.md#get_selected_time_tracking_implementation) | **GET** /rest/api/3/configuration/timetracking | Get selected time tracking provider
[**get_shared_time_tracking_configuration**](TimeTrackingApi.md#get_shared_time_tracking_configuration) | **GET** /rest/api/3/configuration/timetracking/options | Get time tracking settings
[**select_time_tracking_implementation**](TimeTrackingApi.md#select_time_tracking_implementation) | **PUT** /rest/api/3/configuration/timetracking | Select time tracking provider
[**set_shared_time_tracking_configuration**](TimeTrackingApi.md#set_shared_time_tracking_configuration) | **PUT** /rest/api/3/configuration/timetracking/options | Set time tracking settings



## get_available_time_tracking_implementations

> Vec<models::TimeTrackingProvider> get_available_time_tracking_implementations()
Get all time tracking providers

Returns all time tracking providers. By default, Jira only has one time tracking provider: *JIRA provided time tracking*. However, you can install other time tracking providers via apps from the Atlassian Marketplace. For more information on time tracking providers, see the documentation for the [ Time Tracking Provider](https://developer.atlassian.com/cloud/jira/platform/modules/time-tracking-provider/) module.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::TimeTrackingProvider>**](TimeTrackingProvider.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_selected_time_tracking_implementation

> models::TimeTrackingProvider get_selected_time_tracking_implementation()
Get selected time tracking provider

Returns the time tracking provider that is currently selected. Note that if time tracking is disabled, then a successful but empty response is returned.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::TimeTrackingProvider**](TimeTrackingProvider.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_shared_time_tracking_configuration

> models::TimeTrackingConfiguration get_shared_time_tracking_configuration()
Get time tracking settings

Returns the time tracking settings. This includes settings such as the time format, default time unit, and others. For more information, see [Configuring time tracking](https://confluence.atlassian.com/x/qoXKM).  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::TimeTrackingConfiguration**](TimeTrackingConfiguration.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## select_time_tracking_implementation

> serde_json::Value select_time_tracking_implementation(time_tracking_provider)
Select time tracking provider

Selects a time tracking provider.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**time_tracking_provider** | [**TimeTrackingProvider**](TimeTrackingProvider.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_shared_time_tracking_configuration

> models::TimeTrackingConfiguration set_shared_time_tracking_configuration(time_tracking_configuration)
Set time tracking settings

Sets the time tracking settings.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**time_tracking_configuration** | [**TimeTrackingConfiguration**](TimeTrackingConfiguration.md) |  | [required] |

### Return type

[**models::TimeTrackingConfiguration**](TimeTrackingConfiguration.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

