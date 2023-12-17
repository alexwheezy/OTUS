pub mod devices;
pub mod house;
pub mod room;

use mongodb::{Client, Collection};

use self::house::HouseData;

#[derive(Clone)]
pub struct MongoClient {
    mongo: Client,
}

impl MongoClient {
    pub async fn new(connection_str: &str) -> Self {
        Self {
            mongo: Client::with_uri_str(connection_str).await.unwrap(),
        }
    }

    pub async fn collection(&self) -> Collection<HouseData> {
        self.mongo.database("smart_house").collection("houses")
    }
}
