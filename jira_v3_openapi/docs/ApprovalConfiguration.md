# ApprovalConfiguration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active** | **String** | Whether the approval configuration is active. | 
**condition_type** | **String** | How the required approval count is calculated. It may be configured to require a specific number of approvals, or approval by a percentage of approvers. If the approvers source field is Approver groups, you can configure how many approvals per group are required for the request to be approved. The number will be the same across all groups. | 
**condition_value** | **String** | The number or percentage of approvals required for a request to be approved. If `conditionType` is `number`, the value must be 20 or less. If `conditionType` is `percent`, the value must be 100 or less. | 
**exclude** | Option<**Vec<String>**> | A list of roles that should be excluded as possible approvers. | [optional]
**field_id** | **String** | The custom field ID of the \"Approvers\" or \"Approver Groups\" field. | 
**pre_populated_field_id** | Option<**String**> | The custom field ID of the field used to pre-populate the Approver field. Only supports the \"Affected Services\" field. | [optional]
**transition_approved** | **String** | The numeric ID of the transition to be executed if the request is approved. | 
**transition_rejected** | **String** | The numeric ID of the transition to be executed if the request is declined. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


