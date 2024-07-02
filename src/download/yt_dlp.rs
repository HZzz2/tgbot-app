use std::path::Path;

use ferrisgram::{error::GroupIteration, ext::Context, Bot};
use tgbot_app::{util::verify_telegram, GLOBAL_CONFIG};

use ferrisgram::error::Result;

pub async fn ytdlp(bot: Bot, ctx: Context) -> Result<GroupIteration> {
    // Same logic as chat applies on unwrapping effective message here.
    let msg = ctx.effective_message.unwrap();
    let chat_id = msg.chat.id;
    if !verify_telegram(chat_id) {
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

    let task = tokio::task::spawn_blocking(move || {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(com)
            .output()
            .unwrap()
    });

    let msg = bot
        .send_message(chat_id, "正在使用yt-dlp下载视频中···".to_string())
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
            stdout: {:#?}
            stderr: {:#?}
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
