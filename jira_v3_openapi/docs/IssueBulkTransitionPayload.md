# IssueBulkTransitionPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bulk_transition_inputs** | [**Vec<models::BulkTransitionSubmitInput>**](BulkTransitionSubmitInput.md) | List of objects and each object has two properties:   *  Issues that will be bulk transitioned.  *  TransitionId that corresponds to a specific transition of issues that share the same workflow. | 
**send_bulk_notification** | Option<**bool**> | A boolean value that indicates whether to send a bulk change notification when the issues are being transitioned.  If `true`, dispatches a bulk notification email to users about the updates. | [optional][default to true]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


