# Transition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | **String** | The description of the transition. | 
**from** | **Vec<String>** | The statuses the transition can start from. | 
**id** | **String** | The ID of the transition. | 
**name** | **String** | The name of the transition. | 
**properties** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | The properties of the transition. | [optional]
**rules** | Option<[**models::WorkflowRules**](WorkflowRules.md)> |  | [optional]
**screen** | Option<[**models::TransitionScreenDetails**](TransitionScreenDetails.md)> |  | [optional]
**to** | **String** | The status the transition goes to. | 
**r#type** | **String** | The type of the transition. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


