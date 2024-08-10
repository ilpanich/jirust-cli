/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-48dd1e2d75e7eac65741e465279d2700f927137d
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// Icon : An icon. If no icon is defined:   *  for a status icon, no status icon displays in Jira.  *  for the remote object icon, the default link icon displays in Jira.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Icon {
    /// The URL of the tooltip, used only for a status icon. If not set, the status icon in Jira is not clickable.
    #[serde(rename = "link", skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    /// The title of the icon. This is used as follows:   *  For a status icon it is used as a tooltip on the icon. If not set, the status icon doesn't display a tooltip in Jira.  *  For the remote object icon it is used in conjunction with the application name to display a tooltip for the link's icon. The tooltip takes the format \"\\[application name\\] icon title\". Blank itemsare excluded from the tooltip title. If both items are blank, the icon tooltop displays as \"Web Link\".
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The URL of an icon that displays at 16x16 pixel in Jira.
    #[serde(rename = "url16x16", skip_serializing_if = "Option::is_none")]
    pub url16x16: Option<String>,
}

impl Icon {
    /// An icon. If no icon is defined:   *  for a status icon, no status icon displays in Jira.  *  for the remote object icon, the default link icon displays in Jira.
    pub fn new() -> Icon {
        Icon {
            link: None,
            title: None,
            url16x16: None,
        }
    }
}

