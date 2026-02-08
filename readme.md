## Ajisai, the Hydrangea Macrophylla

`axum` `sqlx` `postgresql`

### Setup

```bash
cp .env.example .env
```

### Database

```bash
sqlx database create
sqlx migrate run
```

### Development

```bash
cargo run
```

### Build

```bash
cargo sqlx prepare
docker build --platform linux/amd64 -t ghcr.io/y-ne/ajisai:latest .
docker push ghcr.io/y-ne/ajisai:latest
```
