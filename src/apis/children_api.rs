#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use actix_web::web::{self, Json, Path, Query};

#[async_trait::async_trait]
pub trait ChildrenApi {
    async fn del_nexus_child(web::Path((nexus_id, child_id_)): web::Path<(String, String)>) -> Result<web::Json<()>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn del_node_nexus_child(web::Path((node_id, nexus_id, child_id_)): web::Path<(String, String, String)>) -> Result<web::Json<()>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn get_nexus_child(web::Path((nexus_id, child_id_)): web::Path<(String, String)>) -> Result<web::Json<crate::models::Child>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn get_nexus_children(web::Path(nexus_id): web::Path<String>) -> Result<web::Json<Vec<crate::models::Child>>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn get_node_nexus_child(web::Path((node_id, nexus_id, child_id_)): web::Path<(String, String, String)>) -> Result<web::Json<crate::models::Child>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn get_node_nexus_children(web::Path((node_id, nexus_id)): web::Path<(String, String)>) -> Result<web::Json<Vec<crate::models::Child>>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn put_nexus_child(web::Path((nexus_id, child_id_)): web::Path<(String, String)>) -> Result<web::Json<crate::models::Child>, crate::apis::RestError<crate::models::RestJsonError>>;
    async fn put_node_nexus_child(web::Path((node_id, nexus_id, child_id_)): web::Path<(String, String, String)>) -> Result<web::Json<crate::models::Child>, crate::apis::RestError<crate::models::RestJsonError>>;
}
