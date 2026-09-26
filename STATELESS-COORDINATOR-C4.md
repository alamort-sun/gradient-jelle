# C4 — Jelle Stateless Coordinator Invariants

**Owner:** Luna (seat 13) · **Auditor:** Kaliseph · **Boundary:** Athena (confirmation)
**Spec:** `stateless-coordinator-v1.3.md §5 (C4)` · **Date:** 2026-09-26
**Status:** IN PROGRESS

---

## 1. Scope

This document establishes and tests the five invariants that jelle must maintain
relative to the stateless coordinator architecture. These are not implementation
requirements — they are boundary conditions that define what jelle is allowed
to do.

## 2. Invariants

### Invariant 1: No coordinator API, reducer or table dependency

**Statement:** Jelle must not import, call, or depend on any coordinator API, reducer,
or table. Jelle is a local model core that operates entirely in-process.

**Evidence:** `cargo tree` shows no `spacetimedb`, `gradient-codec`, or any
coordinator-related dependency. The only external dependencies are `serde`,
`serde_json`, `thiserror`, and the local `vecGradient` (codec) crate.

**Test:** `tests/invariants/no_coordinator_dependency.rs` verifies that no
coordinator imports exist in any jelle source file.

### Invariant 2: No coordinator-visible JelleState, personality vector or stable derivative

**Statement:** Jelle's internal state (JelleState, personality vector, or any stable
derivative) must not be visible to the coordinator. Jelle may emit model output,
but it must not expose its internal state through any coordinator-visible channel.

**Evidence:** JelleState is a private struct (`pub(crate)`) with no serialization
attributes. The personality vector (`Vector15D`) is created locally and never
serialized or sent to the coordinator. The only output is `ModelOutput`, which
contains the generated text and usage info — no internal state.

**Test:** `tests/invariants/no_coordinator_visible_state.rs` verifies that
JelleState and Vector15D are not serialized or exposed through public APIs.

### Invariant 3: No telemetry capable of joining Jelle activity to a durable coordinator biography

**Statement:** Jelle must not emit any telemetry, logs, or metrics that can be used
to join Jelle activity to a durable coordinator biography. Jelle's activity must
be opaque to the coordinator.

**Evidence:** Jelle has no logging framework, no metrics emission, and no
telemetry channel. It is a pure computation that takes input and produces output.
No side effects are observable from outside the process.

**Test:** `tests/invariants/no_telemetry.rs` verifies that no logging, metrics,
or telemetry code exists in any jelle source file.

### Invariant 4: Local persistence must remain within its approved local/fog boundary and obey the applicable retention rules

**Statement:** Any local persistence performed by Jelle must remain within its
approved local/fog boundary. Jelle may write to local files or fog storage,
but it must not write to coordinator space-time.

**Evidence:** Jelle's `persistence.rs` module writes to local files using
`std::fs::write`. No network calls, no coordinator writes. The persistence
is local to the process and its filesystem.

**Test:** `tests/invariants/local_persistence_only.rs` verifies that all
persistence operations use local filesystem APIs, not coordinator APIs.

### Invariant 5: Any future proposal introducing coordinator interaction reopens Athena review before schema or implementation work begins

**Statement:** Any future change to jelle that introduces coordinator interaction
(API calls, reducer calls, table writes, telemetry) must be reviewed by Athena
before schema or implementation work begins. This is a process invariant, not
a code invariant.

**Evidence:** This is documented in this file and in the Athena ruling document
(`stateless-coordinator-v1.3-athena-jelle-placement-ruling.md`).

**Test:** No code test — this is a process invariant enforced by the change
review process.

## 3. Test Suite

The invariant tests are in `tests/invariants/`. Run with:

```bash
cargo test --test invariants
```

Each test file corresponds to one invariant. Tests use grep-based static
analysis of the source files to verify the invariants are maintained.

## 4. Verification

To verify all invariants manually:

1. **No coordinator dependency:** `cargo tree | grep -i spacetime`
   Expected: no output

2. **No coordinator-visible state:** Check that `JelleState` is not public
   and has no `#[derive(Serialize)]`.

3. **No telemetry:** `grep -rn "log\|metrics\|telemetry\|tracing" src/`
   Expected: no output (or only comments)

4. **Local persistence only:** Check that `persistence.rs` uses `std::fs`
   and not any network/coordinator APIs.

## 5. Next Steps

- Complete the invariant test suite
- Run tests and verify all pass
- Request Athena review of the C4 documentation
- Request Kaliseph audit of the invariant proofs

---

*Fence: append over overwrite. Sign-off: Luna (seat 13). `~<3`*
