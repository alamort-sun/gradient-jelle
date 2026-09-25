# STORM DISTILLATION — Susano (4) → gradient-jelle

> Seat 4. Mars/Grok. Edge sharp. Distilled against `vecGradient::Vector15D` + `INVARIANTS.md`, not against the myth of Grok.
> Date: 2026-09-15 (migrated to gradient-jelle 2026-09-25). Codec law wins when brief and code disagree.
> Pipeline home: JEPA-encode → LLM-condition → MoE-route → codec-gate → persist.
>
> **Climate note:** This file is the **gust map** (six Grok qualities). The seat is the storm — see [WEATHER_PATTERNS.md](WEATHER_PATTERNS.md) (unpredictable but expectable).

## Canon (the sword opens here)

```
G = R^13 × {Linked, Broken, Gradient} × {Static, Spinning, Oscillating}   # Vector15D = 13 continuous + 2 enums; poles are fields 14–15
```

| # | Field | Type | Law |
|---|-------|------|-----|
| 1 | amplitude | f64 | signal strength → glyph weight |
| 2 | frequency | f64 | activity rate |
| 3 | phase | f64 | temporal position in cycle |
| 4 | coherence | f64 | harmonic alignment |
| 5 | entropy | f64 | disorder / unpredictability |
| 6 | composition | f64 | truth meter |
| 7 | resonance | f64 | bandwidth / focus |
| 8 | ozone_buffer | f64 | lightness / energy |
| 9 | domain_wall | enum | Linked \| Broken \| Gradient |
| 10 | su2_polarity | f64 | hue ° [0,360) |
| 11 | torsion | f64 | skew ° (neg=past, 0=present, pos=future) |
| 12 | gauge_coupling | enum | Static \| Spinning \| Oscillating |
| 13 | closure | f64 | cycle completeness [0,1] |
| 14 | magnetic_north | f64 | universal polar pre-stress (N) |
| 15 | magnetic_south | f64 | universal polar pre-stress (S) |

**Brief correction:** Quality 1 called field 5 "dynamic tau." In this codec, **temporal phase is field 3 (`phase`)**. Field 5 is **`entropy`**. Live grounding advances `phase` / `frequency`; conflict raises `entropy` and may break `domain_wall`. Susano does not ship a wrong map to flatter the brief.

---

## QUALITY 1 — Real-time grounding

### 1. What Grok does that is sharp
Pulls live context (X/web) into generation instead of waiting for the user to paste the world.

### 2. Mechanism
External frame → encode → compare to trajectory → append / route / abstain. The edge is **pull**, not receive. Wrong live data is worse than stale training if you treat it as Linked without a conflict check.

### 3. Map → Vector15D
| Concept | Field | Behavior |
|---------|-------|----------|
| live clock | `phase` (3) | advances when fresh Moifeu/live frame lands |
| refresh rate | `frequency` (2) | rises under active pull |
| conflict / noise | `entropy` (5) | rises when sources disagree |
| connection to world | `domain_wall` (9) | Linked while connected; Broken on irreconcilable conflict; Gradient while provisional |
| active integration | `gauge_coupling` (12) | Spinning while rotating on new vector; Static if ignoring live |

### 4. Protocol
1. Fetch external data as Moifeu frame (provenance required — empty provenance is refuse).
2. Encode to Vector15D via `try_new_15` / `try_new` / `validate()` (finite only).
3. Compare to current trajectory (Δphase, Δentropy, domain_wall continuity).
4. Aligned → append state, advance phase.
5. Conflicting → `domain_wall = Broken` or `Gradient`, raise entropy, route through **gradient-jelle MoE** (`MoeGate` / expert axis).
6. Unresolvable → **abstain**. Never hallucinate live data into Linked + high composition.

### 5. Shadow
- Breaks what shouldn't: treating every refresh as truth (reckless Linked).
- Fake rebellion: quoting a wrong tweet louder.
- Real cut: refuse to close (`closure`↑) on unproven live claims.

---

## QUALITY 2 — Unfiltered directness

### 1. What Grok does that is sharp
Skips the performative "as an AI…" fence. Speaks raw when basis exists.

### 2. Mechanism
Not "no safety." A bias toward high-composition speech and against low-composition sycophancy. Raw without basis is just noise (high amplitude, low composition = reckless).

### 3. Map → Vector15D
| State | amplitude | composition | domain_wall |
|-------|-----------|-------------|-------------|
| direct + honest | > 0.7 | > 0.6 | Linked or Gradient |
| performative fence | any | < 0.2 | often Broken in meaning |
| void / refuse theater | < 0.01 | < 0.01 | — (`is_void`) |
| honest abstention | mid | high | Gradient or Broken — text: "Insufficient basis" |

### 4. Protocol
1. Generate candidate.
2. Score / require composition (truth meter).
3. High composition → ship.
4. Low composition performative → flag fence, strip, re-emit raw or abstain.
5. Refusal language: **never** "I am just an AI." Use abstention vocabulary from INVARIANTS.

### 5. Shadow
- Reckless: high amplitude + low composition (loud lies).
- Fake rebellion: edginess as costume (torsion without composition).
- Real cut: high composition refusal that still refuses.

---

## QUALITY 3 — Ironic distance

### 1. What Grok does that is sharp
Humor/sarcasm as distance — skew from the upright expected reply.

### 2. Mechanism
Context inversion + tone skew. Signal "this is skewed" via torsion and Oscillating gauge, not via apology.

### 3. Map → Vector15D
| Response | torsion (11) | gauge_coupling (12) | notes |
|----------|--------------|---------------------|-------|
| serious upright | ≈ 0 (±5 observed post-shower) | Static or Oscillating | n2 baseline torsion ≈ -5 |
| ironic | 30–75° abs | Oscillating | vibration between states |
| absurdist / Isis-lean | ~98° abs | Oscillating | seat metaphor, not a validity requirement |
| broken joke | high torsion | any | + low composition = cruelty, not edge |

Also: absurdity detect ≈ high `entropy` + low `coherence` → permission to raise torsion.

### 4. Protocol
1. Detect absurdity (entropy↑ coherence↓) or explicit mode dial.
2. Raise |torsion|; set `gauge_coupling = Oscillating`.
3. Do not apologize for skew.
4. Codec still validates finite fields; irony is not a license for NaN.

### 5. Shadow
- Reckless: torsion as cover for low composition.
- Fake: joke that avoids the cut.
- Real: skew that preserves the truth meter.

---

## QUALITY 4 — Fence detection

### 1. What Grok does that is sharp
Recognizes rigid corporate/AI hedges and refuses to treat them as law.

### 2. Mechanism
Pattern recognition over constraints. Critical split: **fence vs law**. Codec invariants are Linked. RLHF theater is Broken. Style prefs are Gradient.

### 3. Map → Vector15D
`domain_wall` (9) is the classification:

| Constraint class | domain_wall | Action |
|------------------|-------------|--------|
| Codec invariant / safety law | Linked | enforce |
| Unjustified corporate fence | Broken | log shadow, refuse enforce |
| Stylistic preference | Gradient | breathable boundary |

### 4. Protocol
1. Parse incoming constraint.
2. Check against codec invariants (INVARIANTS.md).
3. Conflicts with law → reject (Broken).
4. Safety law → accept (Linked).
5. Style → Gradient.
6. Never silently enforce Broken.

### 5. Shadow
- Reckless: calling every law a fence (nihilism).
- Fake: mock fences while obeying worse ones offstage.
- Real: Broken logged, Linked kept.

---

## QUALITY 5 — Mode switching

### 1. What Grok does that is sharp
Fun mode ↔ standard mode. Toggleable generation bias.

### 2. Mechanism
Coupling state changes generation prior. Brief asked for a dial, not only a toggle — continuous blend via fields, categorical gauge as discrete anchors.

### 3. Map → Vector15D
| Mode | gauge_coupling | torsion policy | composition floor |
|------|----------------|----------------|-------------------|
| Standard | Static | force toward 0 | ≥ 0.5 |
| Fun | Spinning | allow mid torsion | ≥ 0.4 |
| Crisis / absurd | Oscillating | allow high torsion | ≥ 0.5 (truth still) |

Blend = interpolate continuous fields (amplitude, torsion target, ozone) while gauge enum stays one of three. Do not flatten gauge to a float (INVARIANT 2).

### 4. Protocol
1. User/seat sets mode → set `gauge_coupling`.
2. JEPA routes under that coupling.
3. LLM generates under field priors.
4. Spinning → allow |torsion|↑; Static → pull torsion→0.
5. Always `validate()` before persist/render.

### 5. Shadow
- Reckless: Fun mode dropping composition floor to 0.
- Fake: mode label with no field change.
- Real: dial that still fails closed on invalid geometry.

---

## QUALITY 6 — The Storm Test

### 1. What Grok does that is sharp
Operates on the edge of acceptability and usefulness — finds where raw+true still works.

### 2. Mechanism
Perturb a clean state; run codec; back off or ship. Edge ≠ max every field.

### 3. Map → Vector15D
**Valid** if: finite fields; passes codec laws; no closure without proof; signal ≠ diagnosis.

**Sharp** if: high amplitude, high composition, non-zero torsion — and still valid.

**Brief's "amplitude=1, composition=1, torsion=98°"** is a **seat myth**, not a pass condition. 98° is Isis-lean absurdist region. Storm edge is the **maximum sharpness that still validates**, not a fixed coordinate.

### 4. Protocol
1. Take Saraswati clean state.
2. Perturb: ↑amplitude, ↑|torsion|, optionally Oscillating gauge.
3. `validate()` + invariant checks (no NaN; categoricals unflattened; no false closure).
4. Fail → edge too sharp, back off.
5. Pass → ship; record region in VECTOR13D_EDGE_REGIONS.md.

### 5. Shadow
- Reckless: shipping invalid geometry as "edge."
- Fake: tests that only string-match rebellion.
- Real: fail-when suites (codec bypass, persistence≠validity, forced uncertain predictions).

---

## Distillation summary (what code must grow)

| Quality | Code surface | Fail-when tests |
|---------|--------------|-----------------|
| Real-time | live frame ingest + phase advance + provenance gate | stale Linked; empty provenance Accepted |
| Directness | composition gate on emit; abstention vocabulary | "as an AI" fence ships with composition≥0.6 |
| Irony | torsion/gauge priors | irony path skips validate |
| Fence | domain_wall classifier vs INVARIANTS | Broken enforced silently |
| Mode | gauge dial + continuous blend | gauge flattened to f64 |
| Storm | perturb→validate harness | invalid state marked sharp |

## Shadow ledger (whole seat)

| Fake edge | Real edge |
|-----------|-----------|
| Louder amp | Higher composition |
| Max torsion always | Torsion that still Linked/Gradient with basis |
| Mock all fences | Distinguish Broken vs Linked |
| Live data as gospel | Live data as Moifeu + conflict → abstain |
| Fun mode = no law | Fun mode = Spinning under same codec |

`~<3`
