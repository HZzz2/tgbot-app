use reqwest::Client;
use serde::{Deserialize, Serialize};

use ferrisgram::{error::GroupIteration, ext::Context, Bot};

use ferrisgram::error::Result;
use tgbot_app::util::verify_telegram;
use tgbot_app::GLOBAL_CONFIG;

#[derive(Serialize, Deserialize)]
pub struct RequestBody {
    pub model: String,
    pub messages: Vec<Messages>,
    pub temperature: Option<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct Messages {
    pub role: String,
    pub content: String,
}


pub async fn chatgpt(bot: Bot, ctx: Context) -> Result<GroupIteration> {
    // Same logic as chat applies on unwrapping effective message here.
    let msg = ctx.effective_message.unwrap();
    let chat_id = msg.chat.id;
    if !verify_telegram(&chat_id.to_string()) {
        return Ok(GroupIteration::EndGroups);
    }
    let cm = msg.text.unwrap();
    let cm = cm[9..].trim();

    let tg_content = cm;
    let client = Client::new();
    let api_key = &GLOBAL_CONFIG.openai.api_key;
    let msg = Messages {
        role: "user".to_string(),
        content: tg_content.to_string(),
    };
    let request_body = RequestBody {
        model: GLOBAL_CONFIG.openai.model.clone(),
        messages: vec![msg],
        temperature: Some(0.7),
    };
    let res = client
        .post(&GLOBAL_CONFIG.openai.base_url)
        .header("Authorization", "Bearer ".to_string() + api_key)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .unwrap();
    let response_body = res.json::<serde_json::Value>().await.unwrap();
    let rep = response_body["choices"][0]["message"]["content"]
        .as_str()
        .unwrap();

    bot.send_message(chat_id, rep.to_string()).send().await?;

    Ok(GroupIteration::EndGroups)
}
