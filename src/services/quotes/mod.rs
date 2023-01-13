pub mod types;

use self::types::QuoteSearchAggregateDoc;
use super::{map_bson_de_err, map_db_err};
use crate::protos::quotes::{
    quotes_service_server::QuotesService, AuthorRequest, LimitFilter, Quote as rQuote, QuoteAuthor,
    QuoteAuthorResponse, QuoteRequest, QuoteResponse, QuoteTranslation, SearchQuotesRequest,
};
use mongodb::bson::{doc, oid::ObjectId};
use std::str::FromStr;
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
    async fn search_quotes(
        &self,
        request: Request<SearchQuotesRequest>,
    ) -> Result<Response<QuoteResponse>, Status> {
        let req = request.into_inner();
        let mut pipeline = Vec::new();
        let limits = req.limit.unwrap_or(LimitFilter {
            limit: 10,
            skip: 0
        });

        if req.random.is_some() {
            pipeline.push(doc! {
                "$sample": doc! {
                    "size": limits.limit
                }
            });
        }

        pipeline.push(doc! {
            "$skip": limits.skip,
        });
        pipeline.push(doc! {
            "$limit": limits.limit,
        });

        if !req.author.is_empty() {
            let author_id = ObjectId::from_str(&req.author)
                .map_err(|_| Status::invalid_argument("Invalid id"))?;
            pipeline.push(doc! {
                "$match": doc! {
                    "author": author_id
                }
            });
        }

        pipeline.push(doc! {
            "$lookup": doc! {
                "from": "Authors",
                "localField": "author",
                "foreignField": "_id",
                "as": "author"
            }
        });

        pipeline.push(doc! {
            "$unwind": doc! {
                "path": "$author",
                "preserveNullAndEmptyArrays": true
            }
        });

        let quotes = self
            .db
            .aggregate_quotes(pipeline, None)
            .await
            .map_err(map_db_err)?;
        let mut response = QuoteResponse::default();

        for quote in quotes {
            let quote = QuoteSearchAggregateDoc::from_document(quote).map_err(map_bson_de_err)?;

            let quote = rQuote {
                id: quote.id.to_hex(),
                author: Some(QuoteAuthor {
                    id: quote.author.id.to_hex(),
                    name: quote.author.name,
                }),
                translations: quote
                    .translations
                    .into_iter()
                    .map(|(language, text)| QuoteTranslation { language, text })
                    .collect(),
            };

            response.quotes.push(quote);
        }

        Ok(Response::new(response))
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
