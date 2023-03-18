// main.rs
// Copyright (C) 2023 dgotshalk <dgotshalk@Dissonance>
// Distributed under terms of the MIT license.

use actix_files::Files;
use actix_web::{web, App, HttpServer};
use handlebars::Handlebars;
mod routes;
use routes::{homepage, iphistory};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {}
}

//need to add the assets folder and allow for sub directories ./assets
//may literally need the actix files
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
            .service(Files::new("/static/assets", "./static/assets"))
            .service(homepage)
            .service(iphistory)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
