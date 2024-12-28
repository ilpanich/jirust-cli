# GetPlanResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cross_project_releases** | Option<[**Vec<models::GetCrossProjectReleaseResponse>**](GetCrossProjectReleaseResponse.md)> | The cross-project releases included in the plan. | [optional]
**custom_fields** | Option<[**Vec<models::GetCustomFieldResponse>**](GetCustomFieldResponse.md)> | The custom fields for the plan. | [optional]
**exclusion_rules** | Option<[**models::GetExclusionRulesResponse**](GetExclusionRulesResponse.md)> | The exclusion rules for the plan. | [optional]
**id** | **i64** | The plan ID. | 
**issue_sources** | Option<[**Vec<models::GetIssueSourceResponse>**](GetIssueSourceResponse.md)> | The issue sources included in the plan. | [optional]
**last_saved** | Option<**String**> | The date when the plan was last saved in UTC. | [optional]
**lead_account_id** | Option<**String**> | The account ID of the plan lead. | [optional]
**name** | Option<**String**> | The plan name. | [optional]
**permissions** | Option<[**Vec<models::GetPermissionResponse>**](GetPermissionResponse.md)> | The permissions for the plan. | [optional]
**scheduling** | [**models::GetSchedulingResponse**](GetSchedulingResponse.md) | The scheduling settings for the plan. | 
**status** | **String** | The plan status. This is \"Active\", \"Trashed\" or \"Archived\". | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


