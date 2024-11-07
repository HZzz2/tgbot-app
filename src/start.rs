use ferrisgram::error::Result;
use ferrisgram::{error::GroupIteration, ext::Context, Bot};

// use tokio::{fs::File, io::AsyncReadExt};

// This is our callable function for the command handler that we declared earlier
// It will be triggered when someone send /start to the bot.
pub async fn start(bot: Bot, ctx: Context) -> Result<GroupIteration> {
    // Same logic as chat applies on unwrapping effective message here.
    let msg = ctx.effective_message.unwrap();
    let chat_id = msg.chat.id;
    tgbot_app::verify_telegram_id!(chat_id);

    let help_msg = r#"
🤖 Telegram Bot 帮助信息
Rust语言编写并开源 🦀，GitHub：https://github.com/HZzz2/tgbot-app

🛠️ 可用命令：

/c {key} - 执行键 key 对应的自定义命令。如果 key 不存在，则显示所有自定义命令 📜
/shell {命令} - 执行任意 Shell 命令并返回标准输出和标准错误信息 🖥️
/shell_no_output {命令} - 执行任意 Shell 命令，返回命令是否执行成功而不返回相关输出 🖥️
/ip {ip地址} - 查询 IP 相关信息 🌍
/dns {ip地址} - 查询 DNS 相关信息 🌐
/chatgpt {消息} - 与 AI 进行对话 或直接发送{消息} 单次对话 🤖
/ls - 显示当前目录下的所有文件 📂（不支持显示指定目录。如有需要，使用 /shell ls xxx）
/ping {example.com} - 检测与另一主机之间的网络连接。默认发送4个数据包 🌐
/aria2c {链接} - 使用aria2c下载受aria2c支持的文件到aria2c_download文件夹下
/ytdlp {视频链接} - 使用yt-dlp下载最佳音视频到当前目录下
/ssh_brute {ip地址} - ssh登录密码找回
发送非命令消息默认与 AI 进行单次对话 💬 例如发送：红烧鱼怎么做？ 🍲
发送油管链接默认下载音频 🎵（工作目录下需要 yt-dlp ）
📌 其他信息：

您可以通过 /c 1 命令执行自定义命令，例如：/c 1 查看本机IP信息 (配置文件中可自定义)🌐
您可以使用 /shell 命令执行任何 Shell 命令，例如：/shell cd /home/user 💻
您可以使用 /chatgpt xxx 命令与 AI 进行对话，例如：/chatgpt 辣椒炒肉怎么做？ 🍲
您可以使用 /ls 命令显示当前目录下的所有文件，例如：/ls 📋
您可以使用 /ping example.com 命令检测与另一主机之间的网络连接，例如：/ping google.com 🌐
"#;
    bot.send_message(chat_id, help_msg.to_string())
        .parse_mode(String::from("HTML"))
        .send()
        .await
        .unwrap();

    //     let mut link_preview_options = LinkPreviewOptions::new();
    //     link_preview_options.is_disabled = Some(true);
    //     // Ferrisgram offers some custom helpers which make your work easy
    //     // Here we have used one of those helpers known as msg.reply
    //     msg.reply(
    //         &bot,
    //         "Hey! I am an echo bot built using [Ferrisgram](https://github.com/ferrisgram/ferrisgram).
    // I will repeat your messages.",
    //     )
    //     // this method will ensure that our text will be sent with markdown formatting.
    //     .parse_mode("markdown".to_string())
    //     .link_preview_options(link_preview_options)
    //     // You must use this send() method in order to send the request to the API
    //     .send()
    //     .await?;

    // bot.send_photo(chat_id, "cat.jpg").send().await?;
    // bot.send_audio(chat_id, "许嵩-有何不可.mp3").send().await?;

    // let mut file = File::open("./许嵩-有何不可.mp3").await.unwrap();
    // let metadata = file.metadata().await.unwrap();
    // let file_size = metadata.len() as usize;

    // // 创建一个足够大的 buffer
    // let mut buffer = Vec::with_capacity(file_size);

    // // 读取整个文件内容
    // file.read_to_end(&mut buffer).await.unwrap();

    // let namefile = NamedFile {
    //     file_name: "许嵩-有何不可.mp3".to_string(),
    //     file_data: buffer,
    // };

    // bot.send_audio(chat_id, namefile).send().await?;
    // GroupIteration::EndGroups will end iteration of groups for an update.
    // This means that rest of the pending groups and their handlers won't be checked
    // for this particular update.
    Ok(GroupIteration::EndGroups)
}
