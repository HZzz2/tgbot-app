//! 与下载有关，下载视频或音频
//! aria2c yt-dlp

mod aria2c;
// mod y_ytdl;
mod yt_dlp;
mod yt_dlp_audio;

pub use aria2c::aria2c;
// pub use y_ytdl::yt_audio;
pub use yt_dlp::ytdlp;
pub use yt_dlp_audio::ytdlp_audio;
