# CreatePlanRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cross_project_releases** | Option<[**Vec<models::CreateCrossProjectReleaseRequest>**](CreateCrossProjectReleaseRequest.md)> | The cross-project releases to include in the plan. | [optional]
**custom_fields** | Option<[**Vec<models::CreateCustomFieldRequest>**](CreateCustomFieldRequest.md)> | The custom fields for the plan. | [optional]
**exclusion_rules** | Option<[**models::CreateExclusionRulesRequest**](CreateExclusionRulesRequest.md)> | The exclusion rules for the plan. | [optional]
**issue_sources** | [**Vec<models::CreateIssueSourceRequest>**](CreateIssueSourceRequest.md) | The issue sources to include in the plan. | 
**lead_account_id** | Option<**String**> | The account ID of the plan lead. | [optional]
**name** | **String** | The plan name. | 
**permissions** | Option<[**Vec<models::CreatePermissionRequest>**](CreatePermissionRequest.md)> | The permissions for the plan. | [optional]
**scheduling** | [**models::CreateSchedulingRequest**](CreateSchedulingRequest.md) | The scheduling settings for the plan. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


