pub mod quotes {
    tonic::include_proto!("quotes");
}
use quotes::{
    quotes_service_client::QuotesServiceClient,
    QuoteRequest, QuoteResponse
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let mut client = QuotesServiceClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(QuoteRequest {
        id: "627e007bab1ab6462c21a5d6".into(),
    });

    let response = client.get_quote(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}

