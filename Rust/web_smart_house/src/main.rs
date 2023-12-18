mod database;
mod endpoints;
mod error;

use crate::body::BoxBody;

use actix_web::body;
use actix_web::http::StatusCode;
use actix_web::web::Data;
use actix_web::{web, App, HttpResponse, HttpServer};

use database::devices::MongoDevices;
use database::room::MongoRoom;
use error::CustomResult;
use futures::StreamExt;
use log::LevelFilter;

use database::house::MongoHouse;
use database::MongoClient;
use endpoints::{devices, house, room};
use mongodb::bson::oid::ObjectId;

use std::env;
use std::error::Error;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv()?;
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();

    let connection = env::var("MONGO_CONNECTION")?;
    let mongo = MongoClient::new(&connection).await;
    let house_data = Arc::new(MongoHouse::new(mongo.clone()).await);
    let room_data = Arc::new(MongoRoom::new(mongo.clone()).await);
    let devices_data = Arc::new(MongoDevices::new(mongo.clone()).await);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(house_data.clone()))
            .service(house::new_house)
            .service(house::get_house)
            .service(house::get_houses)
            .service(house::delete_house)
            .app_data(Data::new(room_data.clone()))
            .service(room::add_room)
            .service(room::get_room)
            .service(room::delete_room)
            .app_data(Data::new(devices_data.clone()))
            .service(devices::add_device)
            .service(devices::delete_device)
            .default_service(web::to(default_response))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;
    Ok(())
}

async fn default_response() -> CustomResult<HttpResponse> {
    Ok(HttpResponse::Ok().body("Go to 'https:://localhost:8080'"))
}
