use sqlx::{Pool, Sqlite};

use crate::global::draw_request::{DbDrawRequest, DrawRequest};

pub async fn get_next_user(database: &Pool<Sqlite>) -> Option<i64> {
    sqlx::query_scalar!(
        r#"
        SELECT `user_id`
        FROM (
            SELECT
                `user_id`,
                'default' AS `state`,
                0 AS `created_at`
            FROM `draw_requests`
            WHERE
                NOT EXISTS (
                    SELECT 1
                    FROM `draw_requests` AS `draw_requests_inner`
                    WHERE 
                        `state` NOT IN ('queued')
                        AND `draw_requests_inner`.`user_id` = `draw_requests`.`user_id`
                )
            UNION ALL
            SELECT
                `user_id`,
                `state`,
                `created_at`
            FROM `draw_requests`
            WHERE
                EXISTS (
                    SELECT 1
                    FROM `draw_requests` AS `draw_requests_inner`
                    WHERE 
                        `state` IN ('queued')
                        AND `draw_requests_inner`.`user_id` = `draw_requests`.`user_id`
                )
        )
        WHERE `state` NOT IN ('queued')
        GROUP BY `user_id`
        ORDER BY MAX(`created_at`) ASC
        "#,
    )
    .fetch_optional(database)
    .await
    .unwrap_or(None)
    .flatten()
}

pub async fn get_next_request(database: &Pool<Sqlite>) -> Option<DrawRequest> {
    let Some(user_id) = get_next_user(database).await else {
        println!("No next user");
        return None;
    };

    let request = sqlx::query_as!(
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
        FROM `draw_requests`
        WHERE
            `user_id` = ?
            AND `state` IN ('queued')
        ORDER BY `created_at` ASC
        "#,
        user_id,
    )
    .fetch_optional(database)
    .await;

    match request {
        Ok(request) => request.map(|r| r.into()),
        Err(why) => {
            println!("Error fetching request: {:?}", why);

            None
        }
    }
}
