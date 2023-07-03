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
    let app = app_with_mysql()
        .await
        .into_make_service_with_connect_info::<std::net::SocketAddr>();
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8080));

    axum::Server::bind(&addr).serve(app).await.unwrap();
}

async fn app_with_mysql() -> Router {
    dotenv::dotenv().ok();
    let pool =
        connect_to_db(&env::var("DATABASE_URL").expect("Expected DATABASE_URL in environment"))
            .await;
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
    use ::axum_test::TestServer;
    use axum::{
        body::Body,
        extract::connect_info::IntoMakeServiceWithConnectInfo,
        http::{Request, StatusCode},
    };
    //use tower::Service;
    //use tower::ServiceExt;

    async fn app_mock_connect() -> IntoMakeServiceWithConnectInfo<axum::Router, std::net::SocketAddr>
    {
        app_with_mysql()
            .await
            .into_make_service_with_connect_info::<std::net::SocketAddr>()
        // This isn't working
        // I should be able to use this instead of doing into_make_service_with_connect_info
        //.layer(MockConnectInfo(std::net::SocketAddr::from((
        //    [127, 0, 0, 1],
        //    3000,
        //)))).into_make_service()
    }

    #[tokio::test]
    async fn run_app() {
        let listener =
            std::net::TcpListener::bind("127.0.0.1:9090".parse::<std::net::SocketAddr>().unwrap())
                .unwrap();
        let app = app_mock_connect().await;

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
                    .header("User-Agent", "custom")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response_index.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_index_route() {
        let test_app = TestServer::new(app_mock_connect().await).unwrap();

        let response_index = test_app
            .get("/")
            .add_header(
                hyper::http::HeaderName::from_static("user-agent"),
                hyper::http::HeaderValue::from_static("custom"),
            )
            .await;
        assert_eq!(response_index.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_history_route() {
        let test_app = TestServer::new(app_mock_connect().await).unwrap();

        let response_history = test_app
            .get("/history")
            .add_header(
                hyper::http::HeaderName::from_static("user-agent"),
                hyper::http::HeaderValue::from_static("custom"),
            )
            .await;
        assert_eq!(response_history.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_asset_route() {
        let test_app = TestServer::new(app_mock_connect().await).unwrap();

        let response_asset = test_app
            .get("/images/quote.png")
            .add_header(
                hyper::http::HeaderName::from_static("user-agent"),
                hyper::http::HeaderValue::from_static("custom"),
            )
            .await;
        assert_eq!(response_asset.status_code(), StatusCode::OK);
    }
}
