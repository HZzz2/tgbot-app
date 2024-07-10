use ferrisgram::Bot;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::GLOBAL_CONFIG;

// 出现失败后向用户发送失败信息
#[inline]
pub async fn send_err_msg<T: Display>(bot: Bot, chat_id: i64, msg: T) {
    let _ = bot
        .send_message(chat_id, format!("错误：{}", msg))
        .parse_mode(String::from("markdown"))
        .send()
        .await;
}

// 验证ID是否存在于配置文件中
#[inline]
pub fn verify_telegram(id: i64) -> bool {
    GLOBAL_CONFIG.telegram.ids.contains(&id)
}


// 分段消息的发送，telegram单条最多4,096个字符
pub async fn chunks_msg<T: AsRef<str>>(bot: &Bot, chat_id: i64, message: T) {
    for chunk in message.as_ref().chars().collect::<Vec<char>>().chunks(4000) {
        let chunk_str: String = chunk.iter().collect();
        let _ = bot.send_message(chat_id, chunk_str).send().await;
    }
}

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

pub async fn ai_q_s<T: Into<String>>(content: T) -> anyhow::Result<String> {
    let tg_content = content.into();
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
        .unwrap().trim_start_matches('"').trim_end_matches('"');

    // let _ = bot.send_message(chat_id, rep.to_string()).parse_mode("markdown".to_string()).send().await.unwrap();
    Ok(rep.to_string())
}
