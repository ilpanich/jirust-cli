# CreateSchedulingRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dependencies** | Option<**String**> | The dependencies for the plan. This must be \"Sequential\" or \"Concurrent\". | [optional]
**end_date** | Option<[**models::CreateDateFieldRequest**](CreateDateFieldRequest.md)> | The end date field for the plan. | [optional]
**estimation** | **String** | The estimation unit for the plan. This must be \"StoryPoints\", \"Days\" or \"Hours\". | 
**inferred_dates** | Option<**String**> | The inferred dates for the plan. This must be \"None\", \"SprintDates\" or \"ReleaseDates\". | [optional]
**start_date** | Option<[**models::CreateDateFieldRequest**](CreateDateFieldRequest.md)> | The start date field for the plan. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


