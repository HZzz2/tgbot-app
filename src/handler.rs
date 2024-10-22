use std::path::Path;

use ferrisgram::error::Result;
use ferrisgram::{error::GroupIteration, ext::Context, Bot};

use tokio::fs::DirBuilder;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::ai::chatgpt;
use crate::download::ytdlp_audio;
use tgbot_app::util::{execute_one_shell, verify_telegram};
use tgbot_app::REQWEST_CLIENT;

// use crate::yt_audio;
// 消息处理函数
pub async fn handler(bot: Bot, ctx: Context) -> Result<GroupIteration> {
    // Command Handler recieves message updates which have chat as a compulsory field.
    // Hence we can unwrap effective chat without checking if it is none.
    // let chat = ctx.effective_chat.unwrap();
    // Same logic as chat applies on unwrapping effective message here.

    let msg = ctx.clone().effective_message.unwrap();
    let chat_id = msg.chat.id;
    if !verify_telegram(chat_id) {
        return Ok(GroupIteration::EndGroups);
    }

    //TODO 接收图片  。。。
    // 目前下载图片后对图片进行file和exiftool命令，发现telegram会对png图片转为jpg
    // 接收图片
    if let Some(mut photo) = ctx.clone().effective_message.unwrap().photo {
        // println!("photo: {:?},len: {}", photo, photo.len());
        // for p in photo {
        //     // let file = File::create(p.file_unique_id).await?;
        //     let p_file = bot.get_file(p.file_id).send().await?;
        //     println!("p_file：{:?}", p_file);
        // }
        let p = photo.pop().unwrap();
        let p_file = bot.get_file(p.file_id).send().await?;
        let file_path = p_file.file_path.unwrap();
        let path = Path::new(&file_path);
        let dir = path.parent().unwrap();
        let _file_name = path.file_name().unwrap();

        let url = format!(
            "https://api.telegram.org/file/bot{}/{}",
            bot.token, file_path
        );
        // 发送 HTTP 请求
        let response_bytes = REQWEST_CLIENT
            .get(url)
            .send()
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();
        // 创建本地文件
        DirBuilder::new().recursive(true).create(dir).await.unwrap();
        let mut output_file = File::create(path).await.unwrap();
        // 写入磁盘
        let _ = output_file.write_all(&response_bytes).await;

        let file_output = execute_one_shell("file".to_string(), &file_path).await;
        let exiftool_output = execute_one_shell("exiftool".to_string(), &file_path).await;

        bot.send_message(chat_id, file_output?).send().await?;

        if let Ok(exiftool_output) = exiftool_output {
            bot.send_message(chat_id, exiftool_output).send().await?;
        }
        let _ = tokio::fs::remove_file(path).await;

        return Ok(GroupIteration::EndGroups);
    }

    let content = msg.text.unwrap();
    let content = content.trim();

    // 斜杠视为命令 不知道为什么/c命令会进来，其它命令不会
    if content.starts_with('/') {
        return Ok(GroupIteration::EndGroups);
    }
    // println!("handler content:{}",content);

    // 如果是油管链接则下载m4a音频格式并发送   网页版或手机版链接
    if content.starts_with(r"https://www.youtube.com") || content.starts_with(r"https://youtu.be") {
        // 由于rusty_ytdl库目前不可用且使用unwarp会使程序崩溃（崩溃不受影响，以服务方式部署会自动重启程序）
        // 使用rusty_ytdl下载后受到上传大小50MB的限制，超过限制保留到当前工作文件夹
        // match yt_audio(&bot, chat_id, content.to_string()).await {
        //     Ok(_) => return Ok(GroupIteration::EndGroups),
        //     Err(e) => {
        //         send_err_msg(bot, chat_id, format!("**Error**: {:#?}", e)).await;
        //         return Ok(GroupIteration::EndGroups);
        //     }
        // }

        // 备选方案yt-dlp下载音频到tdl_dir目录（目前本人无法将音频上传，需借助tdl工具手动执行命令上传音频到群组等，不受50MB限制，可在线听音频）
        let _ = ytdlp_audio(bot, ctx).await;
        return Ok(GroupIteration::EndGroups);
    }

    //todo!  ip? domain?

    //TODO 接收文件 发送云沙箱

    //默认为AI问答
    let _ = chatgpt(bot, ctx).await;

    // Every api method creates a builder which contains various parameters of that respective method.
    // bot.copy_message(chat.id, chat.id, msg.message_id)
    //     // You must use this send() method in order to send the request to the API
    //     .send()
    //     .await?;

    // GroupIteration::EndGroups will end iteration of groups for an update.
    // This means that rest of the pending groups and their handlers won't be checked
    // for this particular update.
    Ok(GroupIteration::EndGroups)
}
