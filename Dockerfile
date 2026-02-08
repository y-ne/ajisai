# Builder
FROM rust:1.93-alpine3.23 AS builder

RUN apk add --no-cache musl-dev

WORKDIR /app
COPY . .

ENV SQLX_OFFLINE=true
RUN cargo build --release

# Runtime
FROM alpine:3.23

RUN apk add --no-cache ca-certificates

COPY --from=builder /app/target/release/ajisai /usr/local/bin/

EXPOSE 3000
CMD ["ajisai"]
