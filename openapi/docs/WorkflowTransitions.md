# WorkflowTransitions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**actions** | Option<[**Vec<models::WorkflowRuleConfiguration>**](WorkflowRuleConfiguration.md)> | The post-functions of the transition. | [optional]
**conditions** | Option<[**models::ConditionGroupConfiguration**](ConditionGroupConfiguration.md)> |  | [optional]
**custom_issue_event_id** | Option<**String**> | The custom event ID of the transition. | [optional]
**description** | Option<**String**> | The description of the transition. | [optional]
**from** | Option<[**Vec<models::WorkflowStatusAndPort>**](WorkflowStatusAndPort.md)> | The statuses and ports that the transition can start from. This field is deprecated - use `toStatusReference`/`links` instead. | [optional]
**id** | Option<**String**> | The ID of the transition. | [optional]
**links** | Option<[**Vec<models::WorkflowTransitionLinks>**](WorkflowTransitionLinks.md)> | The statuses the transition can start from, and the mapping of ports between the statuses. | [optional]
**name** | Option<**String**> | The name of the transition. | [optional]
**properties** | Option<**std::collections::HashMap<String, String>**> | The properties of the transition. | [optional]
**to** | Option<[**models::WorkflowStatusAndPort**](WorkflowStatusAndPort.md)> |  | [optional]
**to_status_reference** | Option<**String**> | The status the transition goes to. | [optional]
**transition_screen** | Option<[**models::WorkflowRuleConfiguration**](WorkflowRuleConfiguration.md)> |  | [optional]
**triggers** | Option<[**Vec<models::WorkflowTrigger>**](WorkflowTrigger.md)> | The triggers of the transition. | [optional]
**r#type** | Option<**String**> | The transition type. | [optional]
**validators** | Option<[**Vec<models::WorkflowRuleConfiguration>**](WorkflowRuleConfiguration.md)> | The validators of the transition. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


