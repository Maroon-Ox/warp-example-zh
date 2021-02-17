FROM rust:1.50 as builder
WORKDIR /app
ADD . /app
RUN rustup target add armv7-unknown-linux-musleabi
RUN USER=root CARGO_HTTP_MULTIPLEXING=false cargo build --release \
    --target armv7-unknown-linux-musleabi

FROM scratch 
COPY --from=builder /app/target/armv7-unknown-linux-musleabi/release/a01-hello /app 
EXPOSE 3030
CMD ["/app"]

