// main.rs
// Copyright (C) 2023 dgotshalk <dgotshalk@Dissonance>
// Distributed under terms of the MIT license.

use axum::{routing::get, Router};
use tower_http::services::ServeDir;

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
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(homepage))
        .route("/history", get(iphistory))
        .nest_service(
            "/static/assets/images",
            ServeDir::new("static/assets/images"),
        );
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
