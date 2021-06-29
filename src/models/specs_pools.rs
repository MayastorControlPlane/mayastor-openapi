/*
 * Mayastor RESTful API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SpecsPools : User specification of a pool.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpecsPools {
    /// absolute disk paths claimed by the pool
    #[serde(rename = "disks")]
    pub disks: Vec<String>,
    /// id of the pool
    #[serde(rename = "id")]
    pub id: String,
    /// Pool labels.
    #[serde(rename = "labels")]
    pub labels: Vec<String>,
    /// id of the mayastor instance
    #[serde(rename = "node")]
    pub node: String,
    #[serde(rename = "operation", skip_serializing_if = "Option::is_none")]
    pub operation: Option<Box<crate::models::SpecsOperation1>>,
    /// state of the pool
    #[serde(rename = "state")]
    pub state: State,
}

impl SpecsPools {
    /// User specification of a pool.
    pub fn new(disks: Vec<String>, id: String, labels: Vec<String>, node: String, state: State) -> SpecsPools {
        SpecsPools {
            disks,
            id,
            labels,
            node,
            operation: None,
            state,
        }
    }
}

/// state of the pool
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

