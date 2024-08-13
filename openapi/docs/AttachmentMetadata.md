# AttachmentMetadata

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**author** | Option<[**models::User**](User.md)> | Details of the user who attached the file. | [optional][readonly]
**content** | Option<**String**> | The URL of the attachment. | [optional][readonly]
**created** | Option<**String**> | The datetime the attachment was created. | [optional][readonly]
**filename** | Option<**String**> | The name of the attachment file. | [optional][readonly]
**id** | Option<**i64**> | The ID of the attachment. | [optional][readonly]
**mime_type** | Option<**String**> | The MIME type of the attachment. | [optional][readonly]
**properties** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Additional properties of the attachment. | [optional][readonly]
**param_self** | Option<**String**> | The URL of the attachment metadata details. | [optional][readonly]
**size** | Option<**i64**> | The size of the attachment. | [optional][readonly]
**thumbnail** | Option<**String**> | The URL of a thumbnail representing the attachment. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


