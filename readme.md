## Ajisai, the Hydrangea Macrophylla

`axum` `sqlx` `postgresql`

`Ehhh? If you compliment me so suddenly, I'll have no choice but to smile, you know?`

Ajisai is defined by her profound kindness and affectionate personality. She is universally gentle, earning the trust and admiration of everyone around her, including the more cynical Koto Satsuki. Amaori Renako holds her in particularly high regard, often referring to her as an "angel."

Her seemingly perfect nature leads some friends, like Renako and Kaho, to humorously speculate that she has a hidden dark side, which they nicknamed "Yamisai-san." While their suspicions are unfounded, Ajisai has her own moments of self-doubt, occasionally wondering if her kindness is a form of passivity.

Despite her patient demeanor, she experiences significant stress from looking after her energetic younger brothers, a rare emotional vulnerability she seldom shows to others. Her affection for Renako is sincere and deep, and her actions are often driven by her romantic feelings.

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
