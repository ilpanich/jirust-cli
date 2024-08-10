# UpdatePrioritySchemeRequestBean

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**default_priority_id** | Option<**i64**> | The default priority of the scheme. | [optional]
**description** | Option<**String**> | The description of the priority scheme. | [optional]
**mappings** | Option<[**models::PriorityMapping**](PriorityMapping.md)> | Instructions to migrate issues. | [optional]
**name** | Option<**String**> | The name of the priority scheme. Must be unique. | [optional]
**priorities** | Option<[**models::UpdatePrioritiesInSchemeRequestBean**](UpdatePrioritiesInSchemeRequestBean.md)> | The priorities in the scheme. | [optional]
**projects** | Option<[**models::UpdateProjectsInSchemeRequestBean**](UpdateProjectsInSchemeRequestBean.md)> | The projects in the scheme. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


