/*
 * Mayastor RESTful API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Nexus : Nexus information



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Nexus {
    /// array of children
    #[serde(rename = "children")]
    pub children: Vec<crate::models::NexusChildren>,
    /// URI of the device for the volume (missing if not published).  Missing property and empty string are treated the same.
    #[serde(rename = "deviceUri")]
    pub device_uri: String,
    /// id of the mayastor instance
    #[serde(rename = "node")]
    pub node: String,
    /// total number of rebuild tasks
    #[serde(rename = "rebuilds")]
    pub rebuilds: i32,
    /// protocol used for exposing the nexus
    #[serde(rename = "share")]
    pub share: Share,
    /// size of the volume in bytes
    #[serde(rename = "size")]
    pub size: i64,
    /// current state of the nexus
    #[serde(rename = "state")]
    pub state: State,
    /// uuid of the nexus
    #[serde(rename = "uuid")]
    pub uuid: String,
}

impl Nexus {
    /// Nexus information
    pub fn new(children: Vec<crate::models::NexusChildren>, device_uri: String, node: String, rebuilds: i32, share: Share, size: i64, state: State, uuid: String) -> Nexus {
        Nexus {
            children,
            device_uri,
            node,
            rebuilds,
            share,
            size,
            state,
            uuid,
        }
    }
}

/// protocol used for exposing the nexus
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Share {
    #[serde(rename = "false")]
    _False,
    #[serde(rename = "nvmf")]
    Nvmf,
    #[serde(rename = "iscsi")]
    Iscsi,
    #[serde(rename = "nbd")]
    Nbd,
}
/// current state of the nexus
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "Online")]
    Online,
    #[serde(rename = "Degraded")]
    Degraded,
    #[serde(rename = "Faulted")]
    Faulted,
}

