#!/bin/bash

# 停止脚本执行时发生错误
set -e

# 编译项目
echo "构建项目..."
cargo build -r

# 停止服务
echo "停止 tgbot-app 服务..."
sudo systemctl stop tgbot-app

# 复制构建输出文件
echo "复制 tgbot-app 可执行文件到 /root/tgbot-app/..."
sudo cp target/release/tgbot-app /root/tgbot-app/

# 设定可执行权限
echo "设置 tgbot-app 可执行权限..."
sudo chmod +x /root/tgbot-app/tgbot-app

# 复制配置文件
echo "复制配置文件到 /root/tgbot-app/..."
sudo cp config.toml /root/tgbot-app/

# 重新加载 systemd
echo "重新加载 systemd 守护进程..."
sudo systemctl daemon-reload

# 重启服务
echo "重启 tgbot-app 服务..."
sudo systemctl restart tgbot-app

echo "操作完成。"