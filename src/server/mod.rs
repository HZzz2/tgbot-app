//! 此模块为第三方API服务的集合
//! resend:通过调用api来发送邮件

mod resend;

pub use resend::resend;
