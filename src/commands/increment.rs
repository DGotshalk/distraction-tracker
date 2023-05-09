//
// increment.rs
// Copyright (C) 2023 dgotshalk <dgotshalk@Dissonance>
// Distributed under terms of the MIT license.
//

use crate::models;
use sqlx::{query, MySQLPool};

//pub async fn increment(pool: &MySQLPool, user: &models::User) -> Result<(), Error> {
//    //want to do:
//    // query to see if user exists, if user does, then increment number,
//    // if user doesn't, then don't increment.
//    let result = sqlx::query_as!(models::User);
//}
//
//async fn check_user(
//    pool: &MySQLPool,
//    user: &models::User,
//) -> sqlx::Result<models::User, sqlx::Error> {
//    let result = sqlx::query_as!(models::User);
//    result
//}
//async fn check_connection(pool: &MySQLPool, user: &models::User) -> sqlx::Result<(), sqlx::Error> {}
