use ferrisgram::error::Result;
use ferrisgram::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use ferrisgram::{error::GroupIteration, ext::Context, Bot};
use tgbot_app::util::{chunks_msg, verify_telegram};
use tgbot_app::MESSAGE_LEN;
use tokio::process::Command;

pub async fn dns(bot: Bot, ctx: Context) -> Result<GroupIteration> {
    // Same logic as chat applies on unwrapping effective message here.
    let msg = ctx.effective_message.unwrap();
    let chat_id = msg.chat.id;
    if !verify_telegram(chat_id) {
        return Ok(GroupIteration::EndGroups);
    }
    let cm = msg.text.unwrap();
    let d = if cm.starts_with('/') {
        cm[5..].trim()
    } else {
        cm[..].trim()
    };

    // 回调  web待加
    let button_dnsrecon = InlineKeyboardButton::callback_button(
        "dnsrecon",
        format!("osint dns cb_dnsrecon {}", d).as_str(),
    );
    let button_dnsenum = InlineKeyboardButton::callback_button(
        "dnsenum",
        format!("osint dns cb_dnsenum {}", d).as_str(),
    );
    let button_dnschecker: InlineKeyboardButton = InlineKeyboardButton::url_button(
        "dnschecker-web",
        format!("https://dnschecker.org/#A/{}", d).as_str(),
    );

    // let button_google = InlineKeyboardButton::url_button("Google", "https://google.com");

    // let button_baidu = InlineKeyboardButton::url_button("百度", "https://baidu.com");

    // dig命令查询
    let dig_output = Command::new("dig")
        .arg(d)
        .output()
        .await
        .expect("dns命令执行失败")
        .stdout;

    bot.send_message(
        chat_id,
        format!("dig:{}", String::from_utf8_lossy(&dig_output)),
    )
    .send()
    .await?;

    // nslookup查询
    let nslookup_output = Command::new("nslookup")
        .arg(d)
        .output()
        .await
        .expect("nslookup命令执行失败")
        .stdout;

    // nslookup查询结果后提供回调和web todo
    bot.send_message(
        chat_id,
        format!("nslookup:{}", String::from_utf8_lossy(&nslookup_output)),
    )
    .reply_markup(InlineKeyboardMarkup::new(vec![vec![
        button_dnsrecon,
        button_dnsenum,
        button_dnschecker,
    ]]))
    .send()
    .await?;

    Ok(GroupIteration::EndGroups)
}

pub async fn cb_dnsenum(arg: &str, bot: Bot, chat_id: i64) -> Result<GroupIteration> {
    let dnsenum_output = Command::new("dnsenum")
        .args(["--reserver", arg]) // --reserver进行反向解析 加快扫描速度。保存文件的话可以不加此参数？
        .output()
        .await
        .expect("dnsenum命令执行失败")
        .stdout;

    if dnsenum_output.len() > MESSAGE_LEN {
        let _ = chunks_msg(&bot, chat_id, String::from_utf8_lossy(&dnsenum_output)).await;
        return Ok(GroupIteration::EndGroups);
    }
    let button_ai = InlineKeyboardButton::callback_button("AI分析", "AI分析 PROMPT_SHELL_OUTPUT");

    bot.send_message(
        chat_id,
        format!("dnsenum:{}", String::from_utf8_lossy(&dnsenum_output)),
    )
    .reply_markup(InlineKeyboardMarkup::new(vec![vec![button_ai]]))
    .send()
    .await?;

    Ok(GroupIteration::EndGroups)
}

pub async fn cb_dnsrecon(arg: &str, bot: Bot, chat_id: i64) -> Result<GroupIteration> {
    let dnsrecon_output = Command::new("dnsrecon")
        .args(["-d", arg])
        .output()
        .await
        .expect("dnsrecon命令执行失败")
        .stdout;

    if dnsrecon_output.len() > MESSAGE_LEN {
        let _ = chunks_msg(&bot, chat_id, String::from_utf8_lossy(&dnsrecon_output)).await;
        return Ok(GroupIteration::EndGroups);
    }

    let button_ai: InlineKeyboardButton =
        InlineKeyboardButton::callback_button("AI分析", "AI分析 PROMPT_SHELL_OUTPUT");

    bot.send_message(
        chat_id,
        format!("dnsrecon:{}", String::from_utf8_lossy(&dnsrecon_output)),
    )
    .reply_markup(InlineKeyboardMarkup::new(vec![vec![button_ai]]))
    .send()
    .await?;

    Ok(GroupIteration::EndGroups)
}
