//
// increment.rs
// Copyright (C) 2023 dgotshalk <dgotshalk@Dissonance>
// Distributed under terms of the MIT license.
//

use crate::models::Users;
use sqlx::MySqlPool;

pub async fn get_user(
    pool: &MySqlPool,
    ip_address: String,
    user_agent: String,
) -> sqlx::Result<Option<Users>> {
    //return a user with the id
    let result = sqlx::query_as!(
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
    Ok(result)
}
