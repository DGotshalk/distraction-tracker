//
// models.rs
// Copyright (C) 2023 dgotshalk <dgotshalk@Dissonance>
// Distributed under terms of the MIT license.
//

//Still debatable if I will need this, I have plans for serializing as json, but don't necessarily
//need to? Will have to check for efficiency in doing so :)
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Users {
    pub id: i32,
    pub ip_address: String,
    pub user_agent: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct UserConnections {
    pub id: i32,
    pub user_id: i32,
    pub connection_date: String,
    pub connection_count: i32,
}
