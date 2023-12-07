use crate::error;
use crate::mongo;

use actix_web::web::Path;
use actix_web::{web, HttpResponse};

use error::CustomResult;
use mongo::{HouseData, MongoHouse};
use std::sync::Arc;

#[actix_web::post("/house")]
async fn create_house(
    house_data: web::Json<HouseData>,
    houses: web::Data<Arc<MongoHouse>>,
) -> CustomResult<HttpResponse> {
    let data = house_data.into_inner();
    let created = houses.create_house(data).await?;
    Ok(HttpResponse::Ok().json(created))
}

#[actix_web::get("/")]
async fn get_houses(houses: web::Data<Arc<MongoHouse>>) -> CustomResult<HttpResponse> {
    let data = houses.get_houses().await?;
    Ok(HttpResponse::Ok().json(data))
}

#[actix_web::delete("/house/{id}")]
async fn delete_house(
    path: Path<String>,
    houses: web::Data<Arc<MongoHouse>>,
) -> CustomResult<HttpResponse> {
    let data = path.into_inner();
    let deleted = houses.delete_house(&data).await?;
    Ok(HttpResponse::Ok().json(deleted))
}
