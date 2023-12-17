use crate::database;
use crate::error;

use actix_web::{web, HttpResponse};

use database::house::{HouseData, MongoHouse};
use error::CustomResult;
use std::sync::Arc;

#[actix_web::get("/")]
async fn get_houses(houses: web::Data<Arc<MongoHouse>>) -> CustomResult<HttpResponse> {
    let data = houses.get_houses().await?;
    Ok(HttpResponse::Ok().json(data))
}

#[actix_web::post("/house")]
async fn new_house(
    house_data: web::Json<HouseData>,
    houses: web::Data<Arc<MongoHouse>>,
) -> CustomResult<HttpResponse> {
    let data = house_data.into_inner();
    let created = houses.new_house(data).await?;
    Ok(HttpResponse::Ok().json(created))
}

#[actix_web::get("/house/{house_name}")]
async fn get_house(
    path: web::Path<String>,
    houses: web::Data<Arc<MongoHouse>>,
) -> CustomResult<HttpResponse> {
    let data = path.into_inner();
    let deleted = houses.get_house(&data).await?;
    Ok(HttpResponse::Ok().json(deleted))
}

#[actix_web::delete("/house/{house_name}")]
async fn delete_house(
    path: web::Path<String>,
    houses: web::Data<Arc<MongoHouse>>,
) -> CustomResult<HttpResponse> {
    let data = path.into_inner();
    let deleted = houses.delete_house(&data).await?;
    Ok(HttpResponse::Ok().json(deleted))
}
