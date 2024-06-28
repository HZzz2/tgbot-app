use ferrisgram::error::Result;
use ferrisgram::{error::GroupIteration, ext::Context, input_file::NamedFile, Bot};
use tgbot_app::util::verify_telegram;
use tokio::{fs::File, io::AsyncReadExt};

// This is our callable function for the command handler that we declared earlier
// It will be triggered when someone send /start to the bot.
pub async fn start(bot: Bot, ctx: Context) -> Result<GroupIteration> {
    // Same logic as chat applies on unwrapping effective message here.
    let msg = ctx.effective_message.unwrap();
    let chat_id = msg.chat.id;
    if !verify_telegram(&chat_id.to_string()) {
        return Ok(GroupIteration::EndGroups);
    }
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

    let mut file = File::open("./许嵩-有何不可.mp3").await.unwrap();
    let metadata = file.metadata().await.unwrap();
    let file_size = metadata.len() as usize;

    // 创建一个足够大的 buffer
    let mut buffer = Vec::with_capacity(file_size);

    // 读取整个文件内容
    file.read_to_end(&mut buffer).await.unwrap();

    let namefile = NamedFile {
        file_name: "许嵩-有何不可.mp3".to_string(),
        file_data: buffer,
    };

    bot.send_audio(chat_id, namefile).send().await?;
    // GroupIteration::EndGroups will end iteration of groups for an update.
    // This means that rest of the pending groups and their handlers won't be checked
    // for this particular update.
    Ok(GroupIteration::EndGroups)
}
