//
// routes.rs
// Copyright (C) 2023 dgotshalk <dgotshalk@Dissonance>
// Distributed under terms of the MIT license.
//


use actix_web::{ get, post, web, HttpResponse, Responder, HttpRequest };

pub struct  AppState{
    pub app_name: String
}

#[get("/")]
pub async fn homepage(req: HttpRequest) -> impl Responder {
        let val = String::from(req.connection_info().realip_remote_addr().unwrap_or("None"));
        HttpResponse::Ok().body(val)
}

#[get("/hello")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/index")]
pub async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {app_name}")
}
