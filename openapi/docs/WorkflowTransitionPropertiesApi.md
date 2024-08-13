# \WorkflowTransitionPropertiesApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_workflow_transition_property**](WorkflowTransitionPropertiesApi.md#create_workflow_transition_property) | **POST** /rest/api/3/workflow/transitions/{transitionId}/properties | Create workflow transition property
[**delete_workflow_transition_property**](WorkflowTransitionPropertiesApi.md#delete_workflow_transition_property) | **DELETE** /rest/api/3/workflow/transitions/{transitionId}/properties | Delete workflow transition property
[**get_workflow_transition_properties**](WorkflowTransitionPropertiesApi.md#get_workflow_transition_properties) | **GET** /rest/api/3/workflow/transitions/{transitionId}/properties | Get workflow transition properties
[**update_workflow_transition_property**](WorkflowTransitionPropertiesApi.md#update_workflow_transition_property) | **PUT** /rest/api/3/workflow/transitions/{transitionId}/properties | Update workflow transition property



## create_workflow_transition_property

> models::WorkflowTransitionProperty create_workflow_transition_property(transition_id, key, workflow_name, workflow_transition_property, workflow_mode)
Create workflow transition property

Adds a property to a workflow transition. Transition properties are used to change the behavior of a transition. For more information, see [Transition properties](https://confluence.atlassian.com/x/zIhKLg#Advancedworkflowconfiguration-transitionproperties) and [Workflow properties](https://confluence.atlassian.com/x/JYlKLg).  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transition_id** | **i64** | The ID of the transition. To get the ID, view the workflow in text mode in the Jira admin settings. The ID is shown next to the transition. | [required] |
**key** | **String** | The key of the property being added, also known as the name of the property. Set this to the same value as the `key` defined in the request body. | [required] |
**workflow_name** | **String** | The name of the workflow that the transition belongs to. | [required] |
**workflow_transition_property** | [**WorkflowTransitionProperty**](WorkflowTransitionProperty.md) |  | [required] |
**workflow_mode** | Option<**String**> | The workflow status. Set to *live* for inactive workflows or *draft* for draft workflows. Active workflows cannot be edited. |  |[default to live]

### Return type

[**models::WorkflowTransitionProperty**](WorkflowTransitionProperty.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_workflow_transition_property

> delete_workflow_transition_property(transition_id, key, workflow_name, workflow_mode)
Delete workflow transition property

Deletes a property from a workflow transition. Transition properties are used to change the behavior of a transition. For more information, see [Transition properties](https://confluence.atlassian.com/x/zIhKLg#Advancedworkflowconfiguration-transitionproperties) and [Workflow properties](https://confluence.atlassian.com/x/JYlKLg).  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transition_id** | **i64** | The ID of the transition. To get the ID, view the workflow in text mode in the Jira admin settings. The ID is shown next to the transition. | [required] |
**key** | **String** | The name of the transition property to delete, also known as the name of the property. | [required] |
**workflow_name** | **String** | The name of the workflow that the transition belongs to. | [required] |
**workflow_mode** | Option<**String**> | The workflow status. Set to `live` for inactive workflows or `draft` for draft workflows. Active workflows cannot be edited. |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workflow_transition_properties

> models::WorkflowTransitionProperty get_workflow_transition_properties(transition_id, workflow_name, include_reserved_keys, key, workflow_mode)
Get workflow transition properties

Returns the properties on a workflow transition. Transition properties are used to change the behavior of a transition. For more information, see [Transition properties](https://confluence.atlassian.com/x/zIhKLg#Advancedworkflowconfiguration-transitionproperties) and [Workflow properties](https://confluence.atlassian.com/x/JYlKLg).  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transition_id** | **i64** | The ID of the transition. To get the ID, view the workflow in text mode in the Jira administration console. The ID is shown next to the transition. | [required] |
**workflow_name** | **String** | The name of the workflow that the transition belongs to. | [required] |
**include_reserved_keys** | Option<**bool**> | Some properties with keys that have the *jira.* prefix are reserved, which means they are not editable. To include these properties in the results, set this parameter to *true*. |  |[default to false]
**key** | Option<**String**> | The key of the property being returned, also known as the name of the property. If this parameter is not specified, all properties on the transition are returned. |  |
**workflow_mode** | Option<**String**> | The workflow status. Set to *live* for active and inactive workflows, or *draft* for draft workflows. |  |[default to live]

### Return type

[**models::WorkflowTransitionProperty**](WorkflowTransitionProperty.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_workflow_transition_property

> models::WorkflowTransitionProperty update_workflow_transition_property(transition_id, key, workflow_name, workflow_transition_property, workflow_mode)
Update workflow transition property

Updates a workflow transition by changing the property value. Trying to update a property that does not exist results in a new property being added to the transition. Transition properties are used to change the behavior of a transition. For more information, see [Transition properties](https://confluence.atlassian.com/x/zIhKLg#Advancedworkflowconfiguration-transitionproperties) and [Workflow properties](https://confluence.atlassian.com/x/JYlKLg).  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transition_id** | **i64** | The ID of the transition. To get the ID, view the workflow in text mode in the Jira admin settings. The ID is shown next to the transition. | [required] |
**key** | **String** | The key of the property being updated, also known as the name of the property. Set this to the same value as the `key` defined in the request body. | [required] |
**workflow_name** | **String** | The name of the workflow that the transition belongs to. | [required] |
**workflow_transition_property** | [**WorkflowTransitionProperty**](WorkflowTransitionProperty.md) |  | [required] |
**workflow_mode** | Option<**String**> | The workflow status. Set to `live` for inactive workflows or `draft` for draft workflows. Active workflows cannot be edited. |  |

### Return type

[**models::WorkflowTransitionProperty**](WorkflowTransitionProperty.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

