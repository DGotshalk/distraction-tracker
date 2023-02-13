// main.rs
// Copyright (C) 2023 dgotshalk <dgotshalk@Dissonance>
// Distributed under terms of the MIT license.

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}


struct  AppState{
    app_name: String
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/index")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {app_name}")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState{
                app_name: String::from("/")
            }))
            .service(hello)
            .service(echo)
            .route("/hey" , web::get().to(manual_hello))
            .service(index)
            .service(
                web::scope("/app")
                    .app_data(web::Data::new(AppState{
                        app_name: String::from("/app")
                    }))
                    .route("/hey", web::get().to(manual_hello))
                    .service(hello)
                    .service(index)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
