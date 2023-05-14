//
// increment.rs
// Copyright (C) 2023 dgotshalk <dgotshalk@Dissonance>
// Distributed under terms of the MIT license.
//

use crate::models::{UserConnections, Users};
use sqlx::MySqlPool;

pub async fn add_user(
    pool: &MySqlPool,
    user_agent: String,
    ip_address: String,
) -> sqlx::Result<Option<Users>> {
    //assume user does not exist create a new user, and increment connection by 1
    let insert = sqlx::query!(
        r#"
        INSERT INTO users (ip_address, user_agent) VALUES(?,?)
        "#,
        ip_address,
        user_agent
    )
    .execute(pool)
    .await?;

    let user = sqlx::query_as!(
        Users,
        r#"
        SELECT * FROM users 
        WHERE ip_address = ? AND user_agent = ?
        "#,
        ip_address,
        user_agent
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}
