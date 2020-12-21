#![doc(html_root_url = "https://warp-in-21-days.rs/")]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![cfg_attr(test, deny(warnings))]
//! # Hello.rs
//!
//! 使用Warp编写的Hello world网页应用。
//!
//! 为了使用到Warp，除了在Cargo.toml中声明。在代码里面还需要明确使用Warp的Filter模块。代码如下：
//! ``use warp::Filter``
//!
//!
//! tokio是一个Rust编程语言的异步运行时,提供异步事件驱动平台，构建快速，
//! 可靠和轻量级网络应用。利用Rust的所有权和并发模型确保线程安全。这里我们通过调用``tikio::main``来
//! 让Warp具备异步处理能力。代码如下：
//! ``#[tokio::main]``
//!
//! 下面我们进入到main函数的编写。
//! ``use warp::Filter;``
//!
//! ``#[tokio::main]``
//!
//! ``async fn main() {``
//!
//! ``   // Match any request and return hello world!``
//!
//! ``   let routes = warp::any().map(|| "Hello, World!");``
//!
//! 
//! ``   warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;``
//!
//! ``}``
//! 
//! 以上是使用Warp编写的第一个Hello
//! World网页应用。如果细心的话，可以发现我们使用了tokio模块的v0.2.5，而不是最新的v0.3。
//! 当使用v0.3的时候会发生什么呢？如果用tokio模块的v0.3来改写这个hello.rs的话，你写出来的代码会和现在
//! 有什么不同呢？
//!
use warp::Filter;

#[tokio::main]
async fn main() {
    // Match any request and return hello world!
    let routes = warp::any().map(|| "Hello, World!");

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
