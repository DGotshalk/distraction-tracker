//
// headers.rs
// Copyright (C) 2023 dgotshalk <dgotshalk@Dissonance>
// Distributed under terms of the MIT license.
//

pub fn check_if_ip(header_ip: std::net::IpAddr) -> String {
    if header_ip.is_ipv4() || header_ip.is_ipv6() {
        header_ip.to_string()
    } else {
        String::from("None")
    }
}

