use ferrisgram::error::GroupIteration;
use ferrisgram::error::Result;
use ferrisgram::ext::Context;
use ferrisgram::Bot;
use resend_rs::types::CreateEmailBaseOptions;
use resend_rs::Resend;
use tgbot_app::util::send_err_msg;
use tgbot_app::GLOBAL_CONFIG;

pub async fn resend(bot: Bot, ctx: Context) -> Result<GroupIteration> {
    let msg = ctx.effective_message.unwrap();
    let chat_id = msg.chat.id;
    tgbot_app::verify_telegram_id!(chat_id);

    let api_key = &GLOBAL_CONFIG.resend.api_key;
    let from = &GLOBAL_CONFIG.resend.from;
    if api_key.is_empty() || from.is_empty() {
        //todo err msg send
        let msg = r#"
        如需使用[resend](https://resend.com)需要添加api_key和发送邮箱地址(from)
        在网站上申请密钥和验证邮箱后填入配置文件中并重启程序服务，如`systemctl restart tgbot-app`
        使用方式：`/resend 接收邮箱地址###邮件标题###邮件正文`
        比如：`/resend abc@efg.com###全民制作人们大家好###我喜欢唱跳Rap篮球，CTRL!!!`
        "#;
        send_err_msg(bot, chat_id, msg.to_string()).await;
        return Ok(GroupIteration::EndGroups);
    }

    let resend = Resend::new(api_key);

    let all_content = msg.text.unwrap();
    let use_content: Vec<&str> = all_content[8..].split("###").map(|c| c.trim()).collect();
    if use_content.len() != 3 {
        let msg = r#"
        使用方式：`/resend 接收邮箱地址###邮件标题###邮件正文`
        比如：`/resend abc@efg.com###全民制作人们大家好###我喜欢唱跳Rap篮球，CTRL!!!`
        "#;
        send_err_msg(bot, chat_id, msg.to_string()).await;
        return Ok(GroupIteration::EndGroups);
    }
    let to = [use_content[0]];
    let subject = use_content[1];

    // let filename = "invoice.pdf";
    // let mut f = File::open(filename).await.unwrap();
    // let mut invoice = Vec::new();
    // f.read_to_end(&mut invoice).await.unwrap();

    let email = CreateEmailBaseOptions::new(from, to, subject).with_text(use_content[2]);
    // .with_attachment(Attachment::from_content(invoice).with_filename(filename))
    // .with_header("X-Entity-Ref-ID", "123456789")
    // .with_tag(Tag::new("category", "confirm_email"));

    // let _email = resend.emails.send(email).await.unwrap();
    match resend.emails.send(email).await {
        Ok(_) => {
            let _ = bot
                .send_message(chat_id, "邮件发送成功".to_string())
                .send()
                .await;
        }
        Err(e) => {
            let _ = bot
                .send_message(chat_id, format!("邮件发送失败:{:#?}", e))
                .send()
                .await;
        }
    }

    Ok(GroupIteration::EndGroups)
}
