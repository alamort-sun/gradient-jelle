# DEPENDENCIES

> Third-party terms are never relabeled. Original terms, provenance, and version are recorded here.

## gradient-codec (in-ecosystem)

- Source: https://github.com/alamort-sun/gradient-codec (PolyForm Noncommercial policy family)

## Canonical pin

- **Codec authority commit: `359d00c`** (Seal plane split: router bin ≠ vecGradient; README/DEPENDENCIES honesty + fail-whens — current `origin/main` of `alamort-sun/gradient-codec`).
- **Path-dep reality:** `Cargo.toml` uses `vecGradient = { path = "../gradient-codec/vecGradient" }`. That tracks the **local tree**, not crates.io. The SHA above is the documented remote HEAD this tree is expected to match after the plane-split seal push; re-verify with `git -C ../gradient-codec rev-parse HEAD` and `cargo test` here before trusting a newer pin.
- Historical anchor: `c8d0ee1` (feat: universal magnetic poles — origin of fields 14–15) is the semantic origin; prior documented pin `057584c` (package rename) is superseded by `359d00c`.
- Provides: `Vector15D` (type alias `Vector13D`; fields 14–15 `magnetic_north`/`magnetic_south`), `DomainWall`/`GaugeCoupling` enums, `observed_n1/n2` baselines, `validate()`/`try_new_15()`, schema `v15d` (see `vecGradient::lib.rs`). Geometry lives in package `vecGradient` / bin `v15d` — **not** the `gradient-codec` router bin.
- Note: types at this boundary must stay in sync with the pinned commit; any drift is a law-break in the jelle → codec → space-time chain.

## Cross-core contract (as of `359d00c` codec / local jelle HEAD / alamort outer space-time)

- `gradient-jelle` path-depends on `../gradient-codec/vecGradient`.
- `gradient-space-time` re-exports `DomainWall`/`GaugeCoupling` from `vecGradient` (no local re-definition — single source of truth, see `types.rs`).
- All three cores must share this single codec pin. Re-pin only when codec `origin/main` advances; re-verify in every consuming repo.

## Third-party (direct `Cargo.toml` deps)

Direct dependencies from this package's `Cargo.toml` (versions are Cargo reqs; consult `Cargo.lock` when present for exact resolves). Licenses below are the crates' declared SPDX on crates.io for these well-known crates — **transitive license tree not fully audited** (see TODO).

| Crate | Cargo.toml req | License (declared upstream) | Purpose |
|-------|----------------|----------------------------|---------|
| serde | `1` (+ derive) | MIT OR Apache-2.0 | Serialization |
| serde_json | `1` | MIT OR Apache-2.0 | JSON |
| thiserror | `1` | MIT OR Apache-2.0 | Error derives |
| vecGradient | path `../gradient-codec/vecGradient` @ documented SHA `359d00c` | PolyForm Noncommercial 1.0.0 | Codec geometry authority |

## TODO

- [ ] Record / verify transitive dependency licenses (unchecked beyond direct table above)

## Policy

- Software source in this repository: PolyForm Noncommercial 1.0.0 (see [LICENSE](LICENSE)). `Cargo.toml` `license` field must match: `PolyForm-Noncommercial-1.0.0`.
- Commercial use requires a separate written license (see [COMMERCIAL.md](COMMERCIAL.md)).
- Never record a third-party item under a license it did not come with.
