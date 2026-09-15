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

## License and permitted use

The software is licensed under the [PolyForm Noncommercial License 1.0.0](LICENSE).

Individuals, hobbyists, students, independent researchers, educators, nonprofits, and community projects are welcome to use, modify, and share the software for noncommercial purposes. See [USE-POLICY.md](USE-POLICY.md).

Commercial use requires a separate written license. See [COMMERCIAL.md](COMMERCIAL.md).
