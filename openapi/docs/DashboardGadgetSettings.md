# DashboardGadgetSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**color** | Option<**String**> | The color of the gadget. Should be one of `blue`, `red`, `yellow`, `green`, `cyan`, `purple`, `gray`, or `white`. | [optional]
**ignore_uri_and_module_key_validation** | Option<**bool**> | Whether to ignore the validation of module key and URI. For example, when a gadget is created that is a part of an application that isn't installed. | [optional]
**module_key** | Option<**String**> | The module key of the gadget type. Can't be provided with `uri`. | [optional]
**position** | Option<[**models::DashboardGadgetPosition**](DashboardGadgetPosition.md)> | The position of the gadget. When the gadget is placed into the position, other gadgets in the same column are moved down to accommodate it. | [optional]
**title** | Option<**String**> | The title of the gadget. | [optional]
**uri** | Option<**String**> | The URI of the gadget type. Can't be provided with `moduleKey`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


