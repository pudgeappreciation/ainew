pub mod api_options;
pub mod options;

use std::time::{SystemTime, UNIX_EPOCH};

use options::Options;
use serde::{Deserialize, Serialize};
use serenity::all::{ChannelId, CommandInteraction, MessageId, UserId};
use sqlx::{Pool, Sqlite};

#[derive(Debug, Serialize, Deserialize)]
pub struct DrawRequest {
    pub state: String,
    pub options: Options,
    pub user_id: UserId,
    pub request_id: MessageId,
    pub channel_id: ChannelId,
}

impl DrawRequest {
    pub fn new_from_command(command: &CommandInteraction, request_id: MessageId) -> DrawRequest {
        DrawRequest {
            state: String::from("queued"),
            options: Options::new_from_command(command),
            user_id: command.user.id,
            request_id,
            channel_id: command.channel_id,
        }
    }

    pub async fn save(&self, database: &Pool<Sqlite>) -> Result<(), ()> {
        let user_id = self.user_id.get() as i64;
        let request_id = self.request_id.get() as i64;
        let channel_id = self.channel_id.get() as i64;

        let Ok(created_at) = SystemTime::now().duration_since(UNIX_EPOCH) else {
            return Err(());
        };
        let created_at = created_at.as_secs() as i64;

        let options = serde_json::to_string(&self.options).map_err(|_| ())?;

        let result = sqlx::query!(
            r#"
            INSERT INTO `draw_requests` (
                `state`,
                `options`,
                `user_id`,
                `request_id`,
                `channel_id`,
                `created_at`
            )
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
            self.state,
            options,
            user_id,
            request_id,
            channel_id,
            created_at,
        )
        .execute(database)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(why) => {
                println!("Cannot respond to slash command: {why}");

                Err(())
            }
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
