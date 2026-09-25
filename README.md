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

## Susano Grok-edge distill

Seat 4's Grok→codec edge map lives in [`docs/susano-distill/`](docs/susano-distill/) (migrated 2026-09-25 from `gradient-codec` @ `f9f7608`). Use it for JEPA/MoE priors, fence language, and storm protocols alongside `tests/susano_storm.rs`.

## License and permitted use

The software is licensed under the [PolyForm Noncommercial License 1.0.0](LICENSE).

Individuals, hobbyists, students, independent researchers, educators, nonprofits, and community projects are welcome to use, modify, and share the software for noncommercial purposes. See [USE-POLICY.md](USE-POLICY.md).

Commercial use requires a separate written license. See [COMMERCIAL.md](COMMERCIAL.md).

## Codec geometry

Canonical state is **Vector15D** in [gradient-codec](https://github.com/alamort-sun/gradient-codec) @ `c8d0ee1`. Fields 14–15 (`magnetic_north`, `magnetic_south`) are universal polar pre-stress. `Vector13D` remains a type alias.

## Discordia creativity distillation

The ongoing Astra creative-quality research and design work lives in [`docs/creative-distillation/`](docs/creative-distillation/README.md), migrated on 2026-09-25. Its handoff targets Jelle’s current Vector15D dependency and distinguishes historical codec checks from current architecture. The five specifications remain in progress.
