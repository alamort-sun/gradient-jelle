# Migration — Susano distill → gradient-jelle

| | |
|--|--|
| **From** | `alamort-sun/gradient-codec` @ `f9f7608` — `docs/susano-distill/` |
| **To** | `alamort-sun/gradient-jelle` — `docs/susano-distill/` |
| **When** | 2026-09-25 (Seat 4, Susano) |
| **Why** | jelle is the JEPA + LLM + MoE project; edge priors and storm protocols belong with the orchestrator that routes them. Codec remains the geometry authority. |

## What moved (byte-faithful + retarget)

- `STORM_DISTILLATION.md` — title/home → jelle; canon table + encode calls → Vector15D (+ poles 14–15); MoE step names jelle `MoeGate`
- `VECTOR13D_EDGE_REGIONS.md` → **`VECTOR15D_EDGE_REGIONS.md`** (same regions; validate API updated)
- `REAL_TIME_GROUNDING_SPEC.md`, `FENCE_DETECTION_PROTOCOL.md`, `STORM_TEST_PROTOCOL.md` — Vector15D naming
- `README.md` — rewritten for jelle home

## What did **not** move

- Fail-when storm suite stays at `tests/susano_storm.rs` (already jelle-native)
- Codec `INVARIANTS.md` / `vecGradient` implementation stay in gradient-codec
- Artemis arrow distill (`docs/ARROW_DISTILLATION.md`) stays in gradient-codec

## Codec working tree note

`gradient-codec` HEAD already lacked `docs/susano-distill/` (only Artemis arrow docs remain). History keeps `f9f7608`. Breadcrumb added: `gradient-codec/docs/SUSANO_DISTILL_MOVED.md`.
