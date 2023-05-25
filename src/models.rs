//
// models.rs
// Copyright (C) 2023 dgotshalk <dgotshalk@Dissonance>
// Distributed under terms of the MIT license.
//

//Still debatable if I will need this, I have plans for serializing as json, but don't necessarily
//need to? Will have to check for efficiency in doing so :)
use chrono::NaiveDate;

#[derive(sqlx::FromRow)]
pub struct Users {
    pub id: u64,
    pub ip_address: String,
    pub user_agent: String,
}

#[derive(sqlx::FromRow)]
pub struct UserConnections {
    pub id: u64,
    pub user_id: u64,
    pub connection_date: NaiveDate,
    pub connection_count: u32,
}
