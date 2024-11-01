use ferrisgram::error::Result;
use ferrisgram::{error::GroupIteration, ext::Context, Bot};
use std::process::Stdio;
use std::time::Instant;
use tgbot_app::util::send_err_msg;
use tokio::process::Command;

pub async fn shell_no_output(bot: Bot, ctx: Context) -> Result<GroupIteration> {
    let msg = ctx.effective_message.unwrap();
    let chat_id = msg.chat.id;
    tgbot_app::verify_telegram_id!(chat_id);
    let cm = msg.text.unwrap();
    let cm = &cm[17..].trim(); // 去掉 "/shell_no_output " 前缀

    bot.send_message(chat_id, format!("收到命令: {}\t正在执行中", cm))
        .send()
        .await?;

    if cm.is_empty() {
        send_err_msg(bot, chat_id, "命令为空".to_string()).await;
        return Ok(GroupIteration::EndGroups);
    }

    let start_time = Instant::now();

    let output = Command::new("bash")
        .arg("-c")
        .arg(cm)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await;

    let elapsed_secs = start_time.elapsed().as_secs();
    let time_format = if elapsed_secs > 60 {
        let minutes = elapsed_secs / 60;
        let seconds = elapsed_secs % 60;
        format!("{} 分 {} 秒", minutes, seconds)
    } else {
        format!("{} 秒", elapsed_secs)
    };

    // bot.send_message(chat_id, format!("命令耗时: {:?}", time_format))
    //     .send()
    //     .await?;

    match output {
        Ok(output) => {
            // let stdout = String::from_utf8_lossy(&output.stdout);
            // let stderr = String::from_utf8_lossy(&output.stderr);
            // let status = output.status;

            // let message = format!(
            //     "命令执行结果:\n\
            //     状态: {}\n\
            //     执行时间: {:?}\n\
            //     命令: {}\n\n\
            //     标准输出 (长度 {} 字节):\n{}\n\n\
            //     错误输出 (长度 {} 字节):\n{}",
            //     status, duration, cm, stdout.len(), stdout, stderr.len(), stderr
            // );

            // 分段发送消息
            // let _ = chunks_msg(&bot, chat_id, message).await;

            let status = output.status;
            if status.success() {
                bot.send_message(
                    chat_id,
                    format!("执行命令成功！耗时：{}\t命令：{}", time_format, cm),
                )
                .send()
                .await?;
            } else {
                send_err_msg(
                    bot,
                    chat_id,
                    format!("命令执行失败，耗时：{}\t命令：{}", time_format, cm),
                )
                .await;
            }
        }
        Err(e) => {
            send_err_msg(
                bot,
                chat_id,
                format!("命令执行失败: {:?}\t失败的命令：{}", e, cm),
            )
            .await;
        }
    }

    Ok(GroupIteration::EndGroups)
}
