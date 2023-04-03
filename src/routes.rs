// routes.rs
// Copyright (C) 2023 dgotshalk <dgotshalk@Dissonance>
// Distributed under terms of the MIT license.
//

use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

struct DateCount {
    date: String,
    count: i32,
}

pub async fn homepage() -> impl IntoResponse {
    // this is where I would pass this to a sql query handler
    // NEED:
    // date (we can generate)
    // ip
    // agent
    // generate counter from these counts and put them here :)

    let template = IndexTemplate {
        message: String::from("Dont' be distracted"),
        ip: String::from("127.0.0.1"),
        agent: String::from("None"),
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

pub async fn iphistory() -> impl IntoResponse {
    //Now I need to get the string back from the request
    // this is where I would pass this to a sql query handler
    // NEED (would make a list of this data and put them in the dates env ):
    // date
    // ip
    // agent
    // generate counter from these counts and put them here

    let data: Vec<DateCount> = vec![
        DateCount {
            date: String::from("10-03-2023"),
            count: 1,
        },
        DateCount {
            date: String::from("11-03-2023"),
            count: 0,
        },
        DateCount {
            date: String::from("12-03-2023"),
            count: 2,
        },
        DateCount {
            date: String::from("13-03-2023"),
            count: 4,
        },
        DateCount {
            date: String::from("14-03-2023"),
            count: 3,
        },
        DateCount {
            date: String::from("15-03-2023"),
            count: 5,
        },
    ];
    let template = HistoryTemplate {
        message: String::from("Dont' be distracted"),
        ip: String::from("127.0.0.1"),
        agent: String::from("None"),
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
