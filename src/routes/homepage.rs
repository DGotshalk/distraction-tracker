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

pub async fn homepage(
    Extension(pool): Extension<MySqlPool>,
    LeftmostXForwardedFor(header_ip): LeftmostXForwardedFor,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
) -> impl IntoResponse {
    let client_ip: String = check_if_ip(header_ip);
    let template = IndexTemplate {
        message: String::from("Don't be distracted!"),
        ip: String::from(client_ip),
        agent: String::from(user_agent.to_string()),
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
    agent: String,
}
