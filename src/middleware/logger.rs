
use axum::{
    body::Body, extract::ConnectInfo, http::Request, middleware::Next, response::Response
};
use std::{net::SocketAddr, time::Instant};
use chrono::Local;


pub async fn logger_middleware(ConnectInfo(addr):ConnectInfo<SocketAddr>,req:Request<Body>,next:Next)->Response {
    let start=Instant::now();
    let method=req.method().clone();
    let uri=req.uri().clone();
    let response=next.run(req).await;
    let latency =start.elapsed();
    let status=response.status();
    let ip =addr.ip();
    let now =Local::now().format("%Y-%m-%d %H:%M:%S");

    
let method_style = match method.as_str() {
    "GET"    => "\x1b[37;42m", // Beyaz metin, Yeşil arka plan
    "POST"   => "\x1b[37;44m", // Beyaz metin, Mavi arka plan
    "DELETE" => "\x1b[37;41m", // Beyaz metin, Kırmızı arka plan
    _        => "\x1b[30;47m", // Siyah metin, Beyaz arka plan (Diğerleri için)
};

    let status_color=match status.as_u16() {
        200..=299 => "\x1b[32m", // Yeşil
        400..=499 => "\x1b[33m", // Sarı
        500..=599 => "\x1b[31m", // Kırmızı
        _ => "\x1b[37m",         // Beyaz
    };
    let reset = "\x1b[0m"; // Renkleri sıfırla
    println!(
        "{} | INFO | {}{:<6}{} | {:<15} | {:<20} | {:>8?} | {}{:6}{}",
        now,
        method_style, method.as_str(), reset,
        format!("IP: {}", ip), 
        uri.path(),
        latency,
        status_color,
        status.as_u16(),
        reset
    );

    response
}