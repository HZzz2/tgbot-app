use std::path::Path;

use ferrisgram::{error::GroupIteration, ext::Context, Bot};
use tgbot_app::GLOBAL_CONFIG;

use ferrisgram::error::Result;

pub async fn ytdlp(bot: Bot, ctx: Context) -> Result<GroupIteration> {
    // Same logic as chat applies on unwrapping effective message here.
    let msg = ctx.effective_message.unwrap();
    let chat_id = msg.chat.id;
    tgbot_app::verify_telegram_id!(chat_id);
    let cm = msg.text.unwrap();
    let link = cm[7..].trim();

    let cookie = GLOBAL_CONFIG.yt_dlp.cookie.as_str();
    let proxy = GLOBAL_CONFIG.yt_dlp.proxy.as_str();
    let args = GLOBAL_CONFIG.yt_dlp.args.as_str();

    let mut com = Vec::new();
    com.push("./yt-dlp");

    if !cookie.is_empty() {
        com.extend_from_slice(&["--cookies", cookie]);
    }

    if !proxy.is_empty() {
        com.extend_from_slice(&["--proxy", proxy]);
    }

    if !args.is_empty() {
        com.push(args);
    }

    com.push(link);
    let command_string = com.join(" ");
    let comm_string = command_string.clone();

    let task = tokio::task::spawn_blocking(move || {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(command_string.clone())
            .output()
            .unwrap()
    });

    let msg = bot
        .send_message(
            chat_id,
            format!("正在使用yt-dlp下载视频中···{}", comm_string),
        )
        .disable_notification(true)
        .send()
        .await
        .unwrap();
    let output = task.await;

    let status = output.as_ref().unwrap().status;
    let result = if status.success() {
        String::from("视频下载成功")
    } else {
        let file_name = "yt-dlp";
        let path = Path::new(file_name);
        if !path.exists() {
            let err_msg = r#"
当前工作目录没有yt-dlp程序: 
```shell
wget https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp
mv ./yt-dlp /root/tgbot_app
cd !$
chmod +x yt-dlp
```
"#;
            String::from(err_msg)
        } else {
            let out = output.as_ref().unwrap().stdout.clone();
            let err = output.unwrap().stderr;
            format!(
                "
            *视频下载失败*:
            stdout: {:?}
            stderr: {:?}
            ",
                out, err
            )
        }
    };

    bot.send_message(chat_id, result)
        .parse_mode("markdown".to_string())
        .send()
        .await?;
    bot.delete_message(chat_id, msg.message_id).send().await?;
    // 修改消息不会修改消息时间，不能知晓下载所花费的时间
    // bot.edit_message_text(result).chat_id(chat_id).message_id(msg.message_id).send().await?;
    Ok(GroupIteration::EndGroups)
}
