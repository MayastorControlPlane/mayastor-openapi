/*
 * Mayastor RESTful API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SpecsVolumes : User specification of a volume.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpecsVolumes {
    /// Volume labels.
    #[serde(rename = "labels")]
    pub labels: Vec<String>,
    /// Number of front-end paths.
    #[serde(rename = "num_paths")]
    pub num_paths: i32,
    /// Number of children the volume should have.
    #[serde(rename = "num_replicas")]
    pub num_replicas: i32,
    #[serde(rename = "operation", skip_serializing_if = "Option::is_none")]
    pub operation: Option<Box<crate::models::SpecsOperation3>>,
    /// Protocol that the volume should be shared over.
    #[serde(rename = "protocol")]
    pub protocol: Protocol,
    /// Size that the volume should be.
    #[serde(rename = "size")]
    pub size: i64,
    /// State that the volume should eventually achieve.
    #[serde(rename = "state")]
    pub state: State,
    /// The node where front-end IO will be sent to
    #[serde(rename = "target_node", skip_serializing_if = "Option::is_none")]
    pub target_node: Option<String>,
    /// Volume Id
    #[serde(rename = "uuid")]
    pub uuid: String,
}

impl SpecsVolumes {
    /// User specification of a volume.
    pub fn new(labels: Vec<String>, num_paths: i32, num_replicas: i32, protocol: Protocol, size: i64, state: State, uuid: String) -> SpecsVolumes {
        SpecsVolumes {
            labels,
            num_paths,
            num_replicas,
            operation: None,
            protocol,
            size,
            state,
            target_node: None,
            uuid,
        }
    }
}

/// Protocol that the volume should be shared over.
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
/// State that the volume should eventually achieve.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "Creating")]
    Creating,
    #[serde(rename = "Created")]
    Created,
    #[serde(rename = "Deleting")]
    Deleting,
    #[serde(rename = "Deleted")]
    Deleted,
}
