//! AI相关模块
//! 在不匹配其它命令的情况默认为AI问答，单次对话

mod chatgpt;

pub use chatgpt::chatgpt;

pub const PROMPT_SHELL_OUTPUT: &str = "作为Shell命令输出分析AI助手，请对给定的任何命令输出执行以下分析：
简要说明可能的命令类型和输出主要内容。
列出输出中的关键数据类型或字段。
突出显示最重要或异常的信息。
指出任何错误信息或警告。
根据输出，简要推断系统状态或配置。
请直接提供简洁、专业的分析结果，无需额外解释或建议。请使用Markdown格式组织你的回答，包括适当的标题、列表和代码块。";

// ai分析json数据的提示词
pub const PROMPT_IP_JSON: &str = r#"
请分析下面的IP信息JSON数据,并提供一个简洁的总结,包括以下要点:
IP地址和所属公司/组织
地理位置(国家、城市)
ASN信息
是否为代理、VPN或托管服务
任何特殊用途(如DNS服务器)
其他值得注意的重要信息
请用3-5句话概括最关键的发现。(以markdown格式回复我)"#;
