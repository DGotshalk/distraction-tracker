//
// homepage.rs
// Copyright (C) 2023 dgotshalk <dgotshalk@Dissonance>
// Distributed under terms of the MIT license.
//

use crate::routes::check_if_ip;
use askama::Template;
use axum::{
    headers::UserAgent,
    http::StatusCode,
    response::{Html, IntoResponse},
    Extension, TypedHeader,
};
use axum_client_ip::LeftmostXForwardedFor;
use sqlx::MySqlPool;
use chrono::{DateTime, Utc};

use crate::commands::{add_user::add_user, get_user::get_user, get_user_connection, increment::increment};

pub async fn homepage(
    Extension(pool): Extension<MySqlPool>,
    LeftmostXForwardedFor(header_ip): LeftmostXForwardedFor,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
) -> impl IntoResponse {
    let client_ip: String = check_if_ip(header_ip);
    let today: DateTime<Utc> = Utc::now();
    let today_naive = today.date_naive();
    let prospective_user = get_user(&pool, client_ip, user_agent.to_string()).await;
    let accepted_user = match prospective_user {
        Ok(user) => user, 
        Err(err) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get user. Error {}", err),
        )
            .into_response(),
    };

    // this is not good. need to handle this better. good to have results and options, but me oh my
    // consider changing the add_user to not be an option, it really shouldnt be

    let connected_user = match accepted_user{
        Some(user) => increment(&pool, &user, today_naive).await,
        None => increment(&pool, 
            match add_user(&pool, client_ip, user_agent.to_string()).await{
                Ok(user) => match user {
                    Some(u) => &u,
                    None => return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to get user. Error 'No user'"),
                )
                    .into_response(),
                }, 
                Err(err) => return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to get user. Error {}", err),
                )
                    .into_response(),

            }, today_naive).await,
    };
    


    

    let template = IndexTemplate {
        message: String::from("Don't be distracted!"),
        ip: String::from(client_ip),
        count: , connected_user.
    };
    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render template. Error {}", err),
        )
            .into_response(),
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    message: String,
    ip: String,
    count: i64,
}
