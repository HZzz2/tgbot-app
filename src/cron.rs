use std::sync::Arc;

use ferrisgram::Bot;
use serde_json::Value;
use tgbot_app::{util::REQWEST_CLIENT, GLOBAL_CONFIG};

pub async fn tianqi(bot: Arc<Bot>) {
    let tianqi:Value = REQWEST_CLIENT.get("https://cn.apihz.cn/api/tianqi/tqyb.php?id=88888888&key=88888888&sheng=湖南&place=长沙")
    .send().await.unwrap().json().await.unwrap();
    // 公共id key容易失败
    if tianqi["place"] == "null" {
        return;
    }
    let str_format = format!(
        "
    *☀️ 天气预报 ☀️*

    🏙️ *地区*: {}
    🌡️ *温度*: {}°C
    💧 *湿度*: {}%
    🌬️ *风速*: {}m/s
    🍃 *风力等级*: {}
    ",
        tianqi["place"],
        tianqi["temperature"],
        tianqi["humidity"],
        tianqi["windSpeed"],
        tianqi["windScale"]
    );
    let _ = bot
        .send_message(GLOBAL_CONFIG.telegram.ids[0], str_format)
        .parse_mode(String::from("markdown"))
        .send()
        .await;
}
