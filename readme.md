## Introduction

### Ajisai, the Hydrangea Macrophylla

`axum` `sqlx` `postgresql`

## Manual

```bash
cargo install sqlx-cli
```

```bash
cp .env.example .env
```

```bash
sqlx database create

sqlx migrate run
```

```bash
cargo sqlx prepare

cargo clean

cargo build

cargo run
```

```bash
docker build -t ajisai .

docker run -p 3000:3000 ajisai
```
