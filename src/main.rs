// main.rs
// Copyright (C) 2023 dgotshalk <dgotshalk@Dissonance>
// Distributed under terms of the MIT license.

use crate::db::connect_to_db;
use crate::routes::{homepage::homepage, iphistory::iphistory};
use axum::{routing::get, Extension, Router};
use axum_client_ip::SecureClientIpSource;
use std::env;
use tower_http::services::ServeDir;

mod commands;
mod db;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let pool =
        connect_to_db(&env::var("DATABASE_URL").expect("Expected DATABASE_URL in environment"))
            .await;

    let app = Router::new()
        .route("/", get(homepage))
        .route("/history", get(iphistory))
        .nest_service("/images", ServeDir::new("./static/assets/images"))
        .layer(SecureClientIpSource::ConnectInfo.into_extension())
        .layer(Extension(pool))
        .into_make_service_with_connect_info::<std::net::SocketAddr>();
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr).serve(app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
}
