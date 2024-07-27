#!/bin/bash

# 停止脚本执行时发生错误
set -e

# 停止tgbot-app服务
echo "停止 tgbot-app 服务..."
sudo systemctl stop tgbot-app

# 命令行运行项目
echo "当前命令行下运行tgbot-app"
cargo run