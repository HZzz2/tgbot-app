use std::path::PathBuf;

use ferrisgram::input_file::NamedFile;
use ferrisgram::Bot;
use rusty_ytdl::Video;
use rusty_ytdl::VideoOptions;
use rusty_ytdl::VideoQuality;
use rusty_ytdl::VideoSearchOptions;
use tgbot_app::util::send_err_msg;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub async fn yt_audio(bot: &Bot, chat_id: i64, url: String) -> Result<(), String> {
    // 下载高质量音频格式文件
    let pathbuf =match down_m4a(&url, VideoQuality::Highest)
        .await{
            Ok(pf)=>pf,
            Err(e)=>{
                send_err_msg(bot.clone(), chat_id, format!("Error: {:#?}", e)).await;
                return Ok(());
            }
        };
        
    let nf = read_m4a(pathbuf).await;
    let namefile = NamedFile {
        file_name: nf.0,
        file_data: nf.1,
    };
    // 如果发送失败则下载低质量音频发送
    if let Err(_error) = bot.send_audio(chat_id, namefile).send().await {
        // let _ = std::fs::remove_file(file);
        // 高品质音频超过50MB会发送失败，将尝试下载低品质音频
        let pathbuf_low = down_m4a(&url, VideoQuality::Lowest)
            .await
            .expect("下载低品质音频失败");
        // 构造发送音频参数
        let nf_low = read_m4a(pathbuf_low).await;
        let namefile = NamedFile {
            file_name: nf_low.0.clone(),
            file_data: nf_low.1,
        };
        // 如果发送失败则发送一条消息提示
        if let Err(error) = bot.send_audio(chat_id, namefile).send().await {
            let _ = std::fs::remove_file(nf_low.0);
            return Err(format!(
                "低品质音频发送失败，高品质音频保存在工作目录下。错误：{:#?}",
                error
            ));
        } else {
            let _ = std::fs::remove_file(nf_low.0);
        }
    }
    // 低品质音频发送失败时，高品质音频保存在当前目录,以供上传到TG群组中，使用tdl项目

    Ok(())
}

async fn down_m4a(url: &String, video_quality: VideoQuality) -> Result<PathBuf, anyhow::Error> {
    // 构建下载音频参数
    let video_options = VideoOptions {
        quality: video_quality.clone(),
        filter: VideoSearchOptions::Audio,
        ..Default::default()
    };
    let audio = Video::new_with_options(url, video_options)?;
    // 获取链接标题
    let mut title = audio.get_info().await?.video_details.title;
    let mut chars: Vec<char> = title.chars().collect();
    // 某些链接标题过长会导致发送失败，进行截断
    if chars.len() > 30 {
        chars.truncate(30);
        title = chars.into_iter().collect();
    }

    // 如果是低质量音频则在文件名后缀添加_low标识
    let file_name = match video_quality {
        VideoQuality::Highest => format!("./{title}.m4a"),
        VideoQuality::Lowest => format!("./{title}_low.m4a"),
        _ => format!("./{title}.m4a"),
    };
    let file = std::path::PathBuf::from(&file_name);
    audio.download(&file).await?;
    Ok(file)
}

async fn read_m4a(pf: PathBuf) -> (String, Vec<u8>) {
    let mut file = File::open(&pf).await.unwrap();
    let metadata = file.metadata().await.unwrap();
    let file_size = metadata.len() as usize;
    // 创建一个足够大的 buffer
    let mut buffer = Vec::with_capacity(file_size);
    // 读取整个文件内容
    file.read_to_end(&mut buffer).await.unwrap();
    (pf.clone().to_str().unwrap().to_string(), buffer)
}
