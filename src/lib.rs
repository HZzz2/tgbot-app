use std::{
    collections::{BTreeMap, HashMap, HashSet},
    sync::Arc,
};

use once_cell::sync::Lazy;
use serde::Deserialize;
pub mod util;
// 获取配置文件信息
pub static GLOBAL_CONFIG: Lazy<Arc<Config>> = Lazy::new(|| {
    // let config_str = include_str!("../config.toml");  // 编译期会将内容包含在可执行程序中
    let config_str = std::fs::read_to_string("./config.toml").expect("未找到config.toml");
    let config = toml::from_str::<Config>(&config_str).expect("Failed to parse config.toml");
    Arc::new(config)
});

// 反序列化配置信息
#[derive(Deserialize, Debug)]
pub struct Config {
    pub telegram: Telegram, // TG相关配置信息
    pub openai: Chatgpt,    // AI相关配置信息
    pub command: Command,   // 常用命令定制配置信息
    pub brute_force: BruteForce,
    pub yt_dlp: YtDlp,
    pub y_ytdl: YYtdl,
    pub resend: ReSend,
}

#[derive(Deserialize, Debug)]
pub struct Telegram {
    pub bot_token: String,
    pub ids: HashSet<i64>,
}

#[derive(Deserialize, Debug)]
pub struct Chatgpt {
    pub base_url: String,
    pub model: String,
    pub api_key: String,
}

#[derive(Deserialize, Debug)]
pub struct Command {
    pub cmd: BTreeMap<String, String>,
}

#[derive(Deserialize, Debug)]
pub struct BruteForce {
    pub ssh: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
pub struct YtDlp {
    pub proxy: String,
}

#[derive(Deserialize, Debug)]
pub struct YYtdl {
    pub proxy: String,
}

#[derive(Deserialize, Debug)]
pub struct ReSend {
    pub api_key: String,
    pub from: String
}