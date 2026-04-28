pub mod chat;

#[tokio::main]
async fn main() {
    chat::main().await;
}
