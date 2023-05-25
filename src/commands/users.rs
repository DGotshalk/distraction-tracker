//
// user.rs
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

pub async fn add_user(
    pool: &MySqlPool,
    ip_address: String,
    user_agent: String,
) -> sqlx::Result<Users> {
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

    let user_opt = get_user(pool, ip_address, user_agent)
        .await?
        .expect("Recently inserted user somehow does not exist");
    Ok(user_opt)
}

