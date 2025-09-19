# TransitionPreview

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**actions** | Option<[**Vec<models::PreviewRuleConfiguration>**](PreviewRuleConfiguration.md)> | The post-functions of the transition. | [optional]
**conditions** | Option<[**models::PreviewConditionGroupConfiguration**](PreviewConditionGroupConfiguration.md)> |  | [optional]
**custom_issue_event_id** | Option<**String**> | The custom issue event ID for the transition. | [optional]
**description** | Option<**String**> | The description of the transition. | [optional]
**id** | Option<**String**> | The ID of the transition. | [optional]
**links** | Option<[**Vec<models::TransitionLink>**](TransitionLink.md)> | The statuses the transition can start from, and the mapping of ports between the statuses. | [optional]
**name** | Option<**String**> | The name of the transition. | [optional]
**to_status_reference** | Option<**String**> | The status the transition goes to. | [optional]
**transition_screen** | Option<[**models::PreviewRuleConfiguration**](PreviewRuleConfiguration.md)> |  | [optional]
**triggers** | Option<[**Vec<models::PreviewTrigger>**](PreviewTrigger.md)> | The triggers of the transition. | [optional]
**r#type** | Option<**String**> | The transition type. | [optional]
**validators** | Option<[**Vec<models::PreviewRuleConfiguration>**](PreviewRuleConfiguration.md)> | The validators of the transition. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


