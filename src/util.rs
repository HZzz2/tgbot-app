use std::fmt::Display;

use ferrisgram::Bot;

use crate::GLOBAL_CONFIG;

// 出现失败后向用户发送失败信息
#[inline]
pub async fn send_err_msg<T: Display>(bot: Bot, chat_id: i64, msg: T) {
    let _ = bot
        .send_message(chat_id, format!("Error：{}", msg))
        .parse_mode(String::from("markdown"))
        .send()
        .await;
}

// 验证ID是否存在于配置文件中
#[inline]
pub fn verify_telegram(id: i64) -> bool
{
    GLOBAL_CONFIG.telegram.ids.contains(&id)
}
