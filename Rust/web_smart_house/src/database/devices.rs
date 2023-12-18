use crate::error::CustomResult;
use mongodb::bson::{doc, ser};
use serde::{Deserialize, Serialize};

use super::{house::HouseData, MongoClient};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Socket {
    power: u32,
    enable: bool,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Thermometer {
    temperature: u32,
    enable: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Devices {
    Socket(Socket),
    Thermometer(Thermometer),
}

#[derive(Clone)]
pub struct MongoDevices(MongoClient);

impl MongoDevices {
    pub async fn new(client: MongoClient) -> Self {
        Self(client)
    }

    pub async fn add_device(
        &self,
        house_name: &str,
        room_name: &str,
        data: Devices,
    ) -> CustomResult<Devices> {
        let collection = self.0.collection().await;
        let query = doc! { "name": house_name, "rooms": {"name": room_name}};
        let update = match data {
            Devices::Socket(ref socket) => {
                doc! { "$push": {"devices": ser::to_bson(&socket)?}}
            }
            Devices::Thermometer(ref thermo) => {
                doc! { "$push": {"devices": ser::to_bson(&thermo)?}}
            }
        };
        collection.update_one(query, update, None).await?;
        Ok(data)
    }

    pub async fn delete_device(
        &self,
        house_name: &str,
        room_name: &str,
        device_name: &str,
    ) -> CustomResult<HouseData> {
        let collection = self.0.collection().await;
        let query = doc! { "name": house_name, "rooms": {"name": room_name} };
        let house = collection.find_one(query.clone(), None).await?.unwrap();
        let update = doc! { "$pull": {"devices": {"name": device_name}}};
        collection.find_one_and_delete(update, None).await?;
        Ok(house)
    }
}
