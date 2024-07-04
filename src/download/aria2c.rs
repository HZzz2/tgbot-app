
use ferrisgram::{error::GroupIteration, ext::Context, Bot};
use tgbot_app::util::{chunks_msg, verify_telegram};

use ferrisgram::error::Result;

pub async fn aria2c(bot: Bot, ctx: Context) -> Result<GroupIteration> {
    // Same logic as chat applies on unwrapping effective message here.
    let msg = ctx.effective_message.unwrap();
    let chat_id = msg.chat.id;
    if !verify_telegram(chat_id) {
        return Ok(GroupIteration::EndGroups);
    }
    let cm = msg.text.unwrap();
    let link = cm[8..].trim();

    let com = format!("aria2c -d aria2c_download -x 16 -s 16 {}", link);
    let task = tokio::task::spawn_blocking(move || {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(com)
            .output()
            .unwrap()
    });

    let msg = bot
        .send_message(chat_id, "正在使用aria2c下载文件中···".to_string())
        .disable_notification(true)
        .send()
        .await
        .unwrap();
    let output = task.await;

    let status = output.as_ref().unwrap().status;
    let result = if status.success() {
        String::from("文件下载成功")
    } else {
        let out = output.as_ref().unwrap().stdout.clone();
        let err = output.unwrap().stderr;
        format!(
            "
            *文件下载失败*:
            stdout: {:?}
            stderr: {:?}
            ",
            out, err
        )
    };

    // bot.send_message(chat_id, result)
    //     .parse_mode("markdown".to_string())
    //     .send()
    //     .await?;
    let _ = chunks_msg(&bot, chat_id, result).await;
    bot.delete_message(chat_id, msg.message_id).send().await?;
    // 修改消息不会修改消息时间，不能知晓下载所花费的时间
    // bot.edit_message_text(result).chat_id(chat_id).message_id(msg.message_id).send().await?;
    Ok(GroupIteration::EndGroups)
}
