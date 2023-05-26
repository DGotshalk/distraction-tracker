//
// homepage.rs
// Copyright (C) 2023 dgotshalk <dgotshalk@Dissonance>
// Distributed under terms of the MIT license.
//

use crate::commands::{
    user_connections::increment,
    users::{add_user, get_user},
};
use crate::routes::helpers::{check_if_ip, get_hst_date, return_error_as_html};

use askama::Template;
use axum::{
    headers::UserAgent,
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
    let today_naive = get_hst_date();
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
