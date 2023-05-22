//
// mod.rs
// Copyright (C) 2023 dgotshalk <dgotshalk@Dissonance>
// Distributed under terms of the MIT license.
//
pub mod add_user;
pub mod daily;
pub mod get_user;
pub mod increment;
pub mod weekly;

pub async fn get_user_connection(
    pool: &sqlx::MySqlPool,
    user: &crate::models::Users,
    today: chrono::NaiveDate,
) -> sqlx::Result<Option<crate::models::UserConnections>> {
    let user_id: u64 = user.id;
    let user_connection = sqlx::query_as!(
        crate::models::UserConnections,
        r#"
        SELECT * FROM user_connections
        WHERE user_id=? AND connection_date=?
        "#,
        user_id,
        today
    )
    .fetch_optional(pool)
    .await?;
    Ok(user_connection)
}

pub async fn add_user_connection(
    pool: &sqlx::MySqlPool,
    user: &crate::models::Users,
    today: chrono::NaiveDate,
) -> sqlx::Result<sqlx::mysql::MySqlQueryResult> {
    let user_id: u64 = user.id;
    let added_connection = sqlx::query!(
        r#"
            INSERT INTO user_connections (user_id, connection_date, connection_count) 
            VALUES(?, ?, 1)
            "#,
        user_id,
        today
    )
    .execute(pool)
    .await?;
    Ok(added_connection)
}
