# 🤖 Telegram Bot - 多功能助手

这是一个用Rust语言编写的多功能Telegram机器人，提供了丰富的实用命令和功能。程序设计为以Linux服务的方式运行，并在出错时自动重启，确保稳定可靠的服务。🚀

## ✨ 功能特点

- 🔧 执行自定义命令
- 🖥️ 执行Shell命令
- 🌐 IP和DNS查询
- 🤖 与AI单次对话
- 📂 文件系统操作
- 🌍 网络连接测试
- 📥 文件下载（调用aria2c和yt-dlp）
- 🔑 SSH暴力枚举（用于密码找回）

## 🛠️ 可用命令

- `/c {key}` - 执行键 *key* 对应的自定义命令。如果 *key* 不存在，则显示所有自定义命令 📜
- `/shell {命令}` - 执行任意 Shell 命令并返回标准输出和标准错误信息 🖥️
- `/shell_no_output {命令}` - 执行任意 Shell 命令，返回命令是否执行成功而不返回相关输出 🔇
- `/ip {ip地址}` - 查询 IP 相关信息 🌍
- `/dns {ip地址}` - 查询 DNS 相关信息 🌐
- `/chatgpt {消息}` - 与 AI 进行对话 或直接发送{消息} 单次对话 🤖
- `/ls` - 显示当前目录下的所有文件 📂（不支持显示指定目录。如有需要，使用 `/shell ls xxx`）
- `/ping {example.com}` - 检测与另一主机之间的网络连接。默认发送4个数据包 🏓
- `/aria2c {链接}` - 使用aria2c下载受aria2c支持的文件到aria2c_download文件夹下 📥
- `/ytdlp {视频链接}` - 使用yt-dlp下载最佳音视频到当前目录下 🎥
- `/ssh_brute {ip地址}` - ssh登录密码找回 🔐

📝 **特别说明：**
- 发送非命令消息默认与 AI 进行单次对话 💬 例如发送：`红烧鱼怎么做？` 🍲
- 发送油管链接默认下载音频 🎵（工作目录下需要 [yt-dlp](https://github.com/yt-dlp/yt-dlp?tab=readme-ov-file#release-files) ）

## 📦 安装编译步骤（提供已编译好的Debian和FreeBSD版本）

1. 确保您的系统中已安装Rust和Cargo。🦀
   ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
   ```
   apt install pkg-config libssl-dev -y
   ```
2. 克隆此仓库：
   ```
   git clone https://github.com/HZzz2/tgbot-app
   ```
3. 进入项目目录：
   ```
   cd tgbot-app
   ```
4. 编译项目：
   ```
   cargo build --release
   ```

## ⚙️ 配置与部署

1. 在项目根目录将`config-template.toml`更名为`config.toml`。📄
   ```
   cp config-template.toml config.toml
   ```

2. 在`config.toml`文件中添加您的Telegram Bot Token和其他必要的配置。🔑

3. 创建服务运行目录：
   ```
   sudo mkdir -p /root/tgbot-app
   ```

4. 将编译好的二进制文件和配置文件复制到服务目录：
   ```
   sudo cp target/release/tgbot-app /root/tgbot-app/
   sudo cp config.toml /root/tgbot-app/
   ```

5. 确保文件权限正确：
   ```
   sudo chmod +x /root/tgbot-app/tgbot-app
   sudo chmod 644 /root/tgbot-app/config.toml
   ```


## 🚀 部署为Linux服务

1. 创建一个系统服务文件，例如`/etc/systemd/system/tgbot-app.service`：

   ```
   [Unit]
   Description=tgbot-app
   After=network.target

   [Service]
   ExecStart=/root/tgbot-app/tgbot-app
   WorkingDirectory=/root/tgbot-app
   Restart=always

   [Install]
   WantedBy=default.target
   ```

2. 替换上面的路径为您系统上的实际值。📍

3. 重新加载systemd配置：
   ```
   sudo systemctl daemon-reload
   ```

4. 启动服务：
   ```
   sudo systemctl start tgbot-app
   ```

5. 设置开机自启：
   ```
   sudo systemctl enable tgbot-app
   ```

## 💡 使用示例

- 执行自定义命令：`/c 1` 🔢
- 执行Shell命令：`/shell ls -la` 📁
- 与AI对话：`/chatgpt 你好，请介绍一下自己` 🤖
- 下载YouTube视频：`/ytdlp https://www.youtube.com/watch?v=dQw4w9WgXcQ` 🎵

## ⚠️ 注意事项

- 使用`/shell`和`/ssh_brute`命令时请谨慎，确保您有权限执行这些操作。🛡️
- 下载文件时请遵守版权法律。📜
- 使用AI对话功能时，请注意可能产生的API使用费用。💰

## 📊 日志和监控

查看服务状态：
```
sudo systemctl status tgbot-app
```

查看服务日志：
```
sudo journalctl -u tgbot-app
```

使用常见的Linux监控工具（如systemctl、top等）来监控程序的资源使用情况。📈

## 🤝 贡献

欢迎提交问题和拉取请求。对于重大更改，请先开issue讨论您想要改变的内容。👥

## 📄 许可证

[MIT](https://choosealicense.com/licenses/mit/)

---

🌟 如果您觉得这个项目有用，请给它一个star！非常感谢您的支持！ 🙏
