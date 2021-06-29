/*
 * Mayastor RESTful API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Volume : Volumes   Volume information



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Volume {
    /// array of children nexuses
    #[serde(rename = "children")]
    pub children: Vec<crate::models::VolumeChildren>,
    /// current share protocol
    #[serde(rename = "protocol")]
    pub protocol: Protocol,
    /// size of the volume in bytes
    #[serde(rename = "size")]
    pub size: i64,
    /// current state of the volume
    #[serde(rename = "state")]
    pub state: State,
    /// name of the volume
    #[serde(rename = "uuid")]
    pub uuid: String,
}

impl Volume {
    /// Volumes   Volume information
    pub fn new(children: Vec<crate::models::VolumeChildren>, protocol: Protocol, size: i64, state: State, uuid: String) -> Volume {
        Volume {
            children,
            protocol,
            size,
            state,
            uuid,
        }
    }
}

/// current share protocol
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Protocol {
    #[serde(rename = "false")]
    _False,
    #[serde(rename = "nvmf")]
    Nvmf,
    #[serde(rename = "iscsi")]
    Iscsi,
    #[serde(rename = "nbd")]
    Nbd,
}
/// current state of the volume
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
