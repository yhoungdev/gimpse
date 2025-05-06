use std::env;
use teloxide::prelude::*;

use dotenv::dotenv;

pub async fn send_telegram_message(content: &str) -> Result<Message, Box<dyn std::error::Error>> {

    dotenv().ok();
    let bot_token = env::var("TELEGRAM_BOT_TOKEN")
        .expect("TELEGRAM_BOT_TOKEN environment variable is not set");
    let chat_id_str = env::var("TELEGRAM_CHAT_ID")
        .expect("TELEGRAM_CHAT_ID environment variable is not set");

    let chat_id = ChatId(chat_id_str.parse::<i64>()
        .expect("TELEGRAM_CHAT_ID must be a valid integer"));

    let bot = Bot::new(bot_token);
    Ok(bot.send_message(chat_id, content).await?)
}

