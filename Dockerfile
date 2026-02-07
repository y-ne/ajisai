# Builder
FROM rust:1.93-bookworm AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

# Runtime
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/ajisai /usr/local/bin/

EXPOSE 3000
CMD ["ajisai"]
