pub mod models;

use self::models::{Quote, QuoteAuthor};
use futures::stream::TryStreamExt;
use mongodb::bson::Document;
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::options::{AggregateOptions, FindOptions};

#[derive(Debug)]
pub struct Database {
    client: mongodb::Client,
    quotes: mongodb::Collection<Quote>,
    authors: mongodb::Collection<QuoteAuthor>,
}

pub async fn create() -> Result<mongodb::Client, mongodb::error::Error> {
    let mongo_uri = std::env::var("MONGO_URI").expect("MONGO_URI must be set");
    let client = mongodb::Client::with_uri_str(&mongo_uri).await?;

    Ok(client)
}

pub fn new(db: mongodb::Client) -> Result<Database, Box<dyn std::error::Error>> {
    let db_name = std::env::var("DB_NAME")?;
    let quotes = db.database(&db_name).collection("Quotes");
    let authors = db.database(&db_name).collection("Authors");

    Ok(Database {
        client: db,
        quotes,
        authors,
    })
}

impl Database {
    pub async fn aggregate_quotes(
        &self,
        pipeline: impl IntoIterator<Item = Document>,
        options: impl Into<Option<AggregateOptions>>,
    ) -> Result<Vec<Document>, mongodb::error::Error> {
        let mut cursor = self.quotes.aggregate(pipeline, options).await?;
        let mut docs: Vec<_> = Vec::new();
        while let Some(doc) = cursor.try_next().await? {
            docs.push(doc);
        }

        Ok(docs)
    }

    pub async fn get_quote(&self, id: ObjectId) -> Result<Option<Quote>, mongodb::error::Error> {
        let filter = doc! {
            "_id": id
        };

        let quote = self.quotes.find_one(filter, None).await?;

        Ok(quote)
    }

    pub async fn get_author(
        &self,
        id: ObjectId,
    ) -> Result<Option<QuoteAuthor>, mongodb::error::Error> {
        let filter = doc! {
            "_id": id
        };

        let author = self.authors.find_one(filter, None).await?;

        Ok(author)
    }
}
