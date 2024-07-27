use ferrisgram::error::Result;
use ferrisgram::{error::GroupIteration, ext::Context, Bot};
use tgbot_app::util::verify_telegram;
use tgbot_app::{util::send_err_msg, GLOBAL_CONFIG};
use tokio::process::Command;

pub async fn c(bot: Bot, ctx: Context) -> Result<GroupIteration> {
    // Same logic as chat applies on unwrapping effective message here.
    let msg = ctx.effective_message.unwrap();
    let chat_id = msg.chat.id;
    if !verify_telegram(chat_id) {
        return Ok(GroupIteration::EndGroups);
    }
    let cm = msg.text.unwrap();
    let cm = cm[3..].trim();

    let cmd = &GLOBAL_CONFIG.command.cmd;
    // let li = match cmd.get(cm) {
    //     Some(cmd_value) => cmd_value,
    //     None => {
    //         let mut help_message = String::new();
    //         for (k, v) in cmd {
    //             help_message.push_str(format!("key:{} = {}\n", k, v).as_str());
    //         }
    //         send_err_msg(bot, chat_id, help_message).await;
    //         return Ok(GroupIteration::EndGroups);
    //     }
    // };

    if cm.is_empty() {
        let mut help_message = String::from("未找到命令:\n");
        for command in cmd.iter() {
            for (k, v) in command {
                help_message.push_str(format!("key:{} = {}\n", k, v).as_str());
            }
        }
        send_err_msg(bot, chat_id, help_message).await;
        return Ok(GroupIteration::EndGroups);
    }

    let mut li = "not_find_command".to_string();
    for c in cmd {
        match c.get(cm) {
            Some(v) => {
                li.clone_from(v);
                break;
            }
            None => continue,
        }
    }
    if li == "not_find_command" {
        let mut help_message = String::from("未找到命令:\n");
        for command in cmd.iter() {
            for (k, v) in command {
                help_message.push_str(format!("key:{} = {}\n", k, v).as_str());
            }
        }
        send_err_msg(bot, chat_id, help_message).await;
        return Ok(GroupIteration::EndGroups);
    }
    let Ok(output) = Command::new("sh").arg("-c").arg(li).output().await else {
        send_err_msg(bot, chat_id, "执行定制常用命令执行失败".to_string()).await;
        return Ok(GroupIteration::EndGroups);
    };
    if !&output.status.success() {
        send_err_msg(bot, chat_id, "执行定制常用命令执行失败".to_string()).await;
        return Ok(GroupIteration::EndGroups);
    }
    let output = output.stdout;
    // let output = task.await;

    bot.send_message(chat_id, String::from_utf8_lossy(&output).to_string())
        .send()
        .await?;

    Ok(GroupIteration::EndGroups)
}
