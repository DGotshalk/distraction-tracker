// routes.rs
// Copyright (C) 2023 dgotshalk <dgotshalk@Dissonance>
// Distributed under terms of the MIT license.
//

use askama::Template;
use axum::{
    headers::UserAgent,
    http::StatusCode,
    response::{Html, IntoResponse},
    TypedHeader,
};
use axum_client_ip::{LeftmostXForwardedFor, SecureClientIp};

struct DateCount {
    date: String,
    count: i32,
}

pub async fn homepage(
    LeftmostXForwardedFor(header_ip): LeftmostXForwardedFor,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
) -> impl IntoResponse {
    let client_ip: String = check_if_ip(header_ip);
    let template = IndexTemplate {
        message: String::from("Dont' be distracted"),
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

pub async fn iphistory(
    LeftmostXForwardedFor(header_ip): LeftmostXForwardedFor,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
) -> impl IntoResponse {
    let data: Vec<DateCount> = vec![
        DateCount {
            date: String::from("This"),
            count: 1,
        },
        DateCount {
            date: String::from("Is"),
            count: 0,
        },
        DateCount {
            date: String::from("Still"),
            count: 2,
        },
        DateCount {
            date: String::from("In"),
            count: 4,
        },
        DateCount {
            date: String::from("Beta"),
            count: 3,
        },
        DateCount {
            date: String::from("Testing"),
            count: 5,
        },
    ];
    let client_ip: String = check_if_ip(header_ip);
    let template = HistoryTemplate {
        message: String::from("Dont' be distracted"),
        ip: String::from(client_ip.to_string()),
        agent: String::from(user_agent.to_string()),
        dates: &data,
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

fn check_if_ip(header_ip: std::net::IpAddr) -> String {
    if header_ip.is_ipv4() || header_ip.is_ipv6() {
        header_ip.to_string()
    } else {
        String::from("None")
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    message: String,
    ip: String,
    agent: String,
}

#[derive(Template)]
#[template(path = "history.html")]
struct HistoryTemplate<'a> {
    message: String,
    ip: String,
    agent: String,
    dates: &'a Vec<DateCount>,
}
