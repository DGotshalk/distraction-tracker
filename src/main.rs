// main.rs
// Copyright (C) 2023 dgotshalk <dgotshalk@Dissonance>
// Distributed under terms of the MIT license.

use axum::{routing::get, Router};
use axum_client_ip::SecureClientIpSource;
use tower_http::services::ServeDir;

mod routes;
use routes::{homepage, iphistory};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {}
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(homepage))
        .route("/history", get(iphistory))
        .nest_service("/images", ServeDir::new("./static/assets/images"))
        .layer(SecureClientIpSource::ConnectInfo.into_extension());
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<std::net::SocketAddr>())
        .await
        .unwrap();
}
