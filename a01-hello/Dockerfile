FROM rust:1.49.0 as builder
RUN apt-get update && apt-get upgrade -y
WORKDIR /app
ADD . /app
RUN rustup target add armv7-unknown-linux-musleabihf
RUN USER=root CARGO_HTTP_MULTIPLEXING=false cargo build --release \
    --target armv7-unknown-linux-musleabihf

FROM scratch 
COPY --from=builder /app/target/armv7-unknown-linux-musleabihf/release/a01-hello /app 
EXPOSE 3030
CMD ["/app"]


