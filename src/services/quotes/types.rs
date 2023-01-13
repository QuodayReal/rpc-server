use std::collections::HashMap;

use mongodb::bson::{self, from_bson, Bson, de::Error as BsonDecodeError, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct QuoteSearchAggregateDoc {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub author: crate::db::models::QuoteAuthor,
    #[serde(rename = "quote")]
    pub translations: HashMap<String, String>,
}

impl QuoteSearchAggregateDoc {
    pub fn from_document(
        doc: bson::Document,
    ) -> Result<QuoteSearchAggregateDoc, BsonDecodeError> {
        from_bson(Bson::Document(doc))
    }
}
