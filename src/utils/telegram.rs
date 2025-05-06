use std::env;
use teloxide::prelude::*;

const TELEGRAM_BOT_TOKEN: &str = env!("TELEGRAM_BOT_TOKEN");
const TELEGRAM_CHAT_ID: &str = env!("TELEGRAM_CHAT_ID");

async fn send_telegram_message(content: &str) -> ResponseResult<Message> {
    let bot = Bot::new(TELEGRAM_BOT_TOKEN);
    bot.send_message(TELEGRAM_CHAT_ID, content).await
}

