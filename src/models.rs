//
// models.rs
// Copyright (C) 2023 dgotshalk <dgotshalk@Dissonance>
// Distributed under terms of the MIT license.
//
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct Users {
    pub id: i32,
    pub ip_address: String,
}

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct UserConnections {
    pub id: i32,
    pub user_id: i32,
    pub connection_date: String,
    pub connection_count: i32,
}
