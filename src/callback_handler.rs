use ferrisgram::error::Result;
use ferrisgram::{error::GroupIteration, ext::Context, Bot};
use tgbot_app::util::{ai_q_s, verify_telegram};

use crate::ai::PROMPT_SHELL_OUTPUT;
use crate::osint::{cb_dnsenum, cb_dnsrecon, cb_ip123};
// use crate::{ai::chatgpt, yt_audio};

// 消息处理函数
pub async fn callback_handler(bot: Bot, ctx: Context) -> Result<GroupIteration> {
    let msg = ctx.effective_message.unwrap();
    let chat_id = msg.chat.id;
    if !verify_telegram(chat_id) {
        return Ok(GroupIteration::EndGroups);
    }
    //按钮的原始文本
    let raw_content = msg.text.unwrap();

    //回调内容
    let content = ctx.update.callback_query.unwrap().data.unwrap();

    let vec_content = content.split(' ').collect::<Vec<&str>>();

    match vec_content.as_slice() {
        ["osint", "ip", "cb_ip123", arg] => {
            let _ = cb_ip123(arg, bot, chat_id).await;
        }
        ["osint", "dns", "cb_dnsrecon", arg] => {
            let _ = cb_dnsrecon(arg, bot, chat_id).await;
        }
        ["osint", "dns", "cb_dnsenum", arg] => {
            let _ = cb_dnsenum(arg, bot, chat_id).await;
        }
        ["AI分析","PROMPT_SHELL_OUTPUT"] => {
            let res = ai_q_s(format!("{}:\n{}",PROMPT_SHELL_OUTPUT,raw_content)).await.unwrap();
            let _ = bot
                .send_message(chat_id, res)
                .parse_mode("markdown".to_string())
                .send()
                .await;
        }
        ["AI分析",prompt] => {
            let res = ai_q_s(format!("{}:\n{}",prompt,raw_content)).await.unwrap();
            let _ = bot
                .send_message(chat_id, res)
                .parse_mode("markdown".to_string())
                .send()
                .await;
        }
        _ => {
            let _ = bot
                .send_message(chat_id, "*未知回调匹配*".to_string())
                .parse_mode("markdown".to_string())
                .send()
                .await;
        }
    }
    Ok(GroupIteration::EndGroups)
}
