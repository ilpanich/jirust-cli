# WorkflowUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**default_status_mappings** | Option<[**Vec<models::StatusMigration>**](StatusMigration.md)> | The mapping of old to new status ID. | [optional]
**description** | Option<**String**> | The new description for this workflow. | [optional]
**id** | **String** | The ID of this workflow. | 
**start_point_layout** | Option<[**models::WorkflowLayout**](WorkflowLayout.md)> |  | [optional]
**status_mappings** | Option<[**Vec<models::StatusMappingDto>**](StatusMappingDTO.md)> | The mapping of old to new status ID for a specific project and issue type. | [optional]
**statuses** | [**Vec<models::StatusLayoutUpdate>**](StatusLayoutUpdate.md) | The statuses associated with this workflow. | 
**transitions** | [**Vec<models::TransitionUpdateDto>**](TransitionUpdateDTO.md) | The transitions of this workflow. | 
**version** | [**models::DocumentVersion**](DocumentVersion.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


