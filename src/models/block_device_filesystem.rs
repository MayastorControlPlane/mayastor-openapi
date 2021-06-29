/*
 * Mayastor RESTful API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BlockDeviceFilesystem : filesystem information in case where a filesystem is present



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockDeviceFilesystem {
    /// filesystem type: ext3, ntfs, ...
    #[serde(rename = "fstype")]
    pub fstype: String,
    /// volume label
    #[serde(rename = "label")]
    pub label: String,
    /// path where filesystem is currently mounted
    #[serde(rename = "mountpoint")]
    pub mountpoint: String,
    /// UUID identifying the volume (filesystem)
    #[serde(rename = "uuid")]
    pub uuid: String,
}

impl BlockDeviceFilesystem {
    /// filesystem information in case where a filesystem is present
    pub fn new(fstype: String, label: String, mountpoint: String, uuid: String) -> BlockDeviceFilesystem {
        BlockDeviceFilesystem {
            fstype,
            label,
            mountpoint,
            uuid,
        }
    }
}


