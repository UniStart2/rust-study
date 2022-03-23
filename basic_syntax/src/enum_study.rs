// 定义 消息的结构
#[derive(Debug)]
pub struct Message {
    from_account_id: u32,  // 发送消息的账号id
    to_account_id: u32,    // 接收消息的账号id
    content: String,       // 消息内容
    status: MessageStatus, // 消息的状态
}

impl Message {
    pub fn new(from_account_id: u32, to_account_id: u32, content: String) -> Message {
        Message {
            from_account_id,
            to_account_id,
            content,
            status: MessageStatus::created,
        }
    }
}

#[derive(Debug)]
pub enum MessageStatus {
    created,  // 已创建
    deleted,  // 已删除
    send,     // 已发送
    recived,  // 已接收
    read,     // 已读·
    unread,   // 未读
    withdraw, // 撤回
}
