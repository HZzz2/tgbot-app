use ferrisgram::{error::GroupIteration, ext::Context, Bot};
use reqwest;
use serde_json::Value;
use tgbot_app::util::{ai_q_s, send_err_msg, verify_telegram};

use ferrisgram::error::Result;

pub async fn ip(bot: Bot, ctx: Context) -> Result<GroupIteration> {
    // Same logic as chat applies on unwrapping effective message here.
    let msg = ctx.effective_message.unwrap();
    let chat_id = msg.chat.id;
    if !verify_telegram(chat_id) {
        return Ok(GroupIteration::EndGroups);
    }
    let cm = msg.text.unwrap();
    let ip = if cm.starts_with('/') {
        cm[4..].trim()
    } else {
        cm[..].trim()
    };
    let ipinfo_json_data: Value =
        match reqwest::get(format!(r"https://ipinfo.io/widget/demo/{}", ip))
            .await
            .unwrap()
            .json()
            .await
        {
            Ok(v) => v,
            Err(e) => {
                let e_msg = format!("获取IP-json数据失败，抓的接口，不能频繁查询：{:?}", e);
                let _ = send_err_msg(bot, chat_id, e_msg).await;
                return Ok(GroupIteration::EndGroups);
            }
        };

    let jdata = serde_json::to_string_pretty(&ipinfo_json_data).unwrap();
    bot.send_message(
        chat_id,
        format!("ipinfo接口，不要频繁调用{}", jdata.clone()),
    )
    .send()
    .await?;
    if let Some(loc_str) = ipinfo_json_data["data"]["loc"].as_str() {
        let loc: Vec<f64> = loc_str
            .split(',')
            .map(|x| x.parse::<f64>().unwrap())
            .collect();
        bot.send_location(chat_id, loc[0], loc[1])
            .send()
            .await
            .unwrap();
    }
    // ai总结json数据的提示词
    let prompt = r#"
请分析下面的IP信息JSON数据,并提供一个简洁的总结,包括以下要点:
IP地址和所属公司/组织
地理位置(国家、城市)
ASN信息
是否为代理、VPN或托管服务
任何特殊用途(如DNS服务器)
其他值得注意的重要信息
请用3-5句话概括最关键的发现。(以markdown格式回复我)
    "#;
    let ai_result = ai_q_s(format!("{}：{}", prompt, jdata)).await;
    if let Ok(ai_answer) = ai_result {
        let _ = bot
            .send_message(chat_id, format!("AI总结：\n{}", ai_answer))
            .parse_mode("markdown".to_string())
            .send()
            .await;
    }

    Ok(GroupIteration::EndGroups)
}
