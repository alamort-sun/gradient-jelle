# DEPENDENCIES

> Third-party terms are never relabeled. Original terms, provenance, and version are recorded here.

## gradient-codec (in-ecosystem)

- Source: https://github.com/alamort-sun/gradient-codec (PolyForm Noncommercial policy family)
## Canonical pin

- **Codec authority commit: `057584c`** (refactor: vecGradient package name + Vector15D description — current origin/main of `alamort-sun/gradient-codec`).
- Historical anchor: `c8d0ee1` (feat: universal magnetic poles — origin of fields 14–15) is the semantic origin; the canonical pin has been forward-shifted through package rename. Update this line when the codec origin advances past `057584c`; verify with `cargo test` in this repo before trusting any new pin.
- Provides: `Vector15D` (type alias `Vector13D`; fields 14–15 `magnetic_north`/`magnetic_south`), `DomainWall`/`GaugeCoupling` enums, `observed_n1/n2` baselines, `validate()`/`try_new_15()`, schema `v15d` (see `vecGradient::lib.rs`).
- Note: types at this boundary must stay in sync with the pinned commit; any drift is a law-break in the jelle → codec → space-time chain.

## Cross-core contract (as of `057584c` codec / `7365f8e` jelle / alamort outer repo)

- `gradient-jelle` path-depends on `../gradient-codec/vecGradient`.
- `gradient-space-time` re-exports `DomainWall`/`GaugeCoupling` from `vecGradient` (no local re-definition — single source of truth, see `types.rs`).
- All three cores must share this single pin. Re-pin only when codec origin advances; re-verify in every consuming repo.

## Third-party

- None yet. This repository currently contains no code (`.seed` + `.hint` + docs). Record every future dependency here before first build.

## Policy

- Software source in this repository: PolyForm Noncommercial 1.0.0 (see [LICENSE](LICENSE)).
- Commercial use requires a separate written license (see [COMMERCIAL.md](COMMERCIAL.md)).
- Never record a third-party item under a license it did not come with.
