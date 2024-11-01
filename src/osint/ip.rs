use crate::ai::PROMPT_IP_JSON;
use crate::util::REQWEST_CLIENT;
use crate::util::{ai_q_s, send_err_msg};

use ferrisgram::error::Result;
use ferrisgram::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use ferrisgram::{error::GroupIteration, ext::Context, Bot};
use reqwest;
use serde_json::Value;
use tokio::process::Command;

pub async fn ip(bot: Bot, ctx: Context) -> Result<GroupIteration> {
    // Same logic as chat applies on unwrapping effective message here.
    let msg = ctx.effective_message.unwrap();
    let chat_id = msg.chat.id;
    crate::verify_telegram_id!(chat_id);
    let cm = msg.text.unwrap();
    let ip = if cm.starts_with('/') {
        // 如果没有提供参数则获取本机IPV4
        if cm[..].trim() == "/ip" {
            let local_ip = Command::new("curl")
                .arg("ipv4.ip.sb")
                .output()
                .await
                .unwrap()
                .stdout;
            let output = String::from_utf8_lossy(&local_ip);
            output.trim().to_string()
        } else {
            // 使用提供的参数
            cm[4..].trim().to_string()
        }
    } else {
        //从本程序传来的IP查询
        cm[..].trim().to_string()
    };

    let ip_json_data: Value = match reqwest::get(format!(r"https://ipinfo.io/widget/demo/{}", ip))
        .await
        .unwrap()
        .json()
        .await
    {
        Ok(v) => v,
        Err(_e) => {
            match reqwest::get(format!(r"https://api.seeip.org/geoip/{}", ip))
                .await
                .unwrap()
                .json()
                .await
            {
                Ok(v) => v,
                Err(e) => {
                    let e_msg = format!("获取IP-json数据失败(seeip.org)：{:?}", e);
                    let _ = send_err_msg(bot, chat_id, e_msg).await;
                    return Ok(GroupIteration::EndGroups);
                }
            }
        }
    };

    let jdata = serde_json::to_string_pretty(&ip_json_data).unwrap();

    // 回调和web访问
    let button_ping0 = InlineKeyboardButton::url_button(
        "ping0-web",
        format!("https://ip.ping0.cc/ip/{}", ip).as_str(),
    );
    let button_ipx =
        InlineKeyboardButton::url_button("ipx-web", format!("https://ipx.ac/{}", ip).as_str());
    let button_ai: InlineKeyboardButton =
        InlineKeyboardButton::callback_button("AI分析", "AI分析 分析关于IP的JSON数据");
    let button_ip123: InlineKeyboardButton = InlineKeyboardButton::callback_button(
        "ip123",
        format!("osint ip cb_ip123 {}", ip).as_str(),
    );

    // 发送IP信息并提供回调和web访问
    bot.send_message(chat_id, format!("ip数据-json格式：{}", jdata.clone()))
        .reply_markup(InlineKeyboardMarkup::new(vec![vec![
            button_ip123,
            button_ping0,
            button_ipx,
            button_ai,
        ]]))
        // .parse_mode("markdown".to_string())
        .send()
        .await?;
    // 发送地理位置
    if let Some(loc_str) = ip_json_data["data"]["loc"].as_str() {
        let loc: Vec<f64> = loc_str
            .split(',')
            .map(|x| x.parse::<f64>().unwrap())
            .collect();
        bot.send_location(chat_id, loc[0], loc[1])
            .send()
            .await
            .unwrap();
    } else {
        let lat = ip_json_data["latitude"].as_str();
        let lon = ip_json_data["longitude"].as_str();
        if let (Some(la), Some(lo)) = (lat, lon) {
            bot.send_location(
                chat_id,
                la.parse::<f64>().unwrap(),
                lo.parse::<f64>().unwrap(),
            )
            .send()
            .await
            .unwrap();
        }
    };
    // AI分析json数据并总结
    let ai_result = ai_q_s(format!("{}：{}", PROMPT_IP_JSON, jdata)).await;
    if let Ok(ai_answer) = ai_result {
        let _ = bot
            .send_message(chat_id, ai_answer)
            .parse_mode("markdown".to_string())
            .send()
            .await;
    }

    Ok(GroupIteration::EndGroups)
}

pub async fn cb_ip123(arg: &str, bot: Bot, chat_id: i64) -> Result<GroupIteration> {
    let ip123_output: Value = REQWEST_CLIENT
        .get(format!("https://ip234.in/fraud_check?ip={}", arg))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    // let button_ai: InlineKeyboardButton =
    //     InlineKeyboardButton::callback_button("AI分析", "AI分析 PROMPT_SHELL_OUTPUT");

    bot.send_message(
        chat_id,
        format!(
            "ip123:{}",
            serde_json::to_string_pretty(&ip123_output).unwrap()
        ),
    )
    // .reply_markup(InlineKeyboardMarkup::new(vec![vec![button_ai]]))
    .send()
    .await?;

    Ok(GroupIteration::EndGroups)
}
