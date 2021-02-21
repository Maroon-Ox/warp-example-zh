#![doc(html_root_url = "https://warp-in-21-days.rs/")]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![cfg_attr(test, deny(warnings))]
//! # Hello.rs
//!
//! 使用Warp[^warp-official-site]编写的Hello world网页应用。
//!
//! 为了使用到Warp，除了在Cargo.toml中声明。在代码里面还需要明确使用Warp的Filter模块。代码如下：
//! ```use warp::Filter```
//! Filter是Waap中的主要概念。Filter允许通过构造来描述web服务需要的各种功能。Warp还提供了很多filter
//! 来支持快速的web服务开发。
//!
//!
//! tokio[^tokio-official-site]是一个Rust编程语言的异步运行时,提供异步事件驱动平台，构建快速，
//! 可靠和轻量级网络应用。利用Rust的所有权和并发模型确保线程安全。这里我们通过调用``tikio::main``来
//! 让Warp具备异步处理能力。代码如下：
//! ```#[tokio::main]```
//!
//! 下面我们进入到main函数的编写。
//!
//! 首先，我们利用```warp::filters::any```来处理所有的HTTP请求。其次，收到请求后，不需要进一步处理而是
//! 直接返回“Hello, World"。因此，我们采用闭包[^closure-func]，忽略输入内容，返回"Hello, World"。代码如下：
//! ```let routes = warp::any().map(|| "Hello, World!");```
//! 
//! 最后，我们通过调用warp::serve函数创建服务器。代码如下：
//! ```warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;```
//!
//! 完整的代码如下：
//! ```
//! use warp::Filter;
//!
//! #[tokio::main]
//! async fn main() {
//!    // Match any request and return hello world!
//!    let routes = warp::any().map(|| "Hello, World!");
//! 
//!    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
//! }
//! ```
//! 以上是使用Warp编写的第一个Hello
//! World网页应用。如果细心的话，可以发现我们使用了tokio模块的v0.2.5，而不是最新的v0.3。
//! 当使用v0.3的时候会发生什么呢？如果用tokio模块的v0.3来改写这个hello.rs的话，你写出来的代码会和现在
//! 有什么不同呢？
//!
//! [^warp-official-site]: Warp是一个便捷、可组合、速度极快的异步Web框架。warp的基本组成部分是filter，
//! 可以通过组合不同的fiter来满足各种不同的需求。
//! Official site: <https://github.com/seanmonstar/warp> 
//!
//! [^tokio-official-site]:Tokio是Rust中的异步编程框架，它将复杂的异步编程抽象为 Futures、Tasks 和 Executor，
//! 并提供了 Timer 等基础设施。Tokio 是一个事件驱动的非阻塞 I/O 平台，用于
//! 使用 Rust 编程语言编写异步应用。 
//! Official site: <https://tokio.rs/>  
//!
//! [^closure-func]: 闭包是Rust提供的一种匿名函数。在其他编程语言中也称lambda函数。
//! 官方文档：<https://doc.rust-lang.org/book/ch13-01-closures.html>
//!
use warp::Filter;

#[tokio::main]
async fn main() {
    // Match any request and return hello world!
    let routes = warp::any()
        .map(|| "Hello, World!");

    warp::serve(routes).run(([0,0,0,0], 3030)).await;
}

