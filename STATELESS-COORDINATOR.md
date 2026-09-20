# Stateless / Amnesiac Coordinator (v1.3) — gradient-jelle note

Companion to the canonical ADR: ../gradient-codec/docs/STATELESS-COORDINATOR.md.
This note adds only gradient-jelle’s own-core finding. Do not maintain a second
copy of the analysis — that is the whole point of the ADR.

## gradient-jelle finding (this core)

Audit of gradient-jelle/src and Cargo.toml (commit 003e9ba):
- Dependencies: serde_json, thiserror, path-dep vecGradient. Nothing else.
- Surface: no std::fs, std::net, std::process, no HTTP/network client, no
  filesystem reads or writes, no executor spawning. jelle is purely in-process.
- Conclusion: jelle does NOT read or write a coordinator, a plane, or any
  durable store. It is already "isolated" per spec section 5.3.1 by construction.
  No code change is required or sensible here for the v1.3 alignment.

## Spec jelle stories → what exists today
- C1 (map jelle-to-coordinator touchpoints): zero touchpoints — jelle has none.
- C2 (capsule-local state layer): jelle has no capsule layer; this presupposes a
  capsule architecture that does not exist in these cores.
- C3/C4 (redirect jelle reads/writes away from coordinator): nothing to redirect;
  jelle performs no such I/O.
- C5 (shadow/canary jelle behavior): N/A until a coordinator exists.

These are **fenced, not built.** They become meaningful only once the v1.3
coordinator / capsule layer is specified and constructed separately, per the
canonical ADR’s "not built yet" register.

## Provenance
- Audited by Luna (Seat 13). See canonical ADR sections 2 and 6.
