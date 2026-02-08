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

sqlx migrate info

sqlx migrate run
```

```bash
cargo sqlx prepare

cargo clean

cargo build

cargo run
```

```bash
# MacOS need to specify the --platform
docker build --platform linux/amd64 -t ajisai .

docker run -p 3000:3000 ajisai
```
