use ferrisgram::error::Result;
use ferrisgram::{error::GroupIteration, ext::Context, Bot};
use std::process::Stdio;
use std::time::Instant;
use tgbot_app::util::{send_err_msg, verify_telegram};
use tokio::process::Command;

pub async fn shell_no_output(bot: Bot, ctx: Context) -> Result<GroupIteration> {
    let msg = ctx.effective_message.unwrap();
    let chat_id = msg.chat.id;
    if !verify_telegram(chat_id) {
        return Ok(GroupIteration::EndGroups);
    }

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

    let duration = start_time.elapsed();

    bot.send_message(chat_id, format!("命令耗时: {:?}", duration))
        .send()
        .await?;

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
                bot.send_message(chat_id, format!("执行命令成功！命令：{}",cm))
                    .send()
                    .await?;
            } else {
                send_err_msg(bot, chat_id, format!("命令执行失败，命令：{}", cm)).await;
            }
        }
        Err(e) => {
            send_err_msg(bot, chat_id, format!("命令执行失败: {:?}\t失败的命令：{}", e,cm)).await;
        }
    }

    Ok(GroupIteration::EndGroups)
}
