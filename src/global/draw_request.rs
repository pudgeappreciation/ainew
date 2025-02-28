pub mod api_options;
mod db;
pub mod options;

use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Utc};
use options::Options;
use serde::{Deserialize, Serialize};
use serenity::all::{
    ChannelId, CommandInteraction, CreateEmbed, MessageBuilder, MessageId, UserId,
};
use sqlx::{Pool, Sqlite};

use super::draw_profile::DrawProfile;

pub use db::DbDrawRequest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrawRequest {
    pub state: String,
    pub options: Options,
    pub original_options: Options,
    pub user_id: UserId,
    pub message_id: MessageId,
    pub channel_id: ChannelId,
    pub created_at: DateTime<Utc>,
}

impl From<DrawRequest> for CreateEmbed {
    fn from(value: DrawRequest) -> Self {
        let mut content = MessageBuilder::new();

        content
            .push_bold_safe("Request made at: ")
            .push_line_safe(value.created_at.format("%F %R").to_string())
            .push_line_safe("");
        value.options.embed(&mut content);

        CreateEmbed::new().title("").description(content.build())
    }
}

impl DrawRequest {
    pub fn new_from_command(
        command: &CommandInteraction,
        message_id: MessageId,
        profile: Option<DrawProfile>,
    ) -> DrawRequest {
        let created_at = Utc::now();

        DrawRequest {
            state: String::from("queued"),
            options: Options::new_from_command(command, profile),
            original_options: Options::new_from_command(command, None),
            user_id: command.user.id,
            message_id,
            channel_id: command.channel_id,
            created_at,
        }
    }

    pub fn to_command_options(&self) -> String {
        let mut command = String::new();

        command.push_str(&self.options.to_command_options());

        command
    }

    pub async fn save(&self, database: &Pool<Sqlite>) -> Result<(), ()> {
        let user_id = self.user_id.get() as i64;
        let message_id = self.message_id.get() as i64;
        let channel_id = self.channel_id.get() as i64;

        let Ok(created_at) = SystemTime::now().duration_since(UNIX_EPOCH) else {
            return Err(());
        };
        let created_at = created_at.as_secs() as i64;

        let options = serde_json::to_string(&self.options).map_err(|_| ())?;
        let original_options = serde_json::to_string(&self.original_options).map_err(|_| ())?;

        let result = sqlx::query!(
            r#"
            INSERT INTO `draw_requests` (
                `state`,
                `options`,
                `original_options`,
                `user_id`,
                `message_id`,
                `channel_id`,
                `created_at`
            )
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
            self.state,
            options,
            original_options,
            user_id,
            message_id,
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

    pub async fn get_from_message_id(
        message_id: MessageId,
        database: &Pool<Sqlite>,
    ) -> Result<Option<Self>, ()> {
        let message_id = message_id.get() as i64;

        let result = sqlx::query_as!(
            DbDrawRequest,
            r#"
            SELECT
                `state`,
                `options`,
                `original_options`,
                `user_id`,
                `message_id`,
                `channel_id`,
                `created_at`
            FROM
                `draw_requests`
            WHERE
                `message_id` = ?
            "#,
            message_id,
        )
        .fetch_optional(database)
        .await;

        match result {
            Ok(Some(draw_request)) => Ok(DrawRequest::try_from(draw_request).ok()),
            Err(why) => {
                println!("Cannot respond to slash command: {why}");

                Err(())
            }
            _ => Err(()),
        }
    }

    pub async fn popular_models(database: &Pool<Sqlite>) -> Vec<String> {
        let Ok(now) = SystemTime::now().duration_since(UNIX_EPOCH) else {
            return Vec::new();
        };
        let last_week = (now.as_secs() - (60 * 60 * 24 * 7)) as i64;

        sqlx::query_scalar!(
            "
            SELECT JSON_EXTRACT(`draw_requests`.`options`, '$.model') AS `models: Option<String>`
            FROM `draw_requests`
            WHERE `draw_requests`.`created_at` > ?
            GROUP BY JSON_EXTRACT(`draw_requests`.`options`, '$.model')
            ORDER BY COUNT(*);
            ",
            last_week,
        )
        .fetch_all(database)
        .await
        .unwrap_or_else(|_| Vec::new())
        .into_iter()
        .filter_map(|model_name| model_name.flatten())
        .collect()
    }

    pub async fn drawing(&self, database: &Pool<Sqlite>) -> Result<(), ()> {
        let message_id = self.message_id.get() as i64;
        let result = sqlx::query!(
            r#"
            UPDATE `draw_requests`
            SET `state` = 'drawing'
            WHERE `message_id` = ?
            "#,
            message_id,
        )
        .execute(database)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    pub async fn complete(&self, database: &Pool<Sqlite>) -> Result<(), ()> {
        let message_id = self.message_id.get() as i64;
        let result = sqlx::query!(
            r#"
            UPDATE `draw_requests`
            SET `state` = 'complete'
            WHERE `message_id` = ?
            "#,
            message_id,
        )
        .execute(database)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    pub async fn failed(&self, database: &Pool<Sqlite>) -> Result<(), ()> {
        let message_id = self.message_id.get() as i64;
        let result = sqlx::query!(
            r#"
            UPDATE `draw_requests`
            SET `state` = 'failed'
            WHERE `message_id` = ?
            "#,
            message_id,
        )
        .execute(database)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }
}
