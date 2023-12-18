use crate::database::room::MongoRoom;
use crate::database::{self, room::RoomData};
use crate::error;

use actix_web::{web, HttpResponse};

use database::house::MongoHouse;
use error::CustomResult;
use std::sync::Arc;

#[actix_web::get("/")]
async fn get_houses(houses: web::Data<Arc<MongoHouse>>) -> CustomResult<HttpResponse> {
    let data = houses.get_houses().await?;
    Ok(HttpResponse::Ok().json(data))
}

#[actix_web::post("/houses/{house_name}/rooms")]
async fn add_room(
    house_name: web::Path<String>,
    room_data: web::Json<RoomData>,
    rooms: web::Data<Arc<MongoRoom>>,
) -> CustomResult<HttpResponse> {
    let data = room_data.into_inner();
    let created = rooms.add_room(&house_name, data).await?;
    Ok(HttpResponse::Ok().json(created))
}

#[actix_web::get("/houses/{house_name}/rooms/{room_name}")]
async fn get_room(
    path: web::Path<(String, String)>,
    rooms: web::Data<Arc<MongoRoom>>,
) -> CustomResult<HttpResponse> {
    let (house_name, room_name) = path.into_inner();
    let created = rooms.get_room(&house_name, &room_name).await?;
    Ok(HttpResponse::Ok().json(created))
}

#[actix_web::delete("/houses/{house_name}/rooms/{room_name}")]
async fn delete_room(
    path: web::Path<(String, String)>,
    rooms: web::Data<Arc<MongoRoom>>,
) -> CustomResult<HttpResponse> {
    let (house_name, room_name) = path.into_inner();
    let created = rooms.delete_room(&house_name, &room_name).await?;
    Ok(HttpResponse::Ok().json(created))
}
