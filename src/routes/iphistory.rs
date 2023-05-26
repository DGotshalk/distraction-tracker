//
// iphistory.rs
// Copyright (C) 2023 dgotshalk <dgotshalk@Dissonance>
// Distributed under terms of the MIT license.
//

use crate::commands::{
    user_connections::weekly,
    users::{add_user, get_user},
};
use crate::models::UserConnections;
use crate::routes::helpers::{check_if_ip, return_error_as_html};
use askama::Template;
use axum::{
    headers::UserAgent,
    response::{Html, IntoResponse},
    Extension, TypedHeader,
};

use axum_client_ip::LeftmostXForwardedFor;
use sqlx::MySqlPool;

pub async fn iphistory(
    Extension(pool): Extension<MySqlPool>,
    LeftmostXForwardedFor(header_ip): LeftmostXForwardedFor,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
) -> impl IntoResponse {
    let client_ip: String = check_if_ip(header_ip);
    let prospective_user = get_user(&pool, client_ip.clone(), user_agent.to_string()).await;
    let accepted_user = match prospective_user {
        Ok(user) => user,
        Err(err) => return return_error_as_html(err),
    };
    let last_7_connections_res = match accepted_user {
        Some(user) => weekly(&pool, user.id).await,
        None => {
            let new_user = add_user(&pool, client_ip.clone(), user_agent.to_string()).await;
            let added_user = match new_user {
                Ok(user) => user,
                Err(err) => return return_error_as_html(err),
            };
            weekly(&pool, added_user.id).await
        }
    };

    let last_7_connections = match last_7_connections_res {
        Ok(user_con) => user_con,
        Err(err) => return return_error_as_html(err),
    };

    let template = HistoryTemplate {
        message: String::from("The last 7 dates that you have connected"),
        dates: &last_7_connections,
    };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => return return_error_as_html(err),
    }
}

#[derive(Template)]
#[template(path = "history.html")]
struct HistoryTemplate<'a> {
    message: String,
    dates: &'a Vec<UserConnections>,
}
