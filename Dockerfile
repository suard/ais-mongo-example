# Gebruik een officiële Rust-runtime als basisimage
FROM rust:1.84-alpine as builder

RUN apk add alpine-sdk openssl-dev

# Werkmap instellen
WORKDIR /usr/src/app

# Kopieer de Cargo.toml en Cargo.lock naar de container
COPY Cargo.toml .
COPY Cargo.lock .

# Maak een dummy main.rs om dependencies te cachen
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Bouw de dependencies (dit wordt gecached)
RUN cargo build --release

# Kopieer de rest van de broncode
COPY src .

# Bouw de applicatie
RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/ais-map /ais-map

CMD ["/ais-map"]