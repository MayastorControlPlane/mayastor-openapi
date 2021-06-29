#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use actix_web::web::{self, Json, Path, Query};

#[async_trait::async_trait]
pub trait VolumesApi {
    async fn del_shares(web::Path(volume_id): web::Path<String>) -> Result<web::Json<()>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn del_volume(web::Path(volume_id): web::Path<String>) -> Result<web::Json<()>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn get_node_volume(web::Path((node_id, volume_id)): web::Path<(String, String)>) -> Result<web::Json<crate::models::Volume>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn get_node_volumes(web::Path(node_id): web::Path<String>) -> Result<web::Json<Vec<crate::models::Volume>>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn get_volume(web::Path(volume_id): web::Path<String>) -> Result<web::Json<crate::models::Volume>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn get_volumes() -> Result<web::Json<Vec<crate::models::Volume>>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn put_volume(web::Path(volume_id): web::Path<String>, web::Json(create_volume_body): web::Json<crate::models::CreateVolumeBody>) -> Result<web::Json<crate::models::Volume>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn put_volume_share(web::Path((volume_id, protocol)): web::Path<(String, String)>) -> Result<web::Json<String>, crate::apis::RestError<crate::models::RestJsonError>>;
}
