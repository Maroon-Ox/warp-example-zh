#![deny(warnings)]
//! # Body
//!
//! 本示例代码演示如何使用body Filter，然后将内容填充到指定结构中。Body支持一下几种类型的filter：
//! * ```json``` - 返回一个匹配了json类型body filter
//! * ```form``` - 返回一个匹配了form类型body filter
//! * ```stream``` - 返回一个匹配了stream类型的body Filter  
//! * ```bytes``` - 返回一个匹配并提取联系字节内容的body Filter.
//! * ```aggregate``` - 返回了一个可匹配任何request，并提取聚合内容的body filter. Buf可能包含多个
//! 不联系的内容缓存。如果传输大量内容时可能会导致性能下降。‘’注意‘’缺省情况下，未设置接受内容长度
//! 限制，这里建议指定长度以避免大请求过多使用内存。
//!
//! 除此之外，还可以通过函数```content_length_limit()```来设置反馈内容的长度。
//!
//! 最后，这里提供了如何使用aggregate的示例代码。
//! ```
//! use warp::{Filter, Buf};
//!
//! fn full_body(mut body: impl Buf) {
//!    while body.has_remaining() {
//!        println!("slice = {:?}", body.bytes());
//!        let cnt = body.bytes().len();
//!         body.advance(cnt);
//!    }
//! }
//!
//! let _route = warp::body::content_length_limit(1024 * 32)
//!        .and(warp::body::aggregate())
//!        .map(full_body);
//! ```
//!
use serde::{Deserialize, Serialize};
use warp::{Filter, Buf};

#[derive(Deserialize, Serialize)]
struct Employee {
    name: String,
    rate: u32,
}

fn full_body(mut body: impl Buf) {
    while body.has_remaining() {
        println!("slice = {:?}", body.bytes());
        let cnt = body.bytes().len();
        body.advance(cnt);
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let _route = warp::body::content_length_limit(1024 * 32)
        .and(warp::body::aggregate())
        .map(full_body);

    // POST /employees/:rate  {"name":"Sean","rate":2}
    let promote = warp::post()
        .and(warp::path("employees"))
        .and(warp::path::param::<u32>())
        // Only accept bodies smaller than 16kb...
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(|rate, mut employee: Employee| {
            employee.rate = rate;
            warp::reply::json(&employee)
        });

    warp::serve(promote).run(([127, 0, 0, 1], 3030)).await
}
