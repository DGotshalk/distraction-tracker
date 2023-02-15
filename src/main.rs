// main.rs
// Copyright (C) 2023 dgotshalk <dgotshalk@Dissonance>
// Distributed under terms of the MIT license.

use actix_web::{ web, App, HttpServer };


mod routes;
use crate::routes::{ AppState, hello, echo, manual_hello, index, homepage };

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState{
                app_name: String::from("/")
            }))
            .service(homepage)
            .service(hello)
            .service(echo)
            .route("/hey" , web::get().to(manual_hello))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
