// main.rs
// Copyright (C) 2023 dgotshalk <dgotshalk@Dissonance>
// Distributed under terms of the MIT license.

use crate::db::connect_to_db;
use crate::routes::{homepage::homepage, iphistory::iphistory};
use axum::{routing::get, Extension, Router};
use axum_client_ip::SecureClientIpSource;
use sqlx::MySqlPool;
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

    let app = app(pool).into_make_service_with_connect_info::<std::net::SocketAddr>();
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr).serve(app).await.unwrap();
}

fn app(pool: MySqlPool) -> Router {
    Router::new()
        .route("/", get(homepage))
        .route("/history", get(iphistory))
        .nest_service("/images", ServeDir::new("./static/assets/images"))
        .layer(SecureClientIpSource::ConnectInfo.into_extension())
        .layer(Extension(pool))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connect_to_db;
    use axum::http::{Request, StatusCode};
    use std::env; //

    #[tokio::test]
    async fn test_routes() {
        dotenv::dotenv().ok();
        let pool =
            connect_to_db(&env::var("DATABASE_URL").expect("Expected DATABASE_URL in environment"))
                .await;
        let listener =
            std::net::TcpListener::bind("127.0.0.1:9090".parse::<std::net::SocketAddr>().unwrap())
                .unwrap();
        let app = app(pool).into_make_service_with_connect_info::<std::net::SocketAddr>();

        tokio::spawn(async move {
            axum::Server::from_tcp(listener)
                .unwrap()
                .serve(app)
                .await
                .unwrap()
        });

        let client = hyper::Client::new();
        let response_index = client
            .request(
                Request::builder()
                    .uri("http://127.0.0.1:9090/")
                    .body(hyper::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        println!("{}", response_index.body());
        assert_eq!(response_index.status(), StatusCode::OK);

        let response_history = client
            .request(
                Request::builder()
                    .uri("http://127.0.0.1:9090/history")
                    .body(hyper::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response_history.status(), StatusCode::OK);

        let response_image = client
            .request(
                Request::builder()
                    .uri("http://127.0.0.1:9090/images/Quote.png")
                    .body(hyper::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response_image.status(), StatusCode::OK);
    }
}
