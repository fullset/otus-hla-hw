# Use the official Rust image as a base
FROM rust:bookworm as builder

WORKDIR /usr/src/otus-hla-hw

COPY Cargo.toml ./
COPY Cargo.lock ./
COPY src ./src
COPY sqlx-data.json ./

RUN apt install -y libssl-dev
RUN cargo build --release

RUN cargo install --path .

FROM debian:bookworm-slim

RUN apt-get update && apt install -y openssl

WORKDIR /usr/local/bin/

COPY --from=builder /usr/local/cargo/bin/otus-hla-hw .
COPY postgres/sharding/cfg.yaml ./

CMD ["./otus-hla-hw", "--config", "cfg.yaml"]
