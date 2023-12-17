use crate::error::{CustomError, CustomResult};
use mongodb::bson::{doc, ser};
use serde::{Deserialize, Serialize};

use super::devices::DevicesData;
use super::MongoClient;

#[derive(Clone, Serialize, Deserialize)]
pub struct RoomData {
    name: String,
    devices: Vec<DevicesData>,
}

impl RoomData {
    pub fn devices(&self) -> &Vec<DevicesData> {
        &self.devices
    }
}

#[derive(Clone)]
pub struct MongoRoom(MongoClient);

impl MongoRoom {
    pub async fn new(client: MongoClient) -> Self {
        Self(client)
    }

    pub async fn add_room(&self, house_name: &str, data: RoomData) -> CustomResult<RoomData> {
        let collection = self.0.collection().await;
        let query = doc! { "name": house_name };
        let update = doc! { "$push": {"rooms": ser::to_bson(&data)? } };
        collection.update_one(query, update, None).await?;
        self.get_room(&house_name, &data.name).await
    }

    pub async fn get_room(&self, house_name: &str, room_name: &str) -> CustomResult<RoomData> {
        let collection = self.0.collection().await;
        let query = doc! { "name": house_name };
        let house = collection.find_one(query, None).await?.unwrap();
        let data = house.rooms().iter().find(|&room| room.name == room_name);
        match data {
            None => Err(CustomError::NotFound(format!(
                "room with id: {}",
                room_name
            ))),
            Some(room) => Ok(room.clone()),
        }
    }
}
