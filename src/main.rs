pub mod db;
pub mod services;

use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let addr = "[::1]:50051".parse()?;

    let db_client = db::create().await?;
    let db_client = db::new(db_client)?;

    println!("Listening on {}", addr);
    Server::builder()
        .add_service(services::quotes_service(db_client))
        .serve(addr)
        .await?;

    Ok(())
}
