/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-1461af1efd5cc75bf21cb8256a8b42f9bd4278be
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// UpdatePriorityDetails : Details of an issue priority.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdatePriorityDetails {
    /// The ID for the avatar for the priority. This parameter is nullable and both iconUrl and avatarId cannot be defined.
    #[serde(rename = "avatarId", skip_serializing_if = "Option::is_none")]
    pub avatar_id: Option<i64>,
    /// The description of the priority.
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// The URL of an icon for the priority. Accepted protocols are HTTP and HTTPS. Built in icons can also be used. Both iconUrl and avatarId cannot be defined.
    #[serde(rename = "iconUrl", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<Option<IconUrl>>,
    /// The name of the priority. Must be unique.
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    /// The status color of the priority in 3-digit or 6-digit hexadecimal format.
    #[serde(rename = "statusColor", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub status_color: Option<Option<String>>,
}

impl UpdatePriorityDetails {
    /// Details of an issue priority.
    pub fn new() -> UpdatePriorityDetails {
        UpdatePriorityDetails {
            avatar_id: None,
            description: None,
            icon_url: None,
            name: None,
            status_color: None,
        }
    }
}
/// The URL of an icon for the priority. Accepted protocols are HTTP and HTTPS. Built in icons can also be used. Both iconUrl and avatarId cannot be defined.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IconUrl {
    #[serde(rename = "/images/icons/priorities/blocker.png")]
    BlockerPeriodPng,
    #[serde(rename = "/images/icons/priorities/critical.png")]
    CriticalPeriodPng,
    #[serde(rename = "/images/icons/priorities/high.png")]
    HighPeriodPng,
    #[serde(rename = "/images/icons/priorities/highest.png")]
    HighestPeriodPng,
    #[serde(rename = "/images/icons/priorities/low.png")]
    LowPeriodPng,
    #[serde(rename = "/images/icons/priorities/lowest.png")]
    LowestPeriodPng,
    #[serde(rename = "/images/icons/priorities/major.png")]
    MajorPeriodPng,
    #[serde(rename = "/images/icons/priorities/medium.png")]
    MediumPeriodPng,
    #[serde(rename = "/images/icons/priorities/minor.png")]
    MinorPeriodPng,
    #[serde(rename = "/images/icons/priorities/trivial.png")]
    TrivialPeriodPng,
    #[serde(rename = "/images/icons/priorities/blocker_new.png")]
    BlockerNewPeriodPng,
    #[serde(rename = "/images/icons/priorities/critical_new.png")]
    CriticalNewPeriodPng,
    #[serde(rename = "/images/icons/priorities/high_new.png")]
    HighNewPeriodPng,
    #[serde(rename = "/images/icons/priorities/highest_new.png")]
    HighestNewPeriodPng,
    #[serde(rename = "/images/icons/priorities/low_new.png")]
    LowNewPeriodPng,
    #[serde(rename = "/images/icons/priorities/lowest_new.png")]
    LowestNewPeriodPng,
    #[serde(rename = "/images/icons/priorities/major_new.png")]
    MajorNewPeriodPng,
    #[serde(rename = "/images/icons/priorities/medium_new.png")]
    MediumNewPeriodPng,
    #[serde(rename = "/images/icons/priorities/minor_new.png")]
    MinorNewPeriodPng,
    #[serde(rename = "/images/icons/priorities/trivial_new.png")]
    TrivialNewPeriodPng,
}

impl Default for IconUrl {
    fn default() -> IconUrl {
        Self::BlockerPeriodPng
    }
}

