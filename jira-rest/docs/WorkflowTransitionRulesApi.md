# \WorkflowTransitionRulesApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_workflow_transition_rule_configurations**](WorkflowTransitionRulesApi.md#delete_workflow_transition_rule_configurations) | **PUT** /rest/api/3/workflow/rule/config/delete | Delete workflow transition rule configurations
[**get_workflow_transition_rule_configurations**](WorkflowTransitionRulesApi.md#get_workflow_transition_rule_configurations) | **GET** /rest/api/3/workflow/rule/config | Get workflow transition rule configurations
[**update_workflow_transition_rule_configurations**](WorkflowTransitionRulesApi.md#update_workflow_transition_rule_configurations) | **PUT** /rest/api/3/workflow/rule/config | Update workflow transition rule configurations



## delete_workflow_transition_rule_configurations

> models::WorkflowTransitionRulesUpdateErrors delete_workflow_transition_rule_configurations(workflows_with_transition_rules_details)
Delete workflow transition rule configurations

Deletes workflow transition rules from one or more workflows. These rule types are supported:   *  [post functions](https://developer.atlassian.com/cloud/jira/platform/modules/workflow-post-function/)  *  [conditions](https://developer.atlassian.com/cloud/jira/platform/modules/workflow-condition/)  *  [validators](https://developer.atlassian.com/cloud/jira/platform/modules/workflow-validator/)  Only rules created by the calling Connect app can be deleted.  **[Permissions](#permissions) required:** Only Connect apps can use this operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workflows_with_transition_rules_details** | [**WorkflowsWithTransitionRulesDetails**](WorkflowsWithTransitionRulesDetails.md) |  | [required] |

### Return type

[**models::WorkflowTransitionRulesUpdateErrors**](WorkflowTransitionRulesUpdateErrors.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workflow_transition_rule_configurations

> models::PageBeanWorkflowTransitionRules get_workflow_transition_rule_configurations(types, start_at, max_results, keys, workflow_names, with_tags, draft, expand)
Get workflow transition rule configurations

Returns a [paginated](#pagination) list of workflows with transition rules. The workflows can be filtered to return only those containing workflow transition rules:   *  of one or more transition rule types, such as [workflow post functions](https://developer.atlassian.com/cloud/jira/platform/modules/workflow-post-function/).  *  matching one or more transition rule keys.  Only workflows containing transition rules created by the calling [Connect](https://developer.atlassian.com/cloud/jira/platform/index/#connect-apps) or [Forge](https://developer.atlassian.com/cloud/jira/platform/index/#forge-apps) app are returned.  Due to server-side optimizations, workflows with an empty list of rules may be returned; these workflows can be ignored.  **[Permissions](#permissions) required:** Only [Connect](https://developer.atlassian.com/cloud/jira/platform/index/#connect-apps) or [Forge](https://developer.atlassian.com/cloud/jira/platform/index/#forge-apps) apps can use this operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**types** | [**Vec<String>**](String.md) | The types of the transition rules to return. | [required] |
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 10]
**keys** | Option<[**Vec<String>**](String.md)> | The transition rule class keys, as defined in the Connect or the Forge app descriptor, of the transition rules to return. |  |
**workflow_names** | Option<[**Vec<String>**](String.md)> | The list of workflow names to filter by. |  |
**with_tags** | Option<[**Vec<String>**](String.md)> | The list of `tags` to filter by. |  |
**draft** | Option<**bool**> | Whether draft or published workflows are returned. If not provided, both workflow types are returned. |  |
**expand** | Option<**String**> | Use [expand](#expansion) to include additional information in the response. This parameter accepts `transition`, which, for each rule, returns information about the transition the rule is assigned to. |  |

### Return type

[**models::PageBeanWorkflowTransitionRules**](PageBeanWorkflowTransitionRules.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_workflow_transition_rule_configurations

> models::WorkflowTransitionRulesUpdateErrors update_workflow_transition_rule_configurations(workflow_transition_rules_update)
Update workflow transition rule configurations

Updates configuration of workflow transition rules. The following rule types are supported:   *  [post functions](https://developer.atlassian.com/cloud/jira/platform/modules/workflow-post-function/)  *  [conditions](https://developer.atlassian.com/cloud/jira/platform/modules/workflow-condition/)  *  [validators](https://developer.atlassian.com/cloud/jira/platform/modules/workflow-validator/)  Only rules created by the calling [Connect](https://developer.atlassian.com/cloud/jira/platform/index/#connect-apps) or [Forge](https://developer.atlassian.com/cloud/jira/platform/index/#forge-apps) app can be updated.  To assist with app migration, this operation can be used to:   *  Disable a rule.  *  Add a `tag`. Use this to filter rules in the [Get workflow transition rule configurations](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-workflow-transition-rules/#api-rest-api-3-workflow-rule-config-get).  Rules are enabled if the `disabled` parameter is not provided.  **[Permissions](#permissions) required:** Only [Connect](https://developer.atlassian.com/cloud/jira/platform/index/#connect-apps) or [Forge](https://developer.atlassian.com/cloud/jira/platform/index/#forge-apps) apps can use this operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workflow_transition_rules_update** | [**WorkflowTransitionRulesUpdate**](WorkflowTransitionRulesUpdate.md) |  | [required] |

### Return type

[**models::WorkflowTransitionRulesUpdateErrors**](WorkflowTransitionRulesUpdateErrors.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

