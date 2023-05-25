//
// homepage.rs
// Copyright (C) 2023 dgotshalk <dgotshalk@Dissonance>
// Distributed under terms of the MIT license.
//

use crate::commands::{add_user::add_user, get_user::get_user, increment::increment};
use crate::routes::check_if_ip;
use askama::Template;
use axum::{
    headers::UserAgent,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    Extension, TypedHeader,
};
use axum_client_ip::LeftmostXForwardedFor;
use chrono::{DateTime, Utc};
use sqlx::MySqlPool;
use std::error::Error;

pub async fn homepage(
    Extension(pool): Extension<MySqlPool>,
    LeftmostXForwardedFor(header_ip): LeftmostXForwardedFor,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
) -> impl IntoResponse {
    let client_ip: String = check_if_ip(header_ip);
    let today: DateTime<Utc> = Utc::now();
    let today_naive = today.date_naive();

    let prospective_user = get_user(&pool, client_ip.clone(), user_agent.to_string()).await;
    let accepted_user = match prospective_user {
        Ok(user) => user,
        Err(err) => return return_error_as_html(err),
    };
    let prospective_connected_user = match accepted_user {
        Some(user) => increment(&pool, user.id, today_naive).await,
        None => {
            let new_user = add_user(&pool, client_ip.clone(), user_agent.to_string()).await;
            let added_user = match new_user {
                Ok(user) => user,
                Err(err) => return return_error_as_html(err),
            };
            increment(&pool, added_user.id, today_naive).await
        }
    };

    let connected_user = match prospective_connected_user {
        Ok(user_con) => user_con,
        Err(err) => return return_error_as_html(err),
    };

    let template = IndexTemplate {
        message: String::from("Don't be distracted!"),
        count: connected_user.connection_count,
    };
    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => return return_error_as_html(err),
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    message: String,
    count: u32,
}

fn return_error_as_html<E: Error>(err: E) -> Response {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Failed to render template. Error {}", err),
    )
        .into_response()
}
