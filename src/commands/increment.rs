//
// increment.rs
// Copyright (C) 2023 dgotshalk <dgotshalk@Dissonance>
// Distributed under terms of the MIT license.
//

use crate::commands::{add_user_connection, get_user_connection};
use crate::models::{UserConnections, Users};
use chrono::NaiveDate;
use sqlx::MySqlPool;

pub async fn increment(
    pool: &MySqlPool,
    user: &Users,
    today_naive: NaiveDate,
) -> sqlx::Result<Option<UserConnections>> {
    // should just return the an &models::Users with an id. that has been incremented
    //want to do:
    //assume the user exists
    // using the id, add increment the connection of today
    let user_connection = get_user_connection(pool, user, today_naive).await?;

    // match to see if we have an existing user connection. if we do, just add it up by 1.
    // if we don't, make a new one. when adding a new one, the user connection becomes 1 smile
    let incremented = match user_connection {
        Some(connection) => add_1(pool, &connection).await?,
        None => add_user_connection(pool, user, today_naive).await?,
    };

    let updated_connection = get_user_connection(pool, user, today_naive).await?;
    Ok(updated_connection)
}

async fn add_1(
    pool: &MySqlPool,
    user_connect: &UserConnections,
) -> sqlx::Result<sqlx::mysql::MySqlQueryResult> {
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
    Ok(add_1)
}
