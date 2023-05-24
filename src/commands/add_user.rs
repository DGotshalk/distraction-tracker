//
// add_user.rs
// Copyright (C) 2023 dgotshalk <dgotshalk@Dissonance>
// Distributed under terms of the MIT license.
//

use crate::commands::get_user::get_user;
use crate::models::Users;
use sqlx::MySqlPool;

pub async fn add_user(
    pool: &MySqlPool,
    user_agent: String,
    ip_address: String,
) -> sqlx::Result<Option<Users>> {
    //assume user does not exist create a new user, and increment connection by 1
    let _insert = sqlx::query!(
        r#"
        INSERT INTO users (ip_address, user_agent) 
        VALUES(?,?)
        "#,
        ip_address,
        user_agent
    )
    .execute(pool)
    .await?;

    let user = get_user(pool, ip_address, user_agent).await?;

    Ok(user)
}
