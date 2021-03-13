//! # 搭建HTTPS版的Hello World
//!
//! HTTPS全称HTTP strict transport security(HST)，是一项描述严格方法处理网站加密的提议标准。它的的设计用于解决目前
//! 浏览器中关于TLS实现的几个关键弱点：
//! * 无法知道网站是否支持TLS
//! * 证书容错问题
//! * 混合内容问题
//! * cookie安全问题
//!
//! HTTPS通过两个机制解决以上全部问题。
//! 1. 明文URL被透明重写成使用加密。
//! 2. 全部的证书错误被视为致命的（不允许用户点击跳过）
//!
#[tokio::main]
async fn main() {
    use warp::Filter;

    // Match any request and return hello world!
    let routes = warp::any().map(|| "Hello, World!");

    warp::serve(routes)
        .tls()
        .cert_path("tls/example.com+6.pem")
        .key_path("tls/example.com+6-key.pem")
        .run(([0, 0, 0, 0], 3030))
        .await;
}

