//! 和Linux shell相关的模块，用来执行shell命令，比如：ls, ping, 机器人接收传入的任意shell命令或定制的通用命令
//! any_shell_command:执行任意shell命令并返回结果，包括标准输出和标准错误
//! shell_no_output:执行任意shell命令并返回是否执行成功
//! ls , ping ...

mod any_shell_command;
mod command;
mod ls;
mod ping;
mod shell_no_output;

pub use any_shell_command::shell;
pub use command::c;
pub use ls::ls;
pub use ping::ping;
pub use shell_no_output::shell_no_output;
