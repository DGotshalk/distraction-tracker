// routes.rs
// Copyright (C) 2023 dgotshalk <dgotshalk@Dissonance>
// Distributed under terms of the MIT license.
//

use askama::Template;
use axum::response::{Html, IntoResponse, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct DateCount {
    date: String,
    count: i32,
}

pub async fn homepage() {
    // this is where I would pass this to a sql query handler
    // NEED:
    // date (we can generate)
    // ip
    // agent
    // generate counter from these counts and put them here :)

    let data = json!({
        "message": "Don't be distracted!",
        "ip": "127.0.0.1",
        "agent": "none"
    });
}

pub async fn iphistory() {
    //Now I need to get the string back from the request
    // this is where I would pass this to a sql query handler
    // NEED (would make a list of this data and put them in the dates env ):
    // date
    // ip
    // agent
    // generate counter from these counts and put them here

    let data = json!({
        "message": "How often have you been distracted?",
        "ip": "127.0.0.1",
        "agent": "none",
        "dates" : [
            {
                "date" : "10-03-2023",
                "count": 1
            },
            {
                "date" : "11-03-2023",
                "count": 0
            },
            {
                "date" : "12-03-2023",
                "count": 2
            },
            {
                "date" : "13-03-2023",
                "count": 4
            },
            {
                "date" : "14-03-2023",
                "count": 3
            },
            {
                "date" : "15-03-2023",
                "count": 5
            }
        ]
    });
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    message: String,
    ip: String,
    agent: String,
}
