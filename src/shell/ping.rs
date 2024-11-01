use ferrisgram::{error::GroupIteration, ext::Context, Bot};
use tgbot_app::util::verify_telegram;
use tokio::process::Command;

use ferrisgram::error::Result;

pub async fn ping(bot: Bot, ctx: Context) -> Result<GroupIteration> {
    // Same logic as chat applies on unwrapping effective message here.
    let msg = ctx.effective_message.unwrap();
    let chat_id = msg.chat.id;
    tgbot_app::verify_telegram_id!(chat_id);
    let cm = msg.text.unwrap();
    let cm = &cm[7..].trim();

    let output = Command::new("ping")
        .args(["-c", "4", cm])
        .output()
        .await
        .expect("ping command failed")
        .stdout;

    bot.send_message(chat_id, String::from_utf8_lossy(&output).to_string())
        .send()
        .await?;

    Ok(GroupIteration::EndGroups)
}
