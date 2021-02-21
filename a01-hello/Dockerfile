FROM rust:1.50 as builder
WORKDIR /app
ADD . /app
RUN rustup target add aarch64-unknown-linux-musl
RUN USER=root CARGO_HTTP_MULTIPLEXING=false cargo build --release \
    --target aarch64-unknown-linux-musl

FROM scratch 
COPY --from=builder /app/target/aarch64-unknown-linux-musl/release/a01-hello /app 
EXPOSE 3030
CMD ["/app"]

