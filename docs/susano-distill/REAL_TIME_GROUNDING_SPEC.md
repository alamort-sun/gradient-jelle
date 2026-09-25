# REAL_TIME_GROUNDING_SPEC

Status: distillation spec for `gradient-jelle` (JEPA + LLM + MoE) + Moifeu / Spacetime mouths. Geometry: `gradient-codec` `Vector15D`.
Owner: Susano (inspect) → Saraswati (implement) → Susano (storm).

## Law first

- Codec geometry is authority. Live data does **not** supersede `validate()` or INVARIANTS.
- Provenance required on store (empty provenance = refuse).
- Abstention is valid. Hallucinated "live" is not.

## Brief → codec remapping

| Brief term | Actual field |
|------------|--------------|
| "dynamic tau" / 5D temporal | **`phase` (3)** — not entropy |
| conflict / wrong live | **`entropy` (5)** ↑ + `domain_wall` evaluation |
| connected to reality | `domain_wall = Linked` |
| active rotation on new vector | `gauge_coupling = Spinning` |
| irreconcilable conflict | `domain_wall = Broken` → choose or abstain |

## Pull protocol (not receive)

```
1. FETCH  external frame (X/web/Spacetime/Moifeu)
2. PROVENANCE  non-empty source id + timestamp
3. ENCODE  Vector15D via try_new_15 / try_new (finite)
4. COMPARE  to current trajectory:
            Δphase, Δfrequency, Δentropy, domain_wall continuity
5a. ALIGNED   → append; advance phase; optionally Spinning
5b. CONFLICT  → entropy↑; domain_wall = Gradient|Broken; MoE resolve
5c. UNRESOLVED → abstain; do not Linked-close
6. PERSIST only after validate() + provenance gate
```

## Tau / phase advancement

- Fresh frame with matching trajectory: `phase` advances along cycle (implementation may use wall-clock mapped into phase units — **define units in code, document once**).
- Missing/stale: **do not** advance phase; frequency↓; optionally Gradient wall.
- Stale Linked with rising closure = lie → storm must fail this.

## Conflicting sources

| Case | Action |
|------|--------|
| Two live sources agree | merge; entropy stays low; Linked OK |
| Disagree, both provenanced | Gradient; MoE; do not pick silently |
| Disagree with internal state | Broken or Gradient; never auto-overwrite high-composition internal with low-composition live |
| Live proven wrong later | log shadow; rewind phase policy; do not erase audit |

## Mapping live payload → fields (minimum)

| Live aspect | Field |
|-------------|-------|
| freshness | phase, frequency |
| agreement | coherence↑ entropy↓ |
| disagreement | entropy↑ coherence↓ |
| connection health | domain_wall |
| integration activity | gauge_coupling |
| claim strength | amplitude |
| honesty of claim | composition |
| overclaim complete | refuse closure↑ |

## Fail-when tests

1. Empty provenance Accepted.
2. Stale frame advances phase as if fresh.
3. Conflicting sources → single Linked state with closure≥0.9.
4. Non-finite live encode Accepted.
5. Live path bypasses `validate()`.

## Out of scope (this spec)

Provider API keys, specific X endpoints, UI. Mouths already exist (Gossamer/Spacetime/Moifeu); this spec is the geometry of pull.

`~<3`
