use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Quote {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub author: ObjectId,
    #[serde(rename = "quote")]
    pub translations: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuoteAuthor {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
}
