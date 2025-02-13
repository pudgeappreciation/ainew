use std::time::{SystemTime, UNIX_EPOCH};

use serenity::all::UserId;
use sqlx::{Pool, Sqlite};

pub trait FavoritesString {
    fn string(&self) -> String;
}

impl<T> FavoritesString for &T
where
    T: FavoritesString,
{
    fn string(&self) -> String {
        self.to_owned().string()
    }
}

#[derive(Debug)]
pub enum Favorite<T>
where
    T: Sized,
{
    Favorited(T),
    Unfavorited(T),
}

impl Favorite<String> {
    pub async fn save(&self, user: UserId, database: &Pool<Sqlite>) -> Result<(), ()> {
        let user_id = user.get() as i64;

        let result = match self {
            Favorite::Favorited(value) => {
                let Ok(created_at) = SystemTime::now().duration_since(UNIX_EPOCH) else {
                    return Err(());
                };
                let created_at = created_at.as_secs() as i64;

                sqlx::query!(
                    r#"
                    INSERT INTO `user_favorites` (
                        `user_id`,
                        `name`,
                        `created_at`
                    )
                    VALUES (?, ?, ?)
                    "#,
                    user_id,
                    value,
                    created_at,
                )
                .execute(database)
                .await
            }
            Favorite::Unfavorited(value) => {
                sqlx::query!(
                    r#"
                    DELETE FROM `user_favorites`
                    WHERE
                        `user_id` = ?
                        AND `name` = ?
                    "#,
                    user_id,
                    value,
                )
                .execute(database)
                .await
            }
        };

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }
}

pub async fn favorited<T>(
    items: T,
    user_id: UserId,
    database: &Pool<Sqlite>,
) -> Vec<Favorite<T::Item>>
where
    T: Iterator,
    T::Item: FavoritesString,
{
    let favorites = get_favorites(user_id, database).await;

    items
        .map(|item| match favorites.contains(&item.string()) {
            true => Favorite::Favorited(item),
            false => Favorite::Unfavorited(item),
        })
        .collect()
}

pub async fn get_favorites(user_id: UserId, database: &Pool<Sqlite>) -> Vec<String> {
    let user_id = user_id.get() as i64;

    sqlx::query_scalar!(
        r#"
        SELECT
            `name`
        FROM `user_favorites`
        WHERE
            `user_id` = ?
        ORDER BY `created_at` ASC
        "#,
        user_id,
    )
    .fetch_all(database)
    .await
    .unwrap_or_else(|_| Vec::new())
}
