use ferrisgram::{error::GroupIteration, ext::Context, Bot};
use tklog::{async_error, async_info};
use tokio::{fs::File, io::BufReader, process::Command};

use ferrisgram::error::Result;

const SHA1_HEX_STRING_LENGTH: usize = 40;

/// sha1哈希爆破
/// ```txt
/// /sha1 hash                or
/// /sha1 hash 本地字典文件    or
/// /sha1 hash 在线字典地址
/// ```
pub async fn sha1_cracker(bot: Bot, ctx: Context) -> Result<GroupIteration> {
    // Same logic as chat applies on unwrapping effective message here.
    let msg = ctx.effective_message.unwrap();
    let chat_id = msg.chat.id;

    let text = msg.text.unwrap();
    if text.len() == 5 {
        let usage = r#"
使用方式：
```txt
/sha1 hash                // 默认本地sha1字典，config中配置
/sha1 hash 本地字典文件    // 指定本地字典
/sha1 hash 在线字典地址    // 在线字典地址URL
```
        "#;
        bot.send_message(chat_id, usage.to_string())
            .parse_mode("markdown".to_string())
            .send()
            .await
            .unwrap();
        return Ok(GroupIteration::EndGroups);
    }
    let text = text[6..].trim();
    let input: Vec<&str> = text.split(' ').collect();

    match input.len() {
        1 => sha1_brute(input[0], bot, chat_id).await,
        2 => sha1_brute_file(input[0], input[1], bot, chat_id).await,
        _ => {
            let usage = r#"
使用方式：
```txt
/sha1 hash                // 默认本地sha1字典，config中配置
/sha1 hash 本地字典文件    // 指定本地字典
/sha1 hash 在线字典地址    // 在线字典地址URL
```
        "#;
            bot.send_message(chat_id, usage.to_string())
                .parse_mode("markdown".to_string())
                .send()
                .await
                .unwrap();
            return Ok(GroupIteration::EndGroups);
        }
    }

    // bot.send_message(chat_id, input.join("-"))
    //     .send()
    //     .await
    //     .unwrap();

    Ok(GroupIteration::EndGroups)
}

async fn sha1_brute(hash: &str, bot: Bot, chat_id: i64) {
    if hash.len() != SHA1_HEX_STRING_LENGTH {
        let _ = bot
            .send_message(chat_id, "请输入正确的sha1哈希(40位)".to_string())
            .send()
            .await;
        return;
    }
    // todo 暂且先这个字典文件，实际为配置定义的文件名 配置还没加
    let wordlist_file = File::open("./wordlist.txt").await.unwrap();
    let reader = BufReader::new(wordlist_file);
}

async fn sha1_brute_file(hash: &str, file: &str, bot: Bot, chat_id: i64) {
    if hash.len() != SHA1_HEX_STRING_LENGTH {
        let _ = bot
            .send_message(chat_id, "请输入正确的sha1哈希(40位)".to_string())
            .send()
            .await;
        return;
    }
    // todo 可以是本地文件或在线字典，如果是在线字典则下载到本地使用
    let wordlist_file = File::open(file).await.unwrap();
    let reader = BufReader::new(wordlist_file);

}
