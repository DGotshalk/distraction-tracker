//
// db.rs
// Copyright (C) 2023 dgotshalk <dgotshalk@Dissonance>
// Distributed under terms of the MIT license.
//
use sqlx::MySqlPool;

pub async fn connect_to_db(mysql_url: &str) -> MySqlPool {
    let pool = MySqlPool::connect(mysql_url)
        .await
        .expect("mysql initialization error");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("error with migrations");
    pool
}
