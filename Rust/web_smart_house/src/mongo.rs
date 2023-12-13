use crate::error::{CustomError, CustomResult};
use crate::{ObjectId, StreamExt};
use mongodb::bson::doc;
use mongodb::Client;
use serde::{Deserialize, Serialize};
use smart_house::house::House;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HouseData {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    #[serde(flatten)]
    house: House,
}

#[derive(Clone)]
pub struct MongoHouse(Client);

impl MongoHouse {
    pub async fn new(connection_str: &str) -> Self {
        Self(Client::with_uri_str(connection_str).await.unwrap())
    }

    pub async fn create_house(&self, data: HouseData) -> CustomResult<HouseData> {
        let collection = self.0.database("smart_house").collection("houses");
        let inserted = collection.insert_one(data, None).await?;
        let id = inserted.inserted_id;
        let query = doc! { "_id": &id };
        let house = collection.find_one(query, None).await?;
        house.ok_or_else(|| CustomError::NotFound(format!("house with id: {}", id)))
    }

    pub async fn get_houses(&self) -> CustomResult<Vec<HouseData>> {
        let collection = self.0.database("smart_house").collection("houses");
        let query = doc! {};
        let mut houses = collection.find(query, None).await?;
        let mut houses_vec = Vec::new();
        while let Some(house) = houses.next().await {
            houses_vec.push(house?);
        }
        Ok(houses_vec)
    }

    pub async fn get_house(&self, house_name: &str) -> CustomResult<HouseData> {
        let collection = self.0.database("smart_house").collection("houses");
        let query = doc! { "name": house_name };
        let house = collection.find_one(query, None).await?;
        house.ok_or_else(|| CustomError::NotFound(format!("house with name: {}", house_name)))
    }

    pub async fn delete_house(&self, house_name: &str) -> CustomResult<HouseData> {
        let collection = self.0.database("smart_house").collection("houses");
        let query = doc! { "name": house_name };
        let house = collection.find_one_and_delete(query, None).await?;
        house.ok_or_else(|| CustomError::NotFound(format!("house with name: {}", house_name)))
    }
}
