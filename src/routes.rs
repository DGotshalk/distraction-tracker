// routes.rs
// Copyright (C) 2023 dgotshalk <dgotshalk@Dissonance>
// Distributed under terms of the MIT license.
//

use askama::Template;
use axum::{
    extract::connect_info::{ConnectInfo, Connected},
    headers::UserAgent,
    http::StatusCode,
    response::{Html, IntoResponse},
    TypedHeader,
};

struct DateCount {
    date: String,
    count: i32,
}

pub async fn homepage(
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
) -> impl IntoResponse {
    // this is where I would pass this to a sql query handler
    // NEED:
    // date (we can generate)
    // ip
    // agent
    // generate counter from these counts and put them here :)

    let template = IndexTemplate {
        message: String::from("Dont' be distracted"),
        ip: String::from(addr.ip().to_string()),
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
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
) -> impl IntoResponse {
    //Now I need to get the string back from the request
    // this is where I would pass this to a sql query handler
    // NEED (would make a list of this data and put them in the dates env ):
    // date
    // ip
    // agent
    // generate counter from these counts and put them here

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
    let template = HistoryTemplate {
        message: String::from("Dont' be distracted"),
        ip: String::from(addr.ip().to_string()),
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
