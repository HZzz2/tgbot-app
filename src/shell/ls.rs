use ferrisgram::{error::GroupIteration, ext::Context, Bot};
use tgbot_app::util::verify_telegram;
use tklog::{async_error, async_info};
use tokio::process::Command;

use ferrisgram::error::Result;

pub async fn ls(bot: Bot, ctx: Context) -> Result<GroupIteration> {
    // Same logic as chat applies on unwrapping effective message here.
    let msg = ctx.effective_message.unwrap();
    let chat_id = msg.chat.id;
    tgbot_app::verify_telegram_id!(chat_id);

    if let Ok(output) = Command::new("ls").args(["-l", "-a", "-h"]).output().await {
        bot.send_message(chat_id, String::from_utf8_lossy(&output.stdout).to_string())
            .send()
            .await?;
        async_info!("ls命令调用成功");
    } else {
        bot.send_message(chat_id, "ls命令调用失败".to_owned())
            .send()
            .await?;
        async_error!("ls命令调用失败");
    }

    Ok(GroupIteration::EndGroups)
}
