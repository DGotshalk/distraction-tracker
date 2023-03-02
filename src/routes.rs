//
// routes.rs
// Copyright (C) 2023 dgotshalk <dgotshalk@Dissonance>
// Distributed under terms of the MIT license.
//

use actix_files::NamedFile;
use actix_web::{get, HttpRequest, HttpResponse, Responder, Result};
use std::path::PathBuf;

#[get("/")]
pub async fn homepage(req: HttpRequest) -> impl Responder {
    //Now I need to get the string back from the request
    let val = String::from(req.connection_info().realip_remote_addr().unwrap_or("None"));
    // this is where I would pass this to a sql query handler
    HttpResponse::Ok().body(val)
}

#[get("/history")]
pub async fn iphistory(req: HttpRequest) -> impl Responder {
    //Now I need to get the string back from the request
    let val = String::from(req.connection_info().realip_remote_addr().unwrap_or("None"));
    // this is where I would pass this to a sql query handler
    HttpResponse::Ok().body(val)
}

#[get("/index")]
pub async fn index(_req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "./src/static/index.html".parse().unwrap();
    Ok(NamedFile::open_async(path).await?)
}
