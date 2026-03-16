use crate::constants::request_constants::*;
use reqwest::Error;
pub async fn send_message_to_bot_func(text:&str) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let response =client
        .get(format!("{}{}/sendMessage",TELEGRAM_API,BOT_ID))
        .query(&[
            (CHAT_ID_HEADER, CHAT_ID_VALUE),
            (TEXT_HEADER, &text)
        ])
        .send()
        .await?;

    let body = response.text().await?;
    Ok(body)
}