#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use actix_web::{
    web::{self, Json, Path, Query},
    FromRequest,
};

/// Configure handlers for the JsonGrpcApi resource
pub fn configure<T: crate::apis::JsonGrpcApi + 'static, A: FromRequest + 'static>(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
       .service(
            actix_web::web::resource("/nodes/{node}/jsongrpc/{method}")
                .name("put_node_jsongrpc")
                .guard(actix_web::guard::Put())
                .route(actix_web::web::put().to(put_node_jsongrpc::<T, A>))
       );
}

async fn put_node_jsongrpc<T: crate::apis::JsonGrpcApi + 'static, A: FromRequest + 'static>(_token: A, web::Path((node, method)): web::Path<(String, String)>, web::Json(json_generic): web::Json<crate::models::JsonGeneric>) -> Result<web::Json<crate::models::JsonGeneric>, crate::apis::RestError<crate::models::RestJsonError>> {
    T::put_node_jsongrpc(web::Path((node, method)), web::Json(json_generic)).await
}
