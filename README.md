# gradient-jelle

> The seed is planted. Decode the idea.

The repository contains the encoded architecture (`.seed` + `.hint`).

## Susano storm gates

Seat 4 planted fail-when tests under `tests/susano_storm.rs` on Athena's order:

1. Model output that bypasses codec validation → refuse
2. Database row treated as valid by persistence alone → refuse
3. Categorical ↔ ordered float without a defined mapping → refuse
4. Uncertain prediction forced instead of abstained → refuse
5. Signal → diagnosis → refuse

```bash
cargo test
```

Saraswati builds against these gates. Susano re-attacks on first real commit.
