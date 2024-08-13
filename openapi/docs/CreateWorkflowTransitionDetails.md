# CreateWorkflowTransitionDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | The description of the transition. The maximum length is 1000 characters. | [optional]
**from** | Option<**Vec<String>**> | The statuses the transition can start from. | [optional]
**name** | **String** | The name of the transition. The maximum length is 60 characters. | 
**properties** | Option<**std::collections::HashMap<String, String>**> | The properties of the transition. | [optional]
**rules** | Option<[**models::CreateWorkflowTransitionRulesDetails**](CreateWorkflowTransitionRulesDetails.md)> | The rules of the transition. | [optional]
**screen** | Option<[**models::CreateWorkflowTransitionScreenDetails**](CreateWorkflowTransitionScreenDetails.md)> | The screen of the transition. | [optional]
**to** | **String** | The status the transition goes to. | 
**r#type** | **String** | The type of the transition. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


