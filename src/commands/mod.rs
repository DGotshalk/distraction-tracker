//
// mod.rs
// Copyright (C) 2023 dgotshalk <dgotshalk@Dissonance>
// Distributed under terms of the MIT license.
//
pub mod add_user;
pub mod get_user;
pub mod increment;
pub mod weekly;

pub async fn get_user_connection(
    pool: &sqlx::MySqlPool,
    user_id: u64,
    today: chrono::NaiveDate,
) -> sqlx::Result<Option<crate::models::UserConnections>> {
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
    user_id: u64,
    today: chrono::NaiveDate,
) -> sqlx::Result<crate::models::UserConnections> {
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
    let user_connection_opt = get_user_connection(pool, user_id, today)
        .await?
        .expect("Recently added user connection does not exist");
    Ok(user_connection_opt)
}
