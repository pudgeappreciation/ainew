use serenity::all::{ChannelId, MessageId, UserId};
use sqlx::{Pool, Sqlite};

use crate::global::draw_request::DrawRequest;

#[derive(Debug)]
struct DbDrawRequest {
    state: String,
    options: String,
    user_id: i64,
    request_id: i64,
    channel_id: i64,
}

impl From<DbDrawRequest> for DrawRequest {
    fn from(value: DbDrawRequest) -> Self {
        DrawRequest {
            state: value.state,
            options: serde_json::from_str(&value.options).unwrap_or_default(),
            user_id: UserId::new(value.user_id as u64),
            request_id: MessageId::new(value.request_id as u64),
            channel_id: ChannelId::new(value.channel_id as u64),
        }
    }
}

pub async fn get_next_user(database: &Pool<Sqlite>) -> Option<i64> {
    sqlx::query_scalar!(
        r#"
        SELECT `user_id` as `user_id!`
        FROM (
            SELECT
                `user_id`,
                'default' AS `state`,
                0 AS `created_at`
            FROM `draw_requests`
            UNION ALL
            SELECT
                `user_id`,
                `state`,
                `created_at`
            FROM `draw_requests`
        )
        WHERE `state` NOT IN ('queued')
        GROUP BY `user_id`
        ORDER BY MAX(`created_at`) DESC
        "#,
    )
    .fetch_optional(database)
    .await
    .unwrap_or(None)
}

pub async fn get_next_request(database: &Pool<Sqlite>) -> Option<DrawRequest> {
    let Some(user_id) = get_next_user(database).await else {
        return None;
    };

    sqlx::query_as!(
        DbDrawRequest,
        r#"
        SELECT
            `state`,
            `options`,
            `user_id`,
            `request_id`,
            `channel_id`
        FROM `draw_requests`
        WHERE
            `user_id` = ?
            AND `state` IN ('queued')
        ORDER BY `created_at` ASC
        "#,
        user_id,
    )
    .fetch_optional(database)
    .await
    .unwrap_or(None)
    .map(|request| request.into())
}
