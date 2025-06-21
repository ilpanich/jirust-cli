# RedactionPosition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**adf_pointer** | Option<**String**> | The ADF pointer indicating the position of the text to be redacted. This is only required when redacting from rich text(ADF) fields. For plain text fields, this field can be omitted. | [optional]
**expected_text** | **String** | The text which will be redacted, encoded using SHA256 hash and Base64 digest | 
**from** | **i32** | The start index(inclusive) for the redaction in specified content | 
**to** | **i32** | The ending index(exclusive) for the redaction in specified content | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


