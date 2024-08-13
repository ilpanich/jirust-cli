# BulkOperationProgress

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created** | Option<**String**> | A timestamp of when the task was submitted. | [optional]
**failed_accessible_issues** | Option<[**std::collections::HashMap<String, Vec<String>>**](Vec.md)> | Map of issue IDs for which the operation failed and that the user has permission to view, to their one or more reasons for failure. These reasons are open-ended text descriptions of the error and are not selected from a predefined list of standard reasons. | [optional]
**invalid_or_inaccessible_issue_count** | Option<**i32**> | The number of issues that are either invalid or issues that the user doesn't have permission to view, regardless of the success or failure of the operation. | [optional]
**processed_accessible_issues** | Option<**Vec<i64>**> | List of issue IDs for which the operation was successful and that the user has permission to view. | [optional]
**progress_percent** | Option<**i64**> | Progress of the task as a percentage. | [optional]
**started** | Option<**String**> | A timestamp of when the task was started. | [optional]
**status** | Option<**String**> | The status of the task. | [optional]
**submitted_by** | Option<[**models::User**](User.md)> |  | [optional]
**task_id** | Option<**String**> | The ID of the task. | [optional][readonly]
**total_issue_count** | Option<**i32**> | The number of issues that the bulk operation was attempted on. | [optional]
**updated** | Option<**String**> | A timestamp of when the task progress was last updated. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


