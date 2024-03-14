use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to read .env file");
    c02_diesel::start().await;
}
