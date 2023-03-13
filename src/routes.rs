//
// routes.rs
// Copyright (C) 2023 dgotshalk <dgotshalk@Dissonance>
// Distributed under terms of the MIT license.
//

use actix_http::header;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct DateCount {
    date: String,
    count: i32,
}

#[get("/")]
pub async fn homepage(req: HttpRequest, hb: web::Data<Handlebars<'_>>) -> impl Responder {
    let req_remote_ip = String::from(req.connection_info().realip_remote_addr().unwrap_or("None"));
    let req_agent = get_header_as_string(req.headers(), header::USER_AGENT);
    // this is where I would pass this to a sql query handler
    // NEED:
    // date (we can generate)
    // ip
    // agent
    // generate counter from these counts and put them here :)

    let data = json!({
        "message": "Don't be distracted!",
        "ip": req_remote_ip,
        "agent": req_agent
    });
    let body = hb.render("index", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[get("/history")]
pub async fn iphistory(req: HttpRequest, hb: web::Data<Handlebars<'_>>) -> impl Responder {
    //Now I need to get the string back from the request
    let req_remote_ip = String::from(req.connection_info().realip_remote_addr().unwrap_or("None"));
    let req_agent = get_header_as_string(req.headers(), header::USER_AGENT);
    // this is where I would pass this to a sql query handler
    // NEED (would make a list of this data and put them in the dates env ):
    // date
    // ip
    // agent
    // generate counter from these counts and put them here

    let data = json!({
        "message": "How often have you been distracted?",
        "ip": req_remote_ip,
        "agent": req_agent,
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
    let body = hb.render("history", &data).unwrap();
    HttpResponse::Ok().body(body)
}

fn get_header_as_string(
    req_headers: &header::HeaderMap,
    header_type: header::HeaderName,
) -> String {
    let req_header = req_headers.get(header_type);
    if req_header.is_some() {
        String::from(req_header.unwrap().to_str().unwrap_or("None")) //JIC
    } else {
        String::from("None")
    }
}
