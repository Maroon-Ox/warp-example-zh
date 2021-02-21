//! # Header
//!
//! 第四个示例代码演示了如何处理HTTP请求中的header数据。
//!
//! 在这个示例代码中，我们应用了warp::header提供的对于HTTP请求header数据的处理功能。实现了一下场景：
//! 1. header数据中``host``字段只能是IP地址
//! 2. 利用``exact()``函数，指定接受的内容格式。例如：text/html, text/plain和application/json等。
//!
//! 此示例代码中，我们只接受text/html。对于其它内容格式不合要求，直接返回``400 Bad request``错误。
//!
#![deny(warnings)]
use serde::Serialize;
use std::convert::Infallible;
use std::net::SocketAddr;
use warp::http::StatusCode;
use warp::{http::header::HeaderValue, Filter, Rejection, Reply};

/// Create a server that requires header conditions:
///
/// - `Host` is a `SocketAddr`
/// - `Accept` is exactly `*/*`
///
/// If these conditions don't match, a 404 is returned.
#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // For this example, we assume no DNS was used,
    // so the Host header should be an address.
    let routes = warp::header::<SocketAddr>("host")
        .and(warp::header::value("content-type"))
        .and(warp::header::exact_ignore_case("accept", "*/*"))
        .map(|addr: SocketAddr, value: HeaderValue| format!("accepting {:?} on {:?}", value, addr));

    warp::serve(routes.recover(handle_rejection))
        .run(([0, 0, 0, 0], 3030))
        .await;
}

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}
async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Rejected due to :NOT_FOUND.";
    } else if let Some(_e) = err.find::<warp::reject::InvalidHeader>() {
        code = StatusCode::BAD_REQUEST;
        message = "Rejected due to :InvalidHeader.";
    } else if let Some(_e) = err.find::<warp::reject::MissingHeader>() {
        code = StatusCode::BAD_REQUEST;
        message = "Rejected due to :MissingHeader.";
    } else if let Some(_e) = err.find::<warp::reject::UnsupportedMediaType>() {
        code = StatusCode::BAD_REQUEST;
        message = "Rejected due to :UnsupportedMediaType.";
    } else {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Unhandled Rejection.";
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}
