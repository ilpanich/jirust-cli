# \ProjectFeaturesApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_features_for_project**](ProjectFeaturesApi.md#get_features_for_project) | **GET** /rest/api/3/project/{projectIdOrKey}/features | Get project features
[**toggle_feature_for_project**](ProjectFeaturesApi.md#toggle_feature_for_project) | **PUT** /rest/api/3/project/{projectIdOrKey}/features/{featureKey} | Set project feature state



## get_features_for_project

> models::ContainerForProjectFeatures get_features_for_project(project_id_or_key)
Get project features

Returns the list of features for a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id_or_key** | **String** | The ID or (case-sensitive) key of the project. | [required] |

### Return type

[**models::ContainerForProjectFeatures**](ContainerForProjectFeatures.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## toggle_feature_for_project

> models::ContainerForProjectFeatures toggle_feature_for_project(project_id_or_key, feature_key, project_feature_state)
Set project feature state

Sets the state of a project feature.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id_or_key** | **String** | The ID or (case-sensitive) key of the project. | [required] |
**feature_key** | **String** | The key of the feature. | [required] |
**project_feature_state** | [**ProjectFeatureState**](ProjectFeatureState.md) | Details of the feature state change. | [required] |

### Return type

[**models::ContainerForProjectFeatures**](ContainerForProjectFeatures.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

