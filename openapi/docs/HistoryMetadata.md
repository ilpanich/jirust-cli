# HistoryMetadata

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**activity_description** | Option<**String**> | The activity described in the history record. | [optional]
**activity_description_key** | Option<**String**> | The key of the activity described in the history record. | [optional]
**actor** | Option<[**models::HistoryMetadataParticipant**](HistoryMetadataParticipant.md)> | Details of the user whose action created the history record. | [optional]
**cause** | Option<[**models::HistoryMetadataParticipant**](HistoryMetadataParticipant.md)> | Details of the cause that triggered the creation the history record. | [optional]
**description** | Option<**String**> | The description of the history record. | [optional]
**description_key** | Option<**String**> | The description key of the history record. | [optional]
**email_description** | Option<**String**> | The description of the email address associated the history record. | [optional]
**email_description_key** | Option<**String**> | The description key of the email address associated the history record. | [optional]
**extra_data** | Option<**std::collections::HashMap<String, String>**> | Additional arbitrary information about the history record. | [optional]
**generator** | Option<[**models::HistoryMetadataParticipant**](HistoryMetadataParticipant.md)> | Details of the system that generated the history record. | [optional]
**r#type** | Option<**String**> | The type of the history record. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


