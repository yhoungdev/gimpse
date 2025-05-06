mod monitor;
mod utils;
use utils::telegram;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    crate::utils::telegram::send_telegram_message("testing event from Rust 🦀").await?;
    Ok(())
}
