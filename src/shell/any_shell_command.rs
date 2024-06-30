use ferrisgram::{error::GroupIteration, ext::Context, Bot};
use tgbot_app::util::{send_err_msg, verify_telegram};
use tokio::process::Command;

use ferrisgram::error::Result;

pub async fn shell(bot: Bot, ctx: Context) -> Result<GroupIteration> {
    // Same logic as chat applies on unwrapping effective message here.
    let msg = ctx.effective_message.unwrap();
    let chat_id = msg.chat.id;
    if !verify_telegram(chat_id) {
        return Ok(GroupIteration::EndGroups);
    }
    let cm = msg.text.unwrap();
    let cm = &cm[7..].trim();

    let Ok(output) = Command::new("sh").arg("-c").arg(cm).output().await else {
        send_err_msg(bot, chat_id, "任意shell命令执行失败".to_string()).await;
        return Ok(GroupIteration::EndGroups);
    };
    if !&output.status.success() {
        send_err_msg(bot, chat_id, "任意shell命令执行失败".to_string()).await;
        return Ok(GroupIteration::EndGroups);
    }
    let output = output.stdout;

    bot.send_message(chat_id, String::from_utf8_lossy(&output).to_string())
        .send()
        .await?;

    Ok(GroupIteration::EndGroups)
}
