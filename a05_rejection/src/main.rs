#![deny(warnings)]
//!    # Rejection
//!
//! 这里的代码展示了当一个filter不能继续处理的情况下，如何让另一个filter继续处理。
//!
//! 本例中代码展示了通过header和body传递，并进行除法。被除数通过http://[...]/math/u16的方式
//! 传递，除数即可能通过header传递，有可以通过body post传递。在处理过程中的任何错误通过
//! 函数recover()指定给handle_rejection()。处理结果通过json格式传递。
//!
//! ## 通过header传递除数
//!
//! 1. header中的除数通过HTTP GET传递，
//! 2. 函数div_by()用来从header中提取除数，
//! 3. map()来进行除法并返回结果。
//!
//! ## 通过body传递除数的
//!
//! 1. body中的除数通过HTTP POST传递，
//! 2. 直接通过解析json数据获取除数，
//! 3. map()来进行除法并返回结果。
//!  
//! ## 函数handle_rejection()中的逻辑
//!
//! handle_rejection()函数接收并处理不同的rejection，尝试返回个性化的内容和代码。否则，进一步
//! 将rejection传递出去。

use std::convert::Infallible;
use std::error::Error;
use std::num::NonZeroU16;

use serde::{Deserialize, Serialize};
use warp::http::StatusCode;
use warp::{reject, Filter, Rejection, Reply};

/// 主函数实现
#[tokio::main]
async fn main() {
    let math = warp::path!("math" / u16);
    let div_with_header = math
        .and(warp::get())
        .and(div_by())
        .map(|num: u16, denom: NonZeroU16| {
            warp::reply::json(&Math {
                op: format!("{} / {}", num, denom),
                output: num / denom.get(),
            })
        });

    let div_with_body = math
        .and(warp::post())
        .and(warp::body::json())
        .map(|num: u16, body: DenomRequest| {
            warp::reply::json(&Math {
                op: format!("{} / {}", num, body.denom),
                output: num / body.denom.get(),
            })
        });

    let routes = div_with_header.or(div_with_body).recover(handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

/// 从header中提取‘div-by’获得除数，否则以DivideByZero结果reject请求。
fn div_by() -> impl Filter<Extract = (NonZeroU16,), Error = Rejection> + Copy {
    warp::header::<u16>("div-by").and_then(|n: u16| async move {
        if let Some(denom) = NonZeroU16::new(n) {
            Ok(denom)
        } else {
            Err(reject::custom(DivideByZero))
        }
    })
}

/// 问题：因为需要解析body中的json数据，就需要定义struct？
#[derive(Deserialize)]
struct DenomRequest {
    pub denom: NonZeroU16,
}

/// 定义struct来支持实现rejection错误信息。示例：
/// ```
/// struct DivideByZero;
/// impl reject::Reject for DivideByZero {}
/// ```
#[derive(Debug)]
struct DivideByZero;
impl reject::Reject for DivideByZero {}

// JSON replies

/// struct Math定义了除法的内容和商。
#[derive(Serialize)]
struct Math {
    op: String,
    output: u16,
}

/// struct ErrorMessage定义了返回错误的json数据包。
#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

/// 接收并处理不同的rejection，尝试返回个性化的内容和代码。否则，进一步 将rejection传递出去。
async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else if let Some(DivideByZero) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = "DIVIDE_BY_ZERO";
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        // This error happens if the body could not be deserialized correctly
        // We can use the cause to analyze the error and customize the error message
        message = match e.source() {
            Some(cause) => {
                if cause.to_string().contains("denom") {
                    "FIELD_ERROR: denom"
                } else {
                    "BAD_REQUEST"
                }
            }
            None => "BAD_REQUEST",
        };
        code = StatusCode::BAD_REQUEST;
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        // We can handle a specific error, here METHOD_NOT_ALLOWED,
        // and render it however we want
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED";
    } else {
        // We should have expected this... Just log and say its a 500
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}

