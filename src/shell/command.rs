use crate::util::chunks_msg;
use crate::{util::send_err_msg, GLOBAL_CONFIG};
use ferrisgram::error::Result;
use ferrisgram::{error::GroupIteration, ext::Context, Bot};
use tokio::process::Command;

pub async fn c(bot: Bot, ctx: Context) -> Result<GroupIteration> {
    // Same logic as chat applies on unwrapping effective message here.
    let msg = ctx.effective_message.unwrap();
    let chat_id = msg.chat.id;
    let cmd = &GLOBAL_CONFIG.command.cmd;
    let cm = msg.text.unwrap();
    if cm.len() == 2 {
        let mut help_message = String::from("已定制的命令:\n");
        for command in cmd.iter() {
            for (k, v) in command {
                help_message.push_str(format!("key:{} = {}\n", k, v).as_str());
            }
        }
        bot.send_message(chat_id, help_message).send().await?;
        return Ok(GroupIteration::EndGroups);
    }

    let cm = cm[3..].trim();

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

    // if cm.is_empty() {
    //     let mut help_message = String::from("未找到命令:\n");
    //     for command in cmd.iter() {
    //         for (k, v) in command {
    //             help_message.push_str(format!("key:{} = {}\n", k, v).as_str());
    //         }
    //     }
    //     send_err_msg(bot, chat_id, help_message).await;
    //     return Ok(GroupIteration::EndGroups);
    // }

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
    } else {
        bot.send_message(chat_id, format!("正在执行{}命令", li))
            .send()
            .await?;
    }
    let Ok(output) = Command::new("sh").arg("-c").arg(li.as_str()).output().await else {
        send_err_msg(bot, chat_id, "执行定制常用命令执行失败".to_string()).await;
        return Ok(GroupIteration::EndGroups);
    };
    if !&output.status.success() {
        send_err_msg(bot, chat_id, "执行定制常用命令执行失败".to_string()).await;
        return Ok(GroupIteration::EndGroups);
    }
    let output = output.stdout;
    // let output = task.await;

    // bot.send_message(chat_id, String::from_utf8_lossy(&output).to_string())
    //     .send()
    //     .await?;
    if cm != "c" {
        chunks_msg(&bot, chat_id, String::from_utf8_lossy(&output)).await;
    } else {
        bot.send_message(chat_id, "/c c命令执行成功！".to_string())
            .send()
            .await?;
    }

    Ok(GroupIteration::EndGroups)
}
