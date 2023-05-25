//
// increment.rs
// Copyright (C) 2023 dgotshalk <dgotshalk@Dissonance>
// Distributed under terms of the MIT license.
//

use crate::commands::{add_user_connection, get_user_connection};
use crate::models::UserConnections;
use chrono::NaiveDate;
use sqlx::MySqlPool;

pub async fn increment(
    pool: &MySqlPool,
    user_id: u64,
    today_naive: NaiveDate,
) -> sqlx::Result<UserConnections> {
    let user_connection = get_user_connection(pool, user_id, today_naive).await?;

    // match to see if we have an existing user connection. if we do, just add it up by 1.
    // if we don't, make a new one. when adding a new one, the user connection becomes 1 smile
    let incremented_connect = match user_connection {
        Some(connection) => add_1(pool, &connection).await?,
        None => add_user_connection(pool, user_id, today_naive).await?,
    };

    Ok(incremented_connect)
}

async fn add_1(pool: &MySqlPool, user_connect: &UserConnections) -> sqlx::Result<UserConnections> {
    let add_1 = sqlx::query!(
        r#"
            UPDATE user_connections 
            SET connection_count = connection_count +1 
            WHERE user_id = ? AND connection_date = ?
            "#,
        user_connect.user_id,
        user_connect.connection_date
    )
    .execute(pool)
    .await?;
    let incremented_user_con =
        get_user_connection(pool, user_connect.user_id, user_connect.connection_date)
            .await?
            .expect("User should already exist, should not be none :)");
    Ok(incremented_user_con)
}
