use ferrisgram::error::Result;
use ferrisgram::{error::GroupIteration, ext::Context, Bot};

use tgbot_app::util::{ai_q_s, verify_telegram};

pub async fn chatgpt(bot: Bot, ctx: Context) -> Result<GroupIteration> {
    // Same logic as chat applies on unwrapping effective message here.
    let msg = ctx.effective_message.unwrap();
    let chat_id = msg.chat.id;
    if !verify_telegram(chat_id) {
        return Ok(GroupIteration::EndGroups);
    }
    let cm = msg.text.unwrap();
    // let cm = cm[9..].trim();
    // 默认为AI问答，如果是从handle过来的则不去除前面的命令
    let cm = if cm.starts_with("/chatgpt") {
        cm[9..].trim()
    } else {
        cm[..].trim()
    };

    // let _ = ai_q_s(&bot, chat_id, cm).await;
    let ai_result = ai_q_s(cm).await;
    if let Ok(ai_answer) = ai_result {
        let _ = bot.send_message(chat_id, ai_answer).send().await;
    } else {
        let _ = bot
            .send_message(chat_id, "AI请求失败".to_string())
            .send()
            .await;
    }
    Ok(GroupIteration::EndGroups)
}
