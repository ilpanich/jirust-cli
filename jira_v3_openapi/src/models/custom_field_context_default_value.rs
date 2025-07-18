/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-4f9780d932de14e7c3dec0c9382f8855050fda22
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CustomFieldContextDefaultValue {
    #[serde(rename="option.cascading")]
    OptionPeriodCascading(Box<models::CustomFieldContextDefaultValueCascadingOption>),
    #[serde(rename="option.multiple")]
    OptionPeriodMultiple(Box<models::CustomFieldContextDefaultValueMultipleOption>),
    #[serde(rename="option.single")]
    OptionPeriodSingle(Box<models::CustomFieldContextDefaultValueSingleOption>),
    #[serde(rename="single.user.select")]
    SinglePeriodUserPeriodSelect(Box<models::CustomFieldContextSingleUserPickerDefaults>),
    #[serde(rename="multi.user.select")]
    MultiPeriodUserPeriodSelect(Box<models::CustomFieldContextDefaultValueMultiUserPicker>),
    #[serde(rename="grouppicker.single")]
    GrouppickerPeriodSingle(Box<models::CustomFieldContextDefaultValueSingleGroupPicker>),
    #[serde(rename="grouppicker.multiple")]
    GrouppickerPeriodMultiple(Box<models::CustomFieldContextDefaultValueMultipleGroupPicker>),
    #[serde(rename="datepicker")]
    Datepicker(Box<models::CustomFieldContextDefaultValueDate>),
    #[serde(rename="datetimepicker")]
    Datetimepicker(Box<models::CustomFieldContextDefaultValueDateTime>),
    #[serde(rename="url")]
    Url(Box<models::CustomFieldContextDefaultValueUrl>),
    #[serde(rename="project")]
    Project(Box<models::CustomFieldContextDefaultValueProject>),
    #[serde(rename="float")]
    Float(Box<models::CustomFieldContextDefaultValueFloat>),
    #[serde(rename="labels")]
    Labels(Box<models::CustomFieldContextDefaultValueLabels>),
    #[serde(rename="textfield")]
    Textfield(Box<models::CustomFieldContextDefaultValueTextField>),
    #[serde(rename="textarea")]
    Textarea(Box<models::CustomFieldContextDefaultValueTextArea>),
    #[serde(rename="readonly")]
    Readonly(Box<models::CustomFieldContextDefaultValueReadOnly>),
    #[serde(rename="version.single")]
    VersionPeriodSingle(Box<models::CustomFieldContextDefaultValueSingleVersionPicker>),
    #[serde(rename="version.multiple")]
    VersionPeriodMultiple(Box<models::CustomFieldContextDefaultValueMultipleVersionPicker>),
    #[serde(rename="forge.string")]
    ForgePeriodString(Box<models::CustomFieldContextDefaultValueForgeStringField>),
    #[serde(rename="forge.string.list")]
    ForgePeriodStringPeriodList(Box<models::CustomFieldContextDefaultValueForgeMultiStringField>),
    #[serde(rename="forge.object")]
    ForgePeriodObject(Box<models::CustomFieldContextDefaultValueForgeObjectField>),
    #[serde(rename="forge.datetime")]
    ForgePeriodDatetime(Box<models::CustomFieldContextDefaultValueForgeDateTimeField>),
    #[serde(rename="forge.group")]
    ForgePeriodGroup(Box<models::CustomFieldContextDefaultValueForgeGroupField>),
    #[serde(rename="forge.group.list")]
    ForgePeriodGroupPeriodList(Box<models::CustomFieldContextDefaultValueForgeMultiGroupField>),
    #[serde(rename="forge.number")]
    ForgePeriodNumber(Box<models::CustomFieldContextDefaultValueForgeNumberField>),
    #[serde(rename="forge.user")]
    ForgePeriodUser(Box<models::CustomFieldContextDefaultValueForgeUserField>),
    #[serde(rename="forge.user.list")]
    ForgePeriodUserPeriodList(Box<models::CustomFieldContextDefaultValueForgeMultiUserField>),
}

impl Default for CustomFieldContextDefaultValue {
    fn default() -> Self {
        Self::OptionPeriodCascading(Default::default())
    }
}


