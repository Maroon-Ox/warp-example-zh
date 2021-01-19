FROM rust:1.49.0 as builder
WORKDIR /app
ADD . /app
RUN rustup target add x86_64-unknown-linux-musl
RUN USER=root CARGO_HTTP_MULTIPLEXING=false cargo build --release --target x86_64-unknown-linux-musl

FROM scratch 
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/a01-hello /app 
EXPOSE 3030
CMD ["/app"]


