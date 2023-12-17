use crate::error::{CustomError, CustomResult};
use crate::{ObjectId, StreamExt};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

use super::MongoClient;

#[derive(Clone, Serialize, Deserialize)]
pub enum DevicesData {
    Socket,
    Thermo,
}

#[derive(Clone)]
pub struct MongoDevices(MongoClient);

impl MongoDevices {
    pub async fn new(client: MongoClient) -> Self {
        Self(client)
    }
}
