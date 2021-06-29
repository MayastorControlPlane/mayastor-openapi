#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use actix_web::{
    web::{self, Json, Path, Query},
    FromRequest,
};

/// Configure handlers for the BlockDevicesApi resource
pub fn configure<T: crate::apis::BlockDevicesApi + 'static, A: FromRequest + 'static>(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
       .service(
            actix_web::web::resource("/nodes/{node}/block_devices")
                .name("get_node_block_devices")
                .guard(actix_web::guard::Get())
                .route(actix_web::web::get().to(get_node_block_devices::<T, A>))
       );
}

async fn get_node_block_devices<T: crate::apis::BlockDevicesApi + 'static, A: FromRequest + 'static>(_token: A, web::Path(node): web::Path<String>, web::Query(all): web::Query<bool>) -> Result<web::Json<Vec<crate::models::BlockDevice>>, crate::apis::RestError<crate::models::RestJsonError>> {
    T::get_node_block_devices(web::Path(node), web::Query(all)).await
}
