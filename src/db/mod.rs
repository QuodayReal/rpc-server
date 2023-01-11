pub mod models;

use futures::stream::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use self::models::{Quote, QuoteAuthor};

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
    pub async fn get_quotes(
        &self,
        ids: Vec<ObjectId>,
    ) -> Result<Vec<Quote>, mongodb::error::Error> {
        let filter = doc! {
            "_id": {
                "$in": ids
            }
        };

        let mut cursor = self.quotes.find(filter, None).await?;
        let mut quotes = Vec::new();
        while let Some(doc) = cursor.try_next().await? {
            quotes.push(doc);
        }

        Ok(quotes)
    }

    pub async fn get_authors(
        &self,
        ids: Vec<ObjectId>,
    ) -> Result<Vec<QuoteAuthor>, mongodb::error::Error> {
        let filter = doc! {
            "_id": {
                "$in": ids
            }
        };

        let mut cursor = self.authors.find(filter, None).await?;
        let mut authors = Vec::new();
        while let Some(doc) = cursor.try_next().await? {
            authors.push(doc);
        }

        Ok(authors)
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
