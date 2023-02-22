// main.rs
// Copyright (C) 2023 dgotshalk <dgotshalk@Dissonance>
// Distributed under terms of the MIT license.

use actix_web::{App, HttpServer};

mod routes;
use crate::routes::{homepage, iphistory};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {}
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(homepage).service(iphistory))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
