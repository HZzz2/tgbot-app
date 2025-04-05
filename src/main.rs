//! 多功能Telegram机器人，提供了丰富的实用命令和功能。程序设计为以Linux服务的方式运行，并在出错时自动重启，确保稳定可靠的服务。
//! 推荐在Linux中以服务的方式进行部署 [GitHub](https://github.com/HZzz2/tgbot-app)

use std::sync::Arc;

use anyhow::Result;
use ferrisgram::ext::filters::callback_query::All;
use ferrisgram::ext::filters::message;
use ferrisgram::ext::handlers::{CallbackQueryHandler, CommandHandler, MessageHandler};
use ferrisgram::ext::{Dispatcher, Updater};
use ferrisgram::types::BotCommand;
use ferrisgram::Bot;
use tokio_cron_scheduler::{JobBuilder, JobScheduler};
// use tgbot_app::brute_force::sha1_cracker;
use tklog::{async_debug, async_fatal, async_info, Format, ASYNC_LOG, LEVEL};

use tgbot_app::GLOBAL_CONFIG;

mod handler;
mod start;
use handler::handler;
use start::start;
mod callback_handler;
use callback_handler::callback_handler;

pub mod util;

pub mod shell;
use shell::{c, ls, ping, shell, shell_no_output};
pub mod ai;
use ai::chatgpt;

pub mod download;
pub use download::{aria2c, ytdlp};

pub mod server;
pub use server::resend;

pub mod osint;
pub use osint::{dns, ip};

pub mod brute_force;
pub use brute_force::sha1_cracker;
pub use brute_force::ssh_brute;

pub mod cron;

/// 配置日志 - debug:控制台输出日志 ；release：文件输出日志
async fn async_log_init() {
    let logger = ASYNC_LOG;

    // 判断是debug还是release编译
    if cfg!(debug_assertions) {
        // 配置全局单例
        logger
            .set_console(true) // 开启控制台输出
            .set_level(LEVEL::Debug) // Set log level to Debug
            .set_format(Format::LevelFlag | Format::Date | Format::Time | Format::LongFileName);
        async_debug!("tgbot-app正在启动中，已开启debug模式日志,日志输出控制台，不输出到文件");
    } else {
        // 配置全局单例
        logger
            .set_console(false) // Disable console output
            .set_level(LEVEL::Info) // Set log level to Info
            .set_format(Format::LevelFlag | Format::Date | Format::Time | Format::LongFileName) // Define structured logging output
            .set_cutmode_by_time("./logs/tgbot-app-log.log", tklog::MODE::MONTH, 3, true) // 每月，三次备份，压缩
            .await;
        async_info!("tgbot-app正在启动中，已开启release模式日志，日志输出到文件，不输出控制台");
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    async_log_init().await;

    // 获取配置文件信息
    let config = GLOBAL_CONFIG.clone();
    async_debug!("config info:", format!("{:#?}", config));

    let bot_token = &config.telegram.bot_token;
    // 此函数创建一个新的机器人实例并相应地处理错误
    let bot: Bot = match Bot::new(bot_token, None).await {
        Ok(bot) => {
            async_info!("tgbot-app启动成功");
            bot
        }
        Err(error) => {
            async_fatal!("创建tgbot-app失败");
            panic!("创建tgbot-app失败: {}", error)
        }
    };

    let short_des = r#"
Telegram Bot助手
开源地址:https://github.com/HZzz2/tgbot-app
"#
    .to_string();
    let des: String = r#"
机器人开源地址： https://github.com/HZzz2/tgbot-app
欢迎提交功能请求，优化建议, BUG，PR
可通过机器人执行shell命令，信息搜集，常用命令执行，发送邮件，下载音频或视频等等
    "#
    .to_string();

    bot.set_my_description()
        .description(des)
        .send()
        .await
        .unwrap();
    bot.set_my_short_description()
        .short_description(short_des)
        .send()
        .await
        .unwrap();

    // bot.set_chat_menu_button()
    //     .menu_button(MenuButton::MenuButtonWebApp(MenuButtonWebApp {
    //         text: "GitHub地址".to_string(),
    //         web_app: WebAppInfo {
    //             url: "https://github.com/HZzz2/tgbot-app".to_string(),
    //         },
    //     }));
    // bot.set_chat_menu_button()
    //     .menu_button(MenuButton::MenuButtonWebApp(MenuButtonWebApp {
    //         text: "作者地址".to_string(),
    //         web_app: WebAppInfo {
    //             url: "https://github.com/HZzz2".to_string(),
    //         },
    //     }));

    // 调度程序是更新程序内部功能的一部分
    // 您可以使用它来添加处理程序。
    let dispatcher = &mut Dispatcher::new(&bot);

    let mut botcommadns: Vec<BotCommand> = Vec::with_capacity(10);
    // add_handler method maps the provided handler in group 0 automatically
    // add_handler 方法自动将提供的处理程序映射到组 0 中
    dispatcher.add_handler(CommandHandler::new("start", start));
    botcommadns.push(BotCommand {
        command: "start".to_string(),
        description: "快速开始向导".to_string(),
    });

    // shell
    dispatcher.add_handler(CommandHandler::new("ls", ls));
    botcommadns.push(BotCommand {
        command: "ls".to_string(),
        description: "显示指定工作目录下之内容（列出目前工作目录所含的文件及子目录),默认-lah"
            .to_string(),
    });

    dispatcher.add_handler(CommandHandler::new("ping", ping));
    botcommadns.push(BotCommand {
        command: "ping".to_string(),
        description: "命令用于检测与另一个主机之间的网络连接,默认发送4个数据包".to_string(),
    });
    dispatcher.add_handler(CommandHandler::new("c", c));
    botcommadns.push(BotCommand {
        command: "c".to_string(),
        description: "执行配置文件中设置的常用命令".to_string(),
    });
    dispatcher.add_handler(CommandHandler::new("shell", shell));
    botcommadns.push(BotCommand {
        command: "shell".to_string(),
        description: "执行任意shell命令,并显示结果".to_string(),
    });
    dispatcher.add_handler(CommandHandler::new("shell_no_output", shell_no_output));
    botcommadns.push(BotCommand {
        command: "shell_no_output".to_string(),
        description: "执行任意shell命令,不输出内容，只输出是否执行成功".to_string(),
    });

    // osint
    dispatcher.add_handler(CommandHandler::new("ip", ip));
    botcommadns.push(BotCommand {
        command: "ip".to_string(),
        description: "获取ip信息，地理位置".to_string(),
    });

    dispatcher.add_handler(CommandHandler::new("dns", dns));
    botcommadns.push(BotCommand {
        command: "dns".to_string(),
        description: "获取DNS相关信息".to_string(),
    });

    dispatcher.add_handler(CommandHandler::new("ssh_brute", ssh_brute));
    botcommadns.push(BotCommand {
        command: "ssh_brute".to_string(),
        description: "ssh爆破".to_string(),
    });
    dispatcher.add_handler(CommandHandler::new("sha1", sha1_cracker));
    botcommadns.push(BotCommand {
        command: "sha1".to_string(),
        description: "sha1爆破".to_string(),
    });

    // ai
    dispatcher.add_handler(CommandHandler::new("chatgpt", chatgpt));
    botcommadns.push(BotCommand {
        command: "chatgpt".to_string(),
        description: "chatgpt openai模型".to_string(),
    });

    // download
    dispatcher.add_handler(CommandHandler::new("ytdlp", ytdlp));
    botcommadns.push(BotCommand {
        command: "ytdlp".to_string(),
        description: "使用yt-dlp下载画质最佳视频，需下载yt-dlp到工作目录".to_string(),
    });

    dispatcher.add_handler(CommandHandler::new("aria2c", aria2c));
    botcommadns.push(BotCommand {
        command: "aria2c".to_string(),
        description: "使用aria2c下载文件，支持 HTTP/HTTPS、FTP、SFTP、BitTorrent 和 Metalink,默认16线程，下载的文件在aria2c_download目录下".to_string(),
    });
    // server
    dispatcher.add_handler(CommandHandler::new("resend", resend));
    botcommadns.push(BotCommand {
        command: "resend".to_string(),
        description: "使用resend发送邮件，需申请设置api和发件地址".to_string(),
    });

    bot.set_my_commands(botcommadns).send().await.unwrap();

    // add_handler_to_group is used to map the provided handler to a group manually.
    // note that handler groups are processed in ascending order.
    dispatcher.add_handler_to_group(
        MessageHandler::new(
            handler,
            message::All::filter(), //接收图片和文件以供查杀检验or?
                                    // This will restrict our echo function to the messages which
                                    // contain either text or a caption.
                                    // message::Text::filter().or(message::Caption::filter()),
        ),
        -1,
    );
    // 回调
    dispatcher.add_handler_to_group(
        CallbackQueryHandler::new(callback_handler, All::filter()),
        1,
    );

    let mut updater = Updater::new(&bot, dispatcher);

    // cron国定任务执行

    // 创建调度器
    let scheduler = JobScheduler::new().await.unwrap();

    // 添加一个每2秒执行一次的任务
    // let job = JobBuilder::new().with_timezone(chrono_tz::Asia::Shanghai)
    // .with_cron_job_type()
    //     .with_schedule("*/2 * * * * *")
    //     .unwrap()
    //     .with_run_async(Box::new(|_uuid, mut _l| {
    //         Box::pin(async move {
    //             async_info!("JHB run async every 2 seconds id");
    //             // async_info!("JHB run async every 2 seconds id {:?}", uuid);
    //             // let next_tick = l.next_tick_for_job(uuid).await;
    //             // match next_tick {
    //             //     Ok(Some(ts)) => async_info!("Next time for JHB 2s is {:?}", ts),
    //             //     _ => async_fatal!("Could not get next tick for 2s job"),
    //             // }
    //         })
    //     }))
    //     .build()
    //     .unwrap();

    // 添加一个天气预报，每日8点执行推送

    //     let job1 = JobBuilder::new()
    //         .with_timezone(chrono_tz::Asia::Shanghai) // 设置任务执行的时区为上海时区（UTC+8）如果没有指定时区，默认使用UTC
    //         .with_cron_job_type() // 指定这是一个基于cron表达式的任务（而不是基于间隔的任务）
    //         .with_schedule("0 8 * * * *") //设置cron表达式，定义任务的执行计划 每天上午8点整（0秒）执行
    //         .unwrap() //如果cron表达式格式正确，继续构建流程
    //         .with_run_async(Box::new({
    //             // 定义要异步执行的逻辑  Box::new(...)创建了一个堆分配的闭包
    //             let cbot = Arc::new(bot.clone());
    //             let config = Arc::new(config.clone());
    //             move |_uuid, _l| {
    //                 let cbot = Arc::clone(&cbot);
    //                 let config = Arc::clone(&config);
    //                 Box::pin(async move {
    //                     let tianqi:Value = REQWEST_CLIENT.get("https://cn.apihz.cn/api/tianqi/tqyb.php?id=88888888&key=88888888&sheng=湖南&place=长沙")
    //                     .send().await.unwrap().json().await.unwrap();
    //                     let str_format = format!("
    // *☀️ 天气预报 ☀️*

    // 🏙️ *地区*: {}
    // 🌡️ *温度*: {}°C
    // 💧 *湿度*: {}%
    // 🌬️ *风速*: {}m/s
    // 🍃 *风力等级*: {}
    // ",
    //     tianqi["place"],
    //     tianqi["temperature"],
    //     tianqi["humidity"],
    //     tianqi["windSpeed"],
    //     tianqi["windScale"]
    // );
    //                     let _ = cbot
    //                         .send_message(config.telegram.ids[0], str_format)
    //                         .parse_mode(String::from("markdown"))
    //                         .send()
    //                         .await;
    //                 }) // Box::pin(async move {...}) 创建了一个固定在堆上的异步Future
    //             }
    //         }))
    //         .build()
    //         .unwrap();

    let job = JobBuilder::new()
        .with_timezone(chrono_tz::Asia::Shanghai) // 设置任务执行的时区为上海时区（UTC+8）如果没有指定时区，默认使用UTC
        .with_cron_job_type()
        .with_schedule("0 0 * * * *") //设置cron表达式，定义任务的执行计划 每小时的第 0 分 0 秒执行一次任务
        .unwrap()
        .with_run_async(Box::new({
            let cbot = Arc::new(bot.clone());
            move |_uuid, _l| {
                let cbot = Arc::clone(&cbot);
                Box::pin(async move {
                    cron::tianqi(cbot).await;
                })
            }
        }))
        .build()
        .unwrap();

    // 将任务添加到调度器
    scheduler.add(job).await.unwrap();
    // 启动调度器 - 这一步非常重要！
    async_info!("启动调度器");
    scheduler.start().await.unwrap();

    // This method will start long polling through the getUpdates method
    // let _ = updater.start_polling(true).await;
    match updater.start_polling(true).await {
        Ok(_) => {
            async_info!("tgbot-app开启长轮询成功");
        }
        Err(e) => {
            async_fatal!("tgbot-app开启长轮询失败:{:?}", e);
        }
    }
    Ok(())
}


