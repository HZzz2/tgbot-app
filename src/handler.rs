use ferrisgram::error::Result;
use ferrisgram::{error::GroupIteration, ext::Context, Bot};
use tgbot_app::util::{send_err_msg, verify_telegram};

use crate::{ai::chatgpt, yt_audio};

// 消息处理函数
pub async fn handler(bot: Bot, ctx: Context) -> Result<GroupIteration> {
    // Command Handler recieves message updates which have chat as a compulsory field.
    // Hence we can unwrap effective chat without checking if it is none.
    // let chat = ctx.effective_chat.unwrap();
    // Same logic as chat applies on unwrapping effective message here.
    let msg = ctx.clone().effective_message.unwrap();
    let chat_id = msg.chat.id;
    if !verify_telegram(chat_id) {
        return Ok(GroupIteration::EndGroups);
    }
    let content = msg.text.unwrap();
    let content = content.trim();

    // 斜杠视为命令
    if content.starts_with('/') {
        return Ok(GroupIteration::EndGroups);
    }

    // 如果是油管链接则下载m4a音频格式并发送   网页版或手机版链接
    if content.starts_with(r"https://www.youtube.com") || content.starts_with(r"https://youtu.be") {
        match yt_audio(&bot, chat_id, content.to_string()).await {
            Ok(_) => return Ok(GroupIteration::EndGroups),
            Err(e) => {
                send_err_msg(bot, chat_id, format!("**Error**: {:#?}", e)).await;
                return Ok(GroupIteration::EndGroups);
            }
        }
    }

    //todo!  ip? domain?

    //todo! 默认为AI问答
    let _ = chatgpt(bot, ctx).await;


    // Every api method creates a builder which contains various parameters of that respective method.
    // bot.copy_message(chat.id, chat.id, msg.message_id)
    //     // You must use this send() method in order to send the request to the API
    //     .send()
    //     .await?;

    // GroupIteration::EndGroups will end iteration of groups for an update.
    // This means that rest of the pending groups and their handlers won't be checked
    // for this particular update.
    Ok(GroupIteration::EndGroups)
}
