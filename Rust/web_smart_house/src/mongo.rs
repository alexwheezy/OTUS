use std::str::FromStr;

use crate::error::{CustomError, CustomResult};
use crate::{ObjectId, StreamExt};
use mongodb::bson::{doc, Document};
use mongodb::Client;
use serde::{Deserialize, Serialize};
use smart_house::house::House;

#[derive(Clone, Serialize, Deserialize)]
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

    pub fn mongoid(&self, id: &str) -> CustomResult<ObjectId> {
        Ok(ObjectId::from_str(id)?)
    }

    pub fn create_query_find_by_id(&self, id: &str) -> CustomResult<Document> {
        let object_id = self.mongoid(id)?;
        Ok(doc! {"_id": object_id })
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

    pub async fn delete_house(&self, house_id: &str) -> CustomResult<HouseData> {
        let collection = self.0.database("smart_house").collection("houses");
        let query = self.create_query_find_by_id(house_id)?;
        let board = collection.find_one_and_delete(query, None).await?;
        board.ok_or_else(|| CustomError::NotFound(format!("house with id: {}", house_id)))
    }
}
