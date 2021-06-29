/*
 * Mayastor RESTful API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateVolumeBodyTopology : Volume topology used to determine how to place/distribute the data  Should either be labelled or explicit, not both.  If neither is used then the control plane will select from all available resources.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateVolumeBodyTopology {
    #[serde(rename = "explicit", skip_serializing_if = "Option::is_none")]
    pub explicit: Option<Box<crate::models::CreateVolumeBodyPolicyTopologyExplicit>>,
    #[serde(rename = "labelled", skip_serializing_if = "Option::is_none")]
    pub labelled: Option<Box<crate::models::CreateVolumeBodyPolicyTopologyLabelled>>,
}

impl CreateVolumeBodyTopology {
    /// Volume topology used to determine how to place/distribute the data  Should either be labelled or explicit, not both.  If neither is used then the control plane will select from all available resources.
    pub fn new() -> CreateVolumeBodyTopology {
        CreateVolumeBodyTopology {
            explicit: None,
            labelled: None,
        }
    }
}


