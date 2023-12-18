use crate::database::devices::{Devices, MongoDevices};
use crate::error;

use actix_web::{web, HttpResponse};

use error::CustomResult;
use std::sync::Arc;

#[actix_web::post("/houses/{house_name}/rooms/{room_name}/devices")]
async fn add_device(
    path: web::Path<(String, String)>,
    data: web::Json<Devices>,
    devices: web::Data<Arc<MongoDevices>>,
) -> CustomResult<HttpResponse> {
    let (house_name, room_name) = path.into_inner();
    let data = data.into_inner();
    let created = devices.add_device(&house_name, &room_name, data).await?;
    Ok(HttpResponse::Ok().json(created))
}

#[actix_web::delete("/houses/{house_name}/rooms/{room_name}/devices/{device_name}")]
async fn delete_device(
    path: web::Path<(String, String, String)>,
    rooms: web::Data<Arc<MongoDevices>>,
) -> CustomResult<HttpResponse> {
    let (house_name, room_name, device_name) = path.into_inner();
    let created = rooms
        .delete_device(&house_name, &room_name, &device_name)
        .await?;
    Ok(HttpResponse::Ok().json(created))
}
