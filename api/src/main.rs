pub mod chat;
pub mod tic_tac_toe;

#[tokio::main]
async fn main() {
    // chat::main().await;
    tic_tac_toe::main().await;
}
