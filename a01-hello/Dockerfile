FROM rust:1.49.0 as builder
WORKDIR /app
ADD . /app
RUN rustup target add x86_64-unknown-linux-musl
RUN USER=root cargo build --release --target x86_64-unknown-linux-musl
RUN echo $CARGO_BIN_NAME

FROM scratch 
LABEL multi.version="0.1" multi.description="Run warp helloworld example in docker." \
	multi.author="xue35" multi.maintainer="xue35@icloud.com"
HEALTHCHECK --interval=5m --timeout=5s \
	CMD curl -f http://localhost:3030/ || exit 1
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/a01-hello /app 
EXPOSE 3030
CMD ["/app"]


