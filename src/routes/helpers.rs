//
// headers.rs
// Copyright (C) 2023 dgotshalk <dgotshalk@Dissonance>
// Distributed under terms of the MIT license.
//

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::{DateTime, NaiveDate, Utc};
use chrono_tz::US::{Hawaii, Pacific};
use std::error::Error;

pub fn check_if_ip(header_ip: std::net::IpAddr) -> String {
    if header_ip.is_ipv4() || header_ip.is_ipv6() {
        header_ip.to_string()
    } else {
        String::from("None")
    }
}

pub fn get_hst_date() -> NaiveDate {
    let today_utc: DateTime<Utc> = Utc::now();
    let today_hst = today_utc.with_timezone(&Hawaii);
    today_hst.date_naive()
}

pub fn get_pst_date() -> NaiveDate {
    let today_utc: DateTime<Utc> = Utc::now();
    let today_hst = today_utc.with_timezone(&Pacific);
    today_hst.date_naive()
}

pub fn return_error_as_html<E: Error>(err: E) -> Response {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Failed to render template. Error {}", err),
    )
        .into_response()
}
