//
// routes.rs
// Copyright (C) 2023 dgotshalk <dgotshalk@Dissonance>
// Distributed under terms of the MIT license.
//

use actix_http::header;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use handlebars::Handlebars;
use serde_json::json;

#[get("/")]
pub async fn homepage(req: HttpRequest, hb: web::Data<Handlebars<'_>>) -> impl Responder {
    let req_remote_ip = String::from(req.connection_info().realip_remote_addr().unwrap_or("None"));
    let req_headers = req.headers();
    let req_agent = get_header_as_string(req_headers, header::USER_AGENT);

    let data = json!({
        "message": "Hello World",
        "ip": req_remote_ip,
        "agent": req_agent
    });

    let body = hb.render("index", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[get("/history")]
pub async fn iphistory(req: HttpRequest) -> impl Responder {
    //Now I need to get the string back from the request
    let val = String::from(req.connection_info().realip_remote_addr().unwrap_or("None"));
    // this is where I would pass this to a sql query handler
    HttpResponse::Ok().body(val)
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
