// main.rs
// Copyright (C) 2023 dgotshalk <dgotshalk@Dissonance>
// Distributed under terms of the MIT license.

use actix_web::{web, App, HttpServer};
use handlebars::Handlebars;

mod routes;
use crate::routes::{homepage, index, iphistory};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {}
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);
    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .service(homepage)
            .service(iphistory)
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
