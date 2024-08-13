# Worklog

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**author** | Option<[**models::UserDetails**](UserDetails.md)> | Details of the user who created the worklog. | [optional][readonly]
**comment** | Option<[**serde_json::Value**](.md)> | A comment about the worklog in [Atlassian Document Format](https://developer.atlassian.com/cloud/jira/platform/apis/document/structure/). Optional when creating or updating a worklog. | [optional]
**created** | Option<**String**> | The datetime on which the worklog was created. | [optional][readonly]
**id** | Option<**String**> | The ID of the worklog record. | [optional][readonly]
**issue_id** | Option<**String**> | The ID of the issue this worklog is for. | [optional][readonly]
**properties** | Option<[**Vec<models::EntityProperty>**](EntityProperty.md)> | Details of properties for the worklog. Optional when creating or updating a worklog. | [optional]
**param_self** | Option<**String**> | The URL of the worklog item. | [optional][readonly]
**started** | Option<**String**> | The datetime on which the worklog effort was started. Required when creating a worklog. Optional when updating a worklog. | [optional]
**time_spent** | Option<**String**> | The time spent working on the issue as days (\\#d), hours (\\#h), or minutes (\\#m or \\#). Required when creating a worklog if `timeSpentSeconds` isn't provided. Optional when updating a worklog. Cannot be provided if `timeSpentSecond` is provided. | [optional]
**time_spent_seconds** | Option<**i64**> | The time in seconds spent working on the issue. Required when creating a worklog if `timeSpent` isn't provided. Optional when updating a worklog. Cannot be provided if `timeSpent` is provided. | [optional]
**update_author** | Option<[**models::UserDetails**](UserDetails.md)> | Details of the user who last updated the worklog. | [optional][readonly]
**updated** | Option<**String**> | The datetime on which the worklog was last updated. | [optional][readonly]
**visibility** | Option<[**models::Visibility**](Visibility.md)> | Details about any restrictions in the visibility of the worklog. Optional when creating or updating a worklog. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


