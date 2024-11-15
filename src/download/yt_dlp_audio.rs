use std::path::Path;

use crate::GLOBAL_CONFIG;
use ferrisgram::{error::GroupIteration, ext::Context, Bot};

use ferrisgram::error::Result;
// use tokio::process::Command;

pub async fn ytdlp_audio(bot: Bot, ctx: Context) -> Result<GroupIteration> {
    // Same logic as chat applies on unwrapping effective message here.
    let msg = ctx.effective_message.unwrap();
    let chat_id = msg.chat.id;
    let cm = msg.text.unwrap();
    let link = if cm.starts_with("/ytdlp_audio") {
        cm[13..].trim()
    } else {
        cm[..].trim()
    };
    let cookie = GLOBAL_CONFIG.yt_dlp.cookie.as_str();
    let proxy = GLOBAL_CONFIG.yt_dlp.proxy.as_str();
    let args = GLOBAL_CONFIG.yt_dlp.args.as_str();

    // let com = match (cookie, proxy) {
    //     // 将格式mp3换成m4a无法在TG在线听。
    //     (ck, px) if !ck.is_empty() && !px.is_empty() => {
    //         format!(
    //             r#"./yt-dlp -x --audio-format mp3 -o "/root/tgbot-app/tdl_dir/%(title)s.%(ext)s" --cookies {} --proxy {} {}"#,
    //             ck, px, link
    //         )
    //     }
    //     (ck, _) if !ck.is_empty() => format!(
    //         r#"./yt-dlp -x --audio-format mp3 -o "/root/tgbot-app/tdl_dir/%(title)s.%(ext)s" --cookies {} {}"#,
    //         ck, link
    //     ),
    //     (_, px) if !px.is_empty() => format!(
    //         r#"./yt-dlp -x --audio-format mp3 -o "/root/tgbot-app/tdl_dir/%(title)s.%(ext)s" --proxy {} {}"#,
    //         px, link
    //     ),
    //     _ => format!(
    //         r#"./yt-dlp -x --audio-format mp3 -o "/root/tgbot-app/tdl_dir/%(title)s.%(ext)s" {}"#,
    //         link
    //     ),
    // };
    let mut com = Vec::new();
    com.push("./yt-dlp -x --audio-format mp3 -o '/root/tgbot-app/tdl_dir/%(title)s.%(ext)s'");

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

    let task = tokio::task::spawn_blocking(move || {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(command_string)
            .output()
            .unwrap()
    });
    let msg = bot
        .send_message(chat_id, "正在使用yt-dlp下载音频中···".to_string())
        .disable_notification(true)
        .send()
        .await
        .unwrap();
    let output = task.await;

    let status = output.as_ref().unwrap().status;

    let result = if status.success() {
        String::from("音频下载成功，在tdl_dir目录下")
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
            // let out = output.as_ref().unwrap().stdout.clone();
            // let err = output.unwrap().stderr;
            // format!(
            //     "
            // *音频下载失败*:
            // stdout: {:?}
            // stderr: {:?}
            // ",
            //     out, err
            // )
            "音频下载失败".to_string()
        }
    };

    bot.send_message(chat_id, result)
        .disable_notification(true)
        .send()
        .await?;
    bot.delete_message(chat_id, msg.message_id).send().await?;
    // 修改消息不会修改消息时间，不能知晓下载所花费的时间
    // bot.edit_message_text(result).chat_id(chat_id).message_id(msg.message_id).send().await?;

    // let Ok(_) = Command::new("sh").arg("-c").arg("tdl up -p /root/tgbot-app/tdl_dir -c 群组ID --rm -t 8 -s 524288 -l 4").output().await else {
    //     send_err_msg(bot, chat_id, "上传音频失败".to_string()).await;
    //     return Ok(GroupIteration::EndGroups);
    // };

    // match Command::new("sh")
    //     .arg("-c")
    //     .arg("tdl up -p /root/tgbot-app/tdl_dir -c 群组ID --rm -t 8 -s 524288 -l 4")
    //     .output()
    //     .await
    // {
    //     Ok(_) => {
    //         bot.send_message(chat_id, "上传音频成功".to_string())
    //             .send()
    //             .await?;
    //     }
    //     Err(e) => {
    //         send_err_msg(bot, chat_id, format!("上传音频失败:{:?}", e)).await;
    //     }
    // }

    Ok(GroupIteration::EndGroups)
}
