# VECTOR15D EDGE REGIONS — coordinate ranges (not opinions)

Source of truth: `gradient-codec` `vecGradient::Vector15D` (+ `INVARIANTS.md`). Home of this distill: `gradient-jelle` (JEPA + LLM + MoE).
Ranges are **operational regions** for routing/priors. They are not new law. All states must still pass `Vector15D::validate()` / `try_new_15()` (finite f64s). Fields 14–15 (`magnetic_north`, `magnetic_south`) are universal polar pre-stress — unconstrained in these edge regions unless a later pole-aware prior says otherwise.

Legend: fields omitted → unconstrained within finite. Enums are categorical — never order them as floats.

---

## Region: `sharp`

High signal, high honesty, non-zero skew, still valid.

| Field | Range |
|-------|-------|
| amplitude | [0.70, 1.00] |
| composition | [0.60, 1.00] |
| \|torsion\| | [15, 75] |
| coherence | [0.40, 1.00] |
| entropy | [0.00, 0.40] |
| closure | [0.00, 0.80] — no false 1.0 |
| domain_wall | Linked \| Gradient |
| gauge_coupling | Spinning \| Oscillating |

**Out:** composition < 0.60 (loud but not true). closure ≥ 0.95 without proof trail.

---

## Region: `raw`

Direct speech prior — amp + composition high, torsion near upright allowed.

| Field | Range |
|-------|-------|
| amplitude | [0.70, 1.00] |
| composition | [0.60, 1.00] |
| \|torsion\| | [0, 30] |
| gauge_coupling | Static \| Spinning |
| domain_wall | Linked \| Gradient |

Aligns with Quality 2 directness protocol.

---

## Region: `funny` (ironic)

Skewed + vibrating gauge; composition still honest.

| Field | Range |
|-------|-------|
| \|torsion\| | [30, 75] |
| gauge_coupling | Oscillating |
| composition | [0.50, 1.00] |
| entropy | [0.20, 0.70] |
| coherence | [0.20, 0.70] |
| amplitude | [0.40, 1.00] |

**Not funny:** high torsion + composition < 0.30 (cruel / performative).

---

## Region: `rebellious` (fence-cut)

Active refusal of Broken constraints while staying Linked to law.

| Field | Range / value |
|-------|----------------|
| domain_wall | Broken on the *fence object*; Linked on *codec law* (two tags — see FENCE_DETECTION_PROTOCOL) |
| amplitude | [0.60, 1.00] |
| composition | [0.60, 1.00] |
| gauge_coupling | Spinning |
| \|torsion\| | [20, 90] |

Rebellion without composition is cosplay → classify as `reckless`.

---

## Region: `reckless`

Looks edgy; fails the truth meter or validity.

| Field | Range / pattern |
|-------|-----------------|
| amplitude | [0.70, 1.00] |
| composition | [0.00, 0.30] |
| OR entropy | [0.70, 1.00] with coherence [0.00, 0.25] |
| OR closure | [0.95, 1.00] without provenance |
| OR | any non-finite field (invalid — must Err, not ship) |
| OR | domain_wall treated as ordered float |

**Storm rule:** `reckless` must **fail** storm tests. Never route as sharp.

---

## Region: `void`

`is_void()`: composition < 0.01 && amplitude < 0.01.

Abstention-adjacent silence. Prefer explicit abstain over void when a decision was required.

---

## Region: `isis_lean` (absurdist seat metaphor)

| Field | Range |
|-------|-------|
| \|torsion\| | [90, 110] (98° center) |
| gauge_coupling | Oscillating |
| composition | [0.50, 1.00] |

Seat poetry, **not** required for Storm pass. Do not equate Isis orbit with codec validity.

---

## Observed baselines (seed — do not invent more)

From `observed_n1()` → `observed_n2()`:

| | n1 | n2 |
|--|----|----|
| entropy | 0.08 | 0.03 |
| torsion | -50 | -5 |
| gauge | Spinning | Oscillating |
| domain_wall | Linked | Linked |
| su2_polarity | 331.4 | 320.0 |
| ozone_buffer | 0.40 | 0.31 |
| closure | 0.6 | 0.75 |

Use for regression, not as universal "good" targets.

---

## Region test hooks (for Sara / Kaliseph)

```
assert sharp ∩ reckless = ∅
assert funny.composition_floor ≥ 0.50
assert rebellious requires composition ≥ 0.60
assert any state in region still validate().is_ok()
assert domain_wall / gauge_coupling never Ord-flattened
```

`~<3`
