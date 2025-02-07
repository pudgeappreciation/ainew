use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use serenity::all::{
    ChannelId, CommandInteraction, MessageId, ResolvedOption, ResolvedValue, UserId,
};
use sqlx::{Pool, Sqlite};

#[derive(Debug, Serialize, Deserialize)]
pub struct DrawRequest {
    pub state: String,
    pub prompt: String,
    pub negative_prompt: String,
    pub steps: u8,
    pub user_id: UserId,
    pub request_id: MessageId,
    pub channel_id: ChannelId,
}

impl DrawRequest {
    pub fn new_from_command(command: &CommandInteraction, request_id: MessageId) -> DrawRequest {
        let mut request = DrawRequest {
            state: "queued".to_string(),
            prompt: "".to_string(),
            negative_prompt: "".to_string(),
            steps: 20,
            user_id: command.user.id,
            request_id,
            channel_id: command.channel_id,
        };

        for option in command.data.options().iter() {
            match option {
                ResolvedOption {
                    value: ResolvedValue::String(value),
                    name: "prompt",
                    ..
                } => request.prompt = value.to_string(),
                ResolvedOption {
                    value: ResolvedValue::String(value),
                    name: "negative_prompt",
                    ..
                } => request.negative_prompt = value.to_string(),
                ResolvedOption {
                    value: ResolvedValue::Integer(value),
                    name: "steps",
                    ..
                } => request.steps = *value as u8,
                _ => {}
            }
        }

        request
    }

    pub async fn save(&self, database: &Pool<Sqlite>) -> Result<(), ()> {
        let user_id = self.user_id.get() as i64;
        let request_id = self.request_id.get() as i64;
        let channel_id = self.channel_id.get() as i64;

        let Ok(created_at) = SystemTime::now().duration_since(UNIX_EPOCH) else {
            return Err(());
        };
        let created_at = created_at.as_secs() as i64;

        let result = sqlx::query!(
            r#"
            INSERT INTO `draw_requests` (
                `state`,
                `prompt`,
                `negative_prompt`,
                `steps`,
                `user_id`,
                `request_id`,
                `channel_id`,
                `created_at`
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            "queued",
            self.prompt,
            self.negative_prompt,
            self.steps,
            user_id,
            request_id,
            channel_id,
            created_at,
        )
        .execute(database)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    pub async fn drawing(&self, database: &Pool<Sqlite>) -> Result<(), ()> {
        let request_id = self.request_id.get() as i64;
        let result = sqlx::query!(
            r#"
            UPDATE `draw_requests`
            SET `state` = 'drawing'
            WHERE `request_id` = ?
            "#,
            request_id,
        )
        .execute(database)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    pub async fn complete(&self, database: &Pool<Sqlite>) -> Result<(), ()> {
        let request_id = self.request_id.get() as i64;
        let result = sqlx::query!(
            r#"
            UPDATE `draw_requests`
            SET `state` = 'complete'
            WHERE `request_id` = ?
            "#,
            request_id,
        )
        .execute(database)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }
}
