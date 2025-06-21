# BoardPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**board_filter_jql** | Option<**String**> | Takes in a JQL string to create a new filter. If no value is provided, it'll default to a JQL filter for the project creating | [optional]
**card_color_strategy** | Option<**String**> | Card color settings of the board | [optional]
**card_layout** | Option<[**models::CardLayout**](CardLayout.md)> |  | [optional]
**card_layouts** | Option<[**Vec<models::CardLayoutField>**](CardLayoutField.md)> | Card layout settings of the board | [optional]
**columns** | Option<[**Vec<models::BoardColumnPayload>**](BoardColumnPayload.md)> | The columns of the board | [optional]
**features** | Option<[**Vec<models::BoardFeaturePayload>**](BoardFeaturePayload.md)> | Feature settings for the board | [optional]
**name** | Option<**String**> | The name of the board | [optional]
**pcri** | Option<[**models::ProjectCreateResourceIdentifier**](ProjectCreateResourceIdentifier.md)> |  | [optional]
**quick_filters** | Option<[**Vec<models::QuickFilterPayload>**](QuickFilterPayload.md)> | The quick filters for the board. | [optional]
**supports_sprint** | Option<**bool**> | Whether sprints are supported on the board | [optional][default to true]
**swimlanes** | Option<[**models::SwimlanesPayload**](SwimlanesPayload.md)> |  | [optional]
**working_days_config** | Option<[**models::WorkingDaysConfig**](WorkingDaysConfig.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


