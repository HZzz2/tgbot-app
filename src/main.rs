use ferrisgram::types::BotCommand;
use tgbot_app::GLOBAL_CONFIG;
// use ferrisgram::error::{GroupIteration, Result};
use ferrisgram::ext::filters::message;
use ferrisgram::ext::handlers::{CommandHandler, MessageHandler};
use ferrisgram::ext::{Dispatcher, Updater};
// use ferrisgram::types::LinkPreviewOptions;
use ferrisgram::Bot;

// use ferrisgram::input_file::NamedFile;
// use tokio::fs::File;
// use tokio::io::AsyncReadExt;
mod handler;
mod start;
use handler::handler;
use start::start;

mod shell;
use shell::{c, ls, ping, shell};
mod ai;
use ai::chatgpt;

pub mod download;
pub use download::{yt_audio, ytdlp};

pub mod server;
pub use server::resend;
use anyhow::Result;
#[tokio::main]
async fn main() -> Result<(),anyhow::Error>{
    // 获取配置文件信息
    let config = GLOBAL_CONFIG.clone();

    let bot_token = &config.telegram.bot_token;
    // 此函数创建一个新的机器人实例并相应地处理错误
    let bot = match Bot::new(bot_token, None).await {
        Ok(bot) => bot,
        Err(error) => panic!("无法创建bot: {}", error),
    };
    bot.set_my_description().description("机器人开源地址：https://github.com/HZzz2/tgbot-app".to_string()).send().await.unwrap();

    // bot.set_my_commands(vec![BotCommand{command:"resend".to_string(),description:"使用resen发送邮件，需申请设置api和发件地址".to_string()}]).send().await.unwrap();
    // 调度程序是更新程序内部功能的一部分
    // 您可以使用它来添加处理程序。
    let dispatcher = &mut Dispatcher::new(&bot);

    // add_handler method maps the provided handler in group 0 automatically
    // add_handler 方法自动将提供的处理程序映射到组 0 中
    dispatcher.add_handler(CommandHandler::new("start", start));
    bot.set_my_commands(vec![BotCommand{command:"start".to_string(),description:"快速开始向导".to_string()}]).send().await.unwrap();

    // shell
    dispatcher.add_handler(CommandHandler::new("ls", ls));
    bot.set_my_commands(vec![BotCommand{command:"ls".to_string(),description:"显示指定工作目录下之内容（列出目前工作目录所含的文件及子目录),默认-lah".to_string()}]).send().await.unwrap();
    dispatcher.add_handler(CommandHandler::new("ping", ping));
    bot.set_my_commands(vec![BotCommand{command:"ping".to_string(),description:"命令用于检测与另一个主机之间的网络连接,默认发送4个数据包".to_string()}]).send().await.unwrap();
    dispatcher.add_handler(CommandHandler::new("c", c));
    bot.set_my_commands(vec![BotCommand{command:"c".to_string(),description:"执行配置文件中设置的常用命令".to_string()}]).send().await.unwrap();
    dispatcher.add_handler(CommandHandler::new("shell", shell));
    bot.set_my_commands(vec![BotCommand{command:"shell".to_string(),description:"执行任意shell命令".to_string()}]).send().await.unwrap();

    // ai
    dispatcher.add_handler(CommandHandler::new("chatgpt", chatgpt));
    bot.set_my_commands(vec![BotCommand{command:"chatgpt".to_string(),description:"chatgpt 大语言模型".to_string()}]).send().await.unwrap();


    // server
    dispatcher.add_handler(CommandHandler::new("resend", resend));
    bot.set_my_commands(vec![BotCommand{command:"resend".to_string(),description:"使用resen发送邮件，需申请设置api和发件地址".to_string()}]).send().await.unwrap();


    // download
    dispatcher.add_handler(CommandHandler::new("ytdlp", ytdlp));
    bot.set_my_commands(vec![BotCommand{command:"ytdlp".to_string(),description:"使用yt-dlp下载画质最佳视频，需下载yt-dlp到工作目录".to_string()}]).send().await.unwrap();

    // add_handler_to_group is used to map the provided handler to a group manually.
    // note that handler groups are processed in ascending order.
    dispatcher.add_handler_to_group(
        MessageHandler::new(
            handler,
            // This will restrict our echo function to the messages which
            // contain either text or a caption.
            message::Text::filter().or(message::Caption::filter()),
        ),
        1,
    );

    let mut updater = Updater::new(&bot, dispatcher);

    // This method will start long polling through the getUpdates method
    let _ = updater.start_polling(true).await;
    Ok(())
}
