use std::path::PathBuf;

use anyhow::anyhow;
use anyhow::Result;
use ferrisgram::input_file::NamedFile;
use ferrisgram::Bot;
use rusty_ytdl::RequestOptions;
use rusty_ytdl::Video;
use rusty_ytdl::VideoOptions;
use rusty_ytdl::VideoQuality;
use rusty_ytdl::VideoSearchOptions;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use tgbot_app::util::send_err_msg;
use tgbot_app::GLOBAL_CONFIG;

// Telegram最大允许发送文件的大小，超过大小则不发
const MAX_FILE_SIZE: usize = 50 * 1024 * 1024; // 50MB

pub async fn yt_audio(bot: &Bot, chat_id: i64, url: String) -> Result<(), String> {
    let msg = bot
        .send_message(chat_id, "正在下载音频···".to_string())
        .disable_notification(true)
        .send()
        .await
        .unwrap();
    // 下载高质量音频格式文件
    let pathbuf = match down_mp3(&url, VideoQuality::Highest).await {
        Ok(pf) => pf,
        Err(e) => {
            send_err_msg(
                bot.clone(),
                chat_id,
                format!("下载高音质音频出错: {:#?}", e),
            )
            .await;
            return Ok(());
        }
    };

    let nf = read_audio(pathbuf).await;
    let namefile = NamedFile {
        file_name: nf.0.clone(),
        file_data: nf.1.clone(),
    };
    // 高音频小于50MB则发送，大于则下载低音质
    if nf.1.len() < MAX_FILE_SIZE {
        match bot
            .send_audio(chat_id, namefile)
            .disable_notification(true)
            .send()
            .await
        {
            Ok(_) => {
                // 成功发送高品质音频后则删除文件
                let _ = tokio::fs::remove_file(nf.0).await;
            }
            Err(_) => {
                //发送失败则下载低品质音频
                let pathbuf_low = down_mp3(&url, VideoQuality::Lowest)
                    .await
                    .expect("下载低品质音频失败");
                // 构造发送音频参数
                let nf_low = read_audio(pathbuf_low).await;
                // 检测低品质音频是否超过50MB，如果超过则不发送，telegram发送限制50MB以下。超50MB需自建API服务器，难申请
                if nf_low.1.len() > MAX_FILE_SIZE {
                    if !GLOBAL_CONFIG.y_ytdl.hight_audio_save {
                        let _ = std::fs::remove_file(nf.0);
                    }
                    let _ = std::fs::remove_file(nf_low.0);
                    return Err("低品质音频超过50MB，停止发送，删除低品质音频。".to_string());
                }
                let namefile_low = NamedFile {
                    file_name: nf_low.0.clone(),
                    file_data: nf_low.1,
                };
                // 如果低音质发送失败则发送一条消息提示
                if let Err(error) = bot
                    .send_audio(chat_id, namefile_low)
                    .disable_notification(true)
                    .send()
                    .await
                {
                    if !GLOBAL_CONFIG.y_ytdl.hight_audio_save {
                        let _ = std::fs::remove_file(nf.0);
                    }
                    let _ = tokio::fs::remove_file(nf_low.0).await;
                    return Err(format!("低品质音频发送失败。错误：{:#?}", error));
                } else {
                    let _ = tokio::fs::remove_file(nf.0).await;
                    let _ = tokio::fs::remove_file(nf_low.0).await;
                }
            }
        }
    } else {
        // 高品质音频超过50MB会发送失败，将尝试下载低品质音频
        let pathbuf_low = down_mp3(&url, VideoQuality::Lowest)
            .await
            .expect("下载低品质音频失败");
        // 构造发送音频参数
        let nf_low = read_audio(pathbuf_low).await;
        // 检测低品质音频是否超过50MB，如果超过则不发送，telegram发送限制50MB以下。超50MB需自建API服务器，难申请
        if nf_low.1.len() > MAX_FILE_SIZE {
            if !GLOBAL_CONFIG.y_ytdl.hight_audio_save {
                let _ = std::fs::remove_file(nf.0);
            }
            let _ = std::fs::remove_file(nf_low.0);
            return Err("低品质音频超过50MB，停止发送，删除低品质音频。".to_string());
        }
        let namefile_low = NamedFile {
            file_name: nf_low.0.clone(),
            file_data: nf_low.1,
        };
        // 如果发送失败则发送一条消息提示
        if let Err(error) = bot
            .send_audio(chat_id, namefile_low)
            .disable_notification(true)
            .send()
            .await
        {
            if !GLOBAL_CONFIG.y_ytdl.hight_audio_save {
                let _ = std::fs::remove_file(nf.0);
            }
            let _ = tokio::fs::remove_file(nf_low.0).await;
            return Err(format!("低品质音频发送失败。错误：{:#?}", error));
        } else {
            let _ = tokio::fs::remove_file(nf.0).await;
            let _ = tokio::fs::remove_file(nf_low.0).await;
        }
    }

    // 低品质音频发送失败时，高品质音频保存在当前目录,以供上传到TG群组中，使用tdl项目
    bot.delete_message(chat_id, msg.message_id)
        .send()
        .await
        .unwrap();
    Ok(())
}

// rusty_ytdl crate下载后缀为mp3，并不就一定是mp3格式的音频。也可能是webm
async fn down_mp3(url: &String, video_quality: VideoQuality) -> Result<PathBuf> {
    // 构建下载音频参数
    let video_options = if GLOBAL_CONFIG.y_ytdl.proxy.is_empty() {
        VideoOptions {
            quality: video_quality.clone(),
            filter: VideoSearchOptions::Audio,
            ..Default::default()
        }
    } else {
        let proxy = &GLOBAL_CONFIG.y_ytdl.proxy;
        if proxy.starts_with("socks5") && proxy.as_str().contains('@') {
            let err_msg = r#"
            reqwest库不支持带身份验证的socks5，请换成http/https (如果支持了请提交issuse告知我)
            如需使用socks5，需要不带身份验证的，比如:`socks5://1.2.3.4:1080`
            "#;
            return Err(anyhow!(err_msg));
            // return Err(err_msg);
        }
        VideoOptions {
            quality: video_quality.clone(),
            filter: VideoSearchOptions::Audio,
            request_options: RequestOptions {
                client: Some(
                    reqwest::ClientBuilder::new()
                        .use_rustls_tls()
                        .user_agent(
                            "Mozilla/5.0 (X11; Linux x86_64; rv:60.0) Gecko/20100101 Firefox/81.0",
                        )
                        .proxy(reqwest::Proxy::all(proxy).unwrap())
                        .build()
                        .unwrap(),
                ),
                ..Default::default()
            },
            ..Default::default()
        }
    };
    let audio = Video::new_with_options(url, video_options)?;
    // 获取链接标题,“/”在标题中会有转义问题，换成“-”
    let mut title = audio
        .get_info()
        .await?
        .video_details
        .title
        .replace(['/', '⧸'], "-");
    let mut chars: Vec<char> = title.chars().collect();
    // 某些链接标题过长会导致在Telegram发送时失败，进行截断
    if chars.len() > 30 {
        chars.truncate(30);
        title = chars.into_iter().collect();
    }

    // 如果是低质量音频则在文件名后添加_low标识
    let file_name = match video_quality {
        VideoQuality::Highest => format!("./{title}.mp3"),
        VideoQuality::Lowest => format!("./{title}_low.mp3"),
        _ => format!("./{title}.mp3"),
    };
    let file = std::path::PathBuf::from(&file_name);
    dbg!("下载前");
    audio.download(&file).await?;
    dbg!("下载后");
    Ok(file)
}

async fn read_audio(pf: PathBuf) -> (String, Vec<u8>) {
    let file = File::open(&pf).await.unwrap();
    let metadata = file.metadata().await.unwrap();
    let file_size = metadata.len() as usize;
    // 创建一个足够大的 buffer
    let mut buffer = Vec::with_capacity(file_size);
    // 读取整个文件内容
    // file.read_to_end(&mut buffer).await.unwrap();
    let mut reader = tokio::io::BufReader::new(file);
    reader.read_to_end(&mut buffer).await.unwrap();
    // 文件名，文件内容
    (
        pf.file_name().unwrap().to_str().unwrap().to_string(),
        buffer,
    )
}
