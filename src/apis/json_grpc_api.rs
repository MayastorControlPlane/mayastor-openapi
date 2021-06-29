#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use actix_web::web::{self, Json, Path, Query};

#[async_trait::async_trait]
pub trait JsonGrpcApi {
    async fn put_node_jsongrpc(web::Path((node, method)): web::Path<(String, String)>, web::Json(json_generic): web::Json<crate::models::JsonGeneric>) -> Result<web::Json<crate::models::JsonGeneric>, crate::apis::RestError<crate::models::RestJsonError>>;
}
