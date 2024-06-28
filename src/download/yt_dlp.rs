use std::path::Path;

use ferrisgram::{error::GroupIteration, ext::Context, Bot};
use tgbot_app::{util::verify_telegram, GLOBAL_CONFIG};

use ferrisgram::error::Result;

pub async fn ytdlp(bot: Bot, ctx: Context) -> Result<GroupIteration> {
    // Same logic as chat applies on unwrapping effective message here.
    let msg = ctx.effective_message.unwrap();
    let chat_id = msg.chat.id;
    if !verify_telegram(&chat_id.to_string()) {
        return Ok(GroupIteration::EndGroups);
    }
    let cm = msg.text.unwrap();
    let link = cm[7..].trim();

    let com = if GLOBAL_CONFIG.yt_dlp.proxy.is_empty() {
        format!(r#"./yt-dlp {}"#, link)
    } else {
        format!(
            r#"./yt-dlp --proxy {} {}"#,
            GLOBAL_CONFIG.yt_dlp.proxy, link
        )
    };
    // let com = format!(r#"./yt-dlp {}"#, link);
    let task = tokio::task::spawn_blocking(move || {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(com)
            .output()
            .unwrap()
    });
    let output = task.await;

    let status = output.unwrap().status;
    let result = if status.success() {
        String::from("视频下载成功")
    } else {
        let file_name = "yt-dlp";
        let path = Path::new(file_name);
        if !path.exists() {
            String::from("当前工作目录没有yt-dlp程序")
        } else {
            String::from("视频下载失败")
        }
    };

    bot.send_message(chat_id, result).send().await?;

    Ok(GroupIteration::EndGroups)
}
