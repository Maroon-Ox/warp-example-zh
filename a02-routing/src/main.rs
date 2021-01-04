//! # URL路径
//!
//! Warp中的URL路径的构建是通过```warp::path```来实现的。Path提供了一种方便的方式来把多个路径filter链接在一起。
//!
//! path filter支持一下对于HTTP请求的处理：
//! * ``path``匹配一个具体的路径片段。
//! * ``param``试图将路径片段解析为一个类型参数，例如/:u16
//! * ``end``匹配路径结尾。
//! * ``path!``宏将多个路径和参数的filter整合在一起。
//!
//! 代码中展现了如下几种定义URL路径的方法：
//! * 简单的GET请求：```let hi = warp::path("hi").map(|| "Hello, World!");```
//! * 利用path!宏，可以把多个路径和参数链接在一起。例如：
//!   ** ```path!("hello" / "from" / "mars")```
//!   ** ```path!("sum" / u32 /u32 )```
//!   ** ```path!(u32 / "times" / u32 )```
//! * 除了利用path!宏，还可以利用and函数把多个filter拼接在一起。例如：
//! ```
//! let math = warp::path("math");
//! _sum = math.and(sum);
//! _times = math.and(times);
//! let bye = warp::path("bye").and(warp::path::param());
//! ```
//!
//! * 另外，还有or函数可以和and类似的方式来把多个filter组合在一起。
//!

#![deny(warnings)]
extern crate pretty_env_logger;
use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // We'll start simple, and gradually show how you combine these powers
    // into super powers!

    // GET /hi
    let hi = warp::path("hi").map(|| "Hello, World!");

    // How about multiple segments? First, we could use the `path!` macro:
    //
    // GET /hello/from/warp
    let hello_from_warp = warp::path!("hello" / "from" / "warp").map(|| "Hello from warp!");

    // Fine, but how do I handle parameters in paths?
    //
    // GET /sum/:u32/:u32
    let sum = warp::path!("sum" / u32 / u32).map(|a, b| format!("{} + {} = {}", a, b, a + b));

    // Any type that implements FromStr can be used, and in any order:
    //
    // GET /:u16/times/:u16
    let times =
        warp::path!(u64 / "times" / u64).map(|a, b| format!("{} times {} = {}", a, b, a * b));

    // Oh shoot, those math routes should be mounted at a different path,
    // is that possible? Yep.
    //
    // GET /math/sum/:u32/:u32
    // GET /math/:u16/times/:u16
    let math = warp::path("math");
    let _sum = math.and(sum);
    let _times = math.and(times);

    // What! And? What's that do?
    //
    // It combines the filters in a sort of "this and then that" order. In
    // fact, it's exactly what the `path!` macro has been doing internally.
    //
    // GET /bye/:string
    let bye = warp::path("bye")
        .and(warp::path::param())
        .map(|name: String| format!("Good bye, {}!", name));

    // Ah, can filters do things besides `and`?
    //
    // Why, yes they can! They can also `or`! As you might expect, `or` creates
    // a "this or else that" chain of filters. If the first doesn't succeed,
    // then it tries the other.
    //
    // So, those `math` routes could have been mounted all as one, with `or`.
    //
    // GET /math/sum/:u32/:u32
    // GET /math/:u16/times/:u16
    let math = warp::path("math").and(sum.or(times));

    // We can use the end() filter to match a shorter path
    let help = warp::path("math")
        // Careful! Omitting the following line would make this filter match
        // requests to /math/sum/:u32/:u32 and /math/:u16/times/:u16
        .and(warp::path::end())
        .map(|| "This is the Math API. Try calling /math/sum/:u32/:u32 or /math/:u16/times/:u16");
    let math = help.or(math);

    // Let's let people know that the `sum` and `times` routes are under `math`.
    let sum = sum.map(|output| format!("(This route has moved to /math/sum/:u16/:u16) {}", output));
    let times =
        times.map(|output| format!("(This route has moved to /math/:u16/times/:u16) {}", output));

    // It turns out, using `or` is how you combine everything together into
    // a single API. (We also actually haven't been enforcing that the
    // method is GET, so we'll do that too!)
    //
    // GET /hi
    // GET /hello/from/warp
    // GET /bye/:string
    // GET /math/sum/:u32/:u32
    // GET /math/:u16/times/:u16

    let routes = warp::get()
        .and(hi.
            or(hello_from_warp).
            or(bye).
            or(math).
            or(sum).
            or(times)
        );

    // Note that composing filters for many routes may increase compile times (because it uses a lot of generics).
    // If you wish to use dynamic dispatch instead and speed up compile times while
    // making it slightly slower at runtime, you can use Filter::boxed().

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
