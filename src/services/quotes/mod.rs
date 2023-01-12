use std::str::FromStr;

use crate::protos::quotes::{
    quotes_service_server::QuotesService, AuthorRequest, FilterQuotesRequest, Quote as rQuote,
    QuoteAuthor, QuoteAuthorResponse, QuoteRequest, QuoteResponse, QuoteTranslation,
};
use mongodb::bson::oid::ObjectId;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct InnerQuotesService {
    pub db: crate::db::Database,
}

impl InnerQuotesService {
    pub fn new(db: crate::db::Database) -> Self {
        Self { db }
    }
}

#[tonic::async_trait]
impl QuotesService for InnerQuotesService {
    async fn filter_quotes(
        &self,
        request: Request<FilterQuotesRequest>,
    ) -> Result<Response<QuoteResponse>, Status> {
        Err(Status::unimplemented("Not implemented"))
    }

    async fn get_quote(
        &self,
        request: Request<QuoteRequest>,
    ) -> Result<Response<QuoteResponse>, Status> {
        let id: String = request.into_inner().id;
        let id = ObjectId::from_str(&id).map_err(|_| Status::invalid_argument("Invalid id"))?;

        let quote = self
            .db
            .get_quote(id)
            .await
            .map_err(|_| Status::internal("Database error"))?;

        if quote.is_none() {
            return Err(Status::not_found("Quote not found"));
        }
        let quote = quote.unwrap();

        let author = self
            .db
            .get_author(quote.author)
            .await
            .map_err(|_| Status::internal("Database error"))?;

        let quote = rQuote {
            id: quote.id.to_hex(),
            author: author.map(|author| QuoteAuthor {
                id: author.id.to_hex(),
                name: author.name,
            }),
            translations: quote
                .translations
                .into_iter()
                .map(|(language, text)| QuoteTranslation { language, text })
                .collect(),
        };

        Ok(Response::new(QuoteResponse {
            quotes: vec![quote],
        }))
    }

    async fn get_author(
        &self,
        request: Request<AuthorRequest>,
    ) -> Result<Response<QuoteAuthorResponse>, Status> {
        let id: String = request.into_inner().id;
        let id = ObjectId::from_str(&id).map_err(|_| Status::invalid_argument("Invalid id"))?;

        let author = self
            .db
            .get_author(id)
            .await
            .map_err(|_| Status::internal("Database error"))?;

        if author.is_none() {
            return Err(Status::not_found("Author not found"));
        }
        let author = author.unwrap();

        Ok(Response::new(QuoteAuthorResponse {
            authors: vec![QuoteAuthor {
                id: author.id.to_hex(),
                name: author.name,
            }],
        }))
    }
}
