# Use the official Rust image as a base
FROM rust:latest as builder

WORKDIR /usr/src/otus-hla-hw

COPY ../../Cargo.toml ./
COPY ../../Cargo.lock ./
COPY ../../src ./src
COPY cfg.yaml ./
COPY ../../sqlx-data.json ./

RUN apt install -y libssl-dev
RUN cargo build --release

RUN cargo install --path .

FROM debian:buster-slim

WORKDIR /usr/local/bin/

COPY --from=builder /usr/local/cargo/bin/otus-hla-hw .

CMD ["./otus-hla-hw", "--config", "/usr/src/otus-hla-hw/cfg.yaml"]
