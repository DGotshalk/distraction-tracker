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
    response::{Html, IntoResponse},
    Extension,
};
use axum_client_ip::{SecureClientIp, XForwardedFor};
use axum_extra::TypedHeader;
use headers::UserAgent;
use sqlx::MySqlPool;

pub async fn homepage(
    Extension(pool): Extension<MySqlPool>,
    XForwardedFor(header_ip_x): XForwardedFor,
    SecureClientIp(header_ip): SecureClientIp,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
) -> impl IntoResponse {
    // a bit brute force, but if we have XForwardedFor headers, then we check them, if we don't we
    // have to trust the ConnectInfo. I wish I could get this automatically in 1 variable, but I
    // the axum_client_ip docs didn't seem to have how to do that :/
    let client_ip: String = if header_ip_x.is_empty() {
        check_if_ip(header_ip)
    } else {
        check_if_ip(header_ip_x[0])
    };
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
