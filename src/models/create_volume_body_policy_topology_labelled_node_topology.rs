/*
 * Mayastor RESTful API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateVolumeBodyPolicyTopologyLabelledNodeTopology : node topology



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateVolumeBodyPolicyTopologyLabelledNodeTopology {
    /// exclusive labels
    #[serde(rename = "exclusion")]
    pub exclusion: Vec<String>,
    /// inclusive labels
    #[serde(rename = "inclusion")]
    pub inclusion: Vec<String>,
}

impl CreateVolumeBodyPolicyTopologyLabelledNodeTopology {
    /// node topology
    pub fn new(exclusion: Vec<String>, inclusion: Vec<String>) -> CreateVolumeBodyPolicyTopologyLabelledNodeTopology {
        CreateVolumeBodyPolicyTopologyLabelledNodeTopology {
            exclusion,
            inclusion,
        }
    }
}


