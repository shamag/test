FROM rust:1.61.0 as builder
WORKDIR /usr/src/myapp
COPY src src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN cargo build --release
 
FROM debian:buster-slim
RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/myapp/target/release/poem-fib /usr/local/bin/myapp
CMD ["myapp"]