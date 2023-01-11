pub mod quotes;
use quotes::proto::quotes_service_server::QuotesServiceServer;

use self::quotes::InnerQuotesService;

pub fn quotes_service(db: crate::db::Database) -> QuotesServiceServer<InnerQuotesService> {
    QuotesServiceServer::new(InnerQuotesService::new(db))
}
