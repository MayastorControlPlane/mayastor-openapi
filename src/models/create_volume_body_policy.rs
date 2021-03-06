/*
 * Mayastor RESTful API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateVolumeBodyPolicy : Volume Healing policy used to determine if and how to replace a replica



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateVolumeBodyPolicy {
    /// the server will attempt to heal the volume by itself  the client should not attempt to do the same if this is enabled
    #[serde(rename = "self_heal")]
    pub self_heal: bool,
    #[serde(rename = "topology", skip_serializing_if = "Option::is_none")]
    pub topology: Option<Box<crate::models::CreateVolumeBodyPolicyTopology>>,
}

impl CreateVolumeBodyPolicy {
    /// Volume Healing policy used to determine if and how to replace a replica
    pub fn new(self_heal: bool) -> CreateVolumeBodyPolicy {
        CreateVolumeBodyPolicy {
            self_heal,
            topology: None,
        }
    }
}


