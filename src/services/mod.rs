pub mod quotes;
use tonic::Status;

use crate::protos::quotes::quotes_service_server::QuotesServiceServer;

use self::quotes::InnerQuotesService;

pub fn quotes_service(db: crate::db::Database) -> QuotesServiceServer<InnerQuotesService> {
    QuotesServiceServer::new(InnerQuotesService::new(db))
}

pub fn map_db_err(e: mongodb::error::Error) -> Status {
    eprintln!("{e}");
    Status::internal("Internal Server Error")
}

pub fn map_bson_de_err(e: mongodb::bson::de::Error) -> Status {
    eprintln!("{e}");
    Status::internal("Internal Server Error")
}
