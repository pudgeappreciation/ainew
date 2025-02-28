use serenity::all::{ChannelId, MessageId, UserId};

use super::DrawRequest;

#[derive(Debug)]
pub struct DbDrawRequest {
    pub state: String,
    pub options: String,
    pub original_options: String,
    pub user_id: i64,
    pub message_id: i64,
    pub channel_id: i64,
}

impl From<DbDrawRequest> for DrawRequest {
    fn from(value: DbDrawRequest) -> Self {
        DrawRequest {
            state: value.state,
            options: serde_json::from_str(&value.options).unwrap_or_default(),
            original_options: serde_json::from_str(&value.original_options).unwrap_or_default(),
            user_id: UserId::new(value.user_id as u64),
            message_id: MessageId::new(value.message_id as u64),
            channel_id: ChannelId::new(value.channel_id as u64),
        }
    }
}
