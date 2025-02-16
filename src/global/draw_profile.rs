pub mod options;

use serenity::all::{ResolvedOption, ResolvedValue, UserId};
use sqlx::{Pool, Sqlite};

use options::Options;

#[derive(Debug)]
pub struct DrawProfile {
    pub name: String,
    pub user_id: UserId,
    pub options: Options,
}

impl DrawProfile {
    pub fn new_from_command<'a>(
        user_id: UserId,
        options: &Vec<ResolvedOption<'a>>,
    ) -> Option<DrawProfile> {
        let mut name = None;

        for option in options.iter() {
            match option {
                ResolvedOption {
                    value: ResolvedValue::String(value),
                    name: "profile_name",
                    ..
                } => name = Some(value.to_string()),
                _ => {}
            }
        }

        Some(DrawProfile {
            name: name?,
            options: Options::new_from_command(options),
            user_id: user_id,
        })
    }

    pub async fn save(&self, database: &Pool<Sqlite>) -> Result<(), ()> {
        let user_id = self.user_id.get() as i64;

        let options = serde_json::to_string(&self.options).map_err(|_| ())?;

        let result = sqlx::query!(
            r#"
            INSERT INTO `user_draw_profiles` (
                `name`,
                `options`,
                `user_id`,
                `active`
            )
            VALUES (?, ?, ?, ?)
            "#,
            self.name,
            options,
            user_id,
            false,
        )
        .execute(database)
        .await;

        println!("{:?}", result);
        match result {
            Ok(_) => Ok(()),
            Err(why) => {
                println!("Cannot save profile: {why}");

                Err(())
            }
        }
    }

    pub async fn get_available(
        user_id: UserId,
        database: &Pool<Sqlite>,
    ) -> Result<Vec<String>, ()> {
        let user_id = user_id.get() as i64;

        let result = sqlx::query_scalar!(
            r#"
            SELECT
                `name`
            FROM
                `user_draw_profiles`
            WHERE
                `user_id` = ?
            "#,
            user_id,
        )
        .fetch_all(database)
        .await;

        result.map_err(|_| ())
    }

    pub async fn remove(name: String, user_id: UserId, database: &Pool<Sqlite>) -> Result<(), ()> {
        let user_id = user_id.get() as i64;

        let result = sqlx::query!(
            r#"
            DELETE FROM
                `user_draw_profiles`
            WHERE
                `user_id` = ?
                AND `name` = ?
            "#,
            user_id,
            name,
        )
        .execute(database)
        .await;

        match result {
            Ok(_) => Ok(()),
            _ => Err(()),
        }
    }
}
