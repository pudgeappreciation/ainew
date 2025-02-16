pub mod options;

use serenity::all::{CreateEmbed, MessageBuilder, ResolvedOption, ResolvedValue, UserId};
use sqlx::{Pool, Sqlite};

use options::Options;

use crate::discord::commands::utilities::copy_modal::CopyButtonId;

#[derive(Debug)]
pub struct DrawProfile {
    pub name: String,
    pub user_id: UserId,
    pub options: Options,
    pub active: bool,
}

#[derive(Debug)]
struct DbDrawProfile {
    pub name: String,
    pub user_id: i64,
    pub options: String,
    pub active: bool,
}

impl TryFrom<DbDrawProfile> for DrawProfile {
    type Error = ();

    fn try_from(value: DbDrawProfile) -> Result<Self, Self::Error> {
        let options: Options = serde_json::from_str(&value.options).map_err(|_| ())?;

        Ok(Self {
            options,
            user_id: UserId::new(value.user_id as u64),
            active: value.active,
            name: value.name,
        })
    }
}

impl CopyButtonId for DrawProfile {
    fn id(&self) -> String {
        self.name.clone()
    }
}

impl DrawProfile {
    pub fn new_from_command<'a>(
        user_id: UserId,
        options: &Vec<ResolvedOption<'a>>,
    ) -> Option<DrawProfile> {
        let mut name = None;
        let mut active = false;

        for option in options.iter() {
            match option {
                ResolvedOption {
                    value: ResolvedValue::String(value),
                    name: "profile_name",
                    ..
                } => name = Some(value.to_string()),
                ResolvedOption {
                    value: ResolvedValue::Boolean(value),
                    name: "active",
                    ..
                } => active = *value,
                _ => {}
            }
        }

        Some(DrawProfile {
            name: name?,
            options: Options::new_from_command(options),
            user_id: user_id,
            active,
        })
    }

    pub fn embed(&self) -> CreateEmbed {
        let mut content = MessageBuilder::new();
        content
            .push_bold_safe("Active: ")
            .push_line(if self.active { "True" } else { "False" });
        self.options.embed(&mut content);

        CreateEmbed::new()
            .title(&self.name)
            .description(content.build())
    }

    pub fn to_command_options(&self) -> String {
        let mut command = format!("profile_name:{}", self.name);

        if self.active {
            command.push_str("\nactive:True");
        }

        command.push_str(&self.options.to_command_options());

        command
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
            ON CONFLICT DO
            UPDATE
            SET
                `options` = ?
            "#,
            self.name,
            options,
            user_id,
            false,
            options,
        )
        .execute(database)
        .await;

        if self.active {
            _ = Self::set_active(Some(self.name.clone()), self.user_id, database).await;
        }

        match result {
            Ok(_) => Ok(()),
            Err(why) => {
                println!("Cannot save profile: {why}");

                Err(())
            }
        }
    }

    pub async fn set_active(
        name: Option<String>,
        user_id: UserId,
        database: &Pool<Sqlite>,
    ) -> Result<(), ()> {
        let user_id = user_id.get() as i64;

        let result = sqlx::query!(
            r#"
            UPDATE
                `user_draw_profiles`
            SET
                `active` = COALESCE(`user_draw_profiles`.`name` = ?, false)
            WHERE
                `user_id` = ?
            "#,
            name,
            user_id,
        )
        .execute(database)
        .await;

        match result {
            Ok(_) => Ok(()),
            _ => Err(()),
        }
    }

    pub async fn get_available(
        user_id: UserId,
        database: &Pool<Sqlite>,
    ) -> Result<Vec<DrawProfile>, ()> {
        let user_id = user_id.get() as i64;

        let result = sqlx::query_as!(
            DbDrawProfile,
            r#"
            SELECT
                `name`,
                `options`,
                `user_id`,
                `active`
            FROM
                `user_draw_profiles`
            WHERE
                `user_id` = ?
            "#,
            user_id,
        )
        .fetch_all(database)
        .await;

        match result {
            Ok(profiles) => Ok(profiles
                .into_iter()
                .filter_map(|profile| profile.try_into().ok())
                .collect()),
            Err(_) => Err(()),
        }
    }

    pub async fn get(
        user_id: UserId,
        name: &str,
        database: &Pool<Sqlite>,
    ) -> Result<Option<DrawProfile>, ()> {
        let user_id = user_id.get() as i64;

        let result = sqlx::query_as!(
            DbDrawProfile,
            r#"
            SELECT
                `name`,
                `options`,
                `user_id`,
                `active`
            FROM
                `user_draw_profiles`
            WHERE
                `user_id` = ?
                AND `name` = ?
            "#,
            user_id,
            name,
        )
        .fetch_optional(database)
        .await;

        match result {
            Ok(profile) => match profile {
                Some(profile) => Ok(profile.try_into().ok()),
                None => Ok(None),
            },
            Err(_) => Err(()),
        }
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
