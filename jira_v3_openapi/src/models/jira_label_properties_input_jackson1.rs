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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JiraLabelPropertiesInputJackson1 {
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl JiraLabelPropertiesInputJackson1 {
    pub fn new() -> JiraLabelPropertiesInputJackson1 {
        JiraLabelPropertiesInputJackson1 {
            color: None,
            name: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Color {
    #[serde(rename = "GREY_LIGHTEST")]
    GreyLightest,
    #[serde(rename = "GREY_LIGHTER")]
    GreyLighter,
    #[serde(rename = "GREY")]
    Grey,
    #[serde(rename = "GREY_DARKER")]
    GreyDarker,
    #[serde(rename = "GREY_DARKEST")]
    GreyDarkest,
    #[serde(rename = "PURPLE_LIGHTEST")]
    PurpleLightest,
    #[serde(rename = "PURPLE_LIGHTER")]
    PurpleLighter,
    #[serde(rename = "PURPLE")]
    Purple,
    #[serde(rename = "PURPLE_DARKER")]
    PurpleDarker,
    #[serde(rename = "PURPLE_DARKEST")]
    PurpleDarkest,
    #[serde(rename = "BLUE_LIGHTEST")]
    BlueLightest,
    #[serde(rename = "BLUE_LIGHTER")]
    BlueLighter,
    #[serde(rename = "BLUE")]
    Blue,
    #[serde(rename = "BLUE_DARKER")]
    BlueDarker,
    #[serde(rename = "BLUE_DARKEST")]
    BlueDarkest,
    #[serde(rename = "TEAL_LIGHTEST")]
    TealLightest,
    #[serde(rename = "TEAL_LIGHTER")]
    TealLighter,
    #[serde(rename = "TEAL")]
    Teal,
    #[serde(rename = "TEAL_DARKER")]
    TealDarker,
    #[serde(rename = "TEAL_DARKEST")]
    TealDarkest,
    #[serde(rename = "GREEN_LIGHTEST")]
    GreenLightest,
    #[serde(rename = "GREEN_LIGHTER")]
    GreenLighter,
    #[serde(rename = "GREEN")]
    Green,
    #[serde(rename = "GREEN_DARKER")]
    GreenDarker,
    #[serde(rename = "GREEN_DARKEST")]
    GreenDarkest,
    #[serde(rename = "LIME_LIGHTEST")]
    LimeLightest,
    #[serde(rename = "LIME_LIGHTER")]
    LimeLighter,
    #[serde(rename = "LIME")]
    Lime,
    #[serde(rename = "LIME_DARKER")]
    LimeDarker,
    #[serde(rename = "LIME_DARKEST")]
    LimeDarkest,
    #[serde(rename = "YELLOW_LIGHTEST")]
    YellowLightest,
    #[serde(rename = "YELLOW_LIGHTER")]
    YellowLighter,
    #[serde(rename = "YELLOW")]
    Yellow,
    #[serde(rename = "YELLOW_DARKER")]
    YellowDarker,
    #[serde(rename = "YELLOW_DARKEST")]
    YellowDarkest,
    #[serde(rename = "ORANGE_LIGHTEST")]
    OrangeLightest,
    #[serde(rename = "ORANGE_LIGHTER")]
    OrangeLighter,
    #[serde(rename = "ORANGE")]
    Orange,
    #[serde(rename = "ORANGE_DARKER")]
    OrangeDarker,
    #[serde(rename = "ORANGE_DARKEST")]
    OrangeDarkest,
    #[serde(rename = "RED_LIGHTEST")]
    RedLightest,
    #[serde(rename = "RED_LIGHTER")]
    RedLighter,
    #[serde(rename = "RED")]
    Red,
    #[serde(rename = "RED_DARKER")]
    RedDarker,
    #[serde(rename = "RED_DARKEST")]
    RedDarkest,
    #[serde(rename = "MAGENTA_LIGHTEST")]
    MagentaLightest,
    #[serde(rename = "MAGENTA_LIGHTER")]
    MagentaLighter,
    #[serde(rename = "MAGENTA")]
    Magenta,
    #[serde(rename = "MAGENTA_DARKER")]
    MagentaDarker,
    #[serde(rename = "MAGENTA_DARKEST")]
    MagentaDarkest,
}

impl Default for Color {
    fn default() -> Color {
        Self::GreyLightest
    }
}

