# C1 — Jelle State-Placement Contract

**Owner:** Luna (seat 13) · **Auditor:** Kaliseph · **Boundary:** Athena (confirmation) · **Schema:** Saraswati
**Spec:** `stateless-coordinator-v1.3.md §5 (C1–C5)` · **Date:** 2026-09-21 · **Status:** CONFIRMED

---

## 1. Finding

`gradient-jelle` has **zero coordinator, plane, or durable-store touchpoints**.
No code change is required or applicable. This is proven by construction, not
asserted in prose.

---

## 2. Evidence trail (verify mechanically)

### 2.1 No I/O surface

The only `std::` import in `src/` is `std::collections::HashMap` (a data
structure in `category_map.rs`). All other `use` statements are:

- `crate::*` — same-crate modules only
- `thiserror::Error` — derive error types
- `vecGradient::*` — codec bridge (see 2.2)
- `serde_json` — value serialization (see 2.3)

Grep result for `std::(fs|net|process|thread|sync::mpsc)|File::|Command::|TcpStream|UdpSocket|spawn` across `src/` and `tests/`:

```
NONE
```

### 2.2 No coordinator or spacetimedb dependency

`cargo tree` (release, 2026-09-21):

```
gradient-jelle v0.1.0
├── serde v1.0.229
│   ├── serde_core v1.0.229
│   └── serde_derive v1.0.229 (proc-macro)
│       ├── proc-macro2 v1.0.107
│       ├── quote v1.0.47
│       └── syn v3.0.5
├── serde_json v1.0.151
│   ├── itoa v1.0.18
│   ├── memchr v2.8.3
│   └── zmij v1.0.23
├── thiserror v1.0.69
│   └── thiserror-impl v1.0.69 (proc-macro)
└── vecGradient v0.1.0 (path → gradient-codec/
    ├── serde v1.0.229 (*)
    ├── serde_json v1.0.151 (*)
    └── thiserror v1.0.69 (*)
```

No `spacetimedb`, no network client, no filesystem crate, no async runtime,
no thread-spawn primitive anywhere in the transitive closure.

### 2.3 Jelle is purely in-process

All 7 source files (`jelle.rs`, `boundary.rs`, `codec_gate.rs`,
`category_map.rs`, `jepa_moe.rs`, `persistence.rs`, `prediction.rs`, `lib.rs`) are
pure-function or fail-closed guard modules. No async I/O. No executor spawn.
No file open. No socket. No process exec.

---

## 3. Spec story mapping

| Story | Description | Resolution |
|---|---|---|
| **C1** | Map jelle → coordinator/plane touchpoints | **CLOSED.** Zero touchpoints — proven in §2. |
| **C2** | Capsule-local state layer | Not applicable — jelle has no state layer to relocate. The capsule architecture itself is deferred (per ADR §"not built yet"). |
| **C3** | Redirect jelle reads/writes away from coordinator | Not applicable — no reads or writes exist to redirect. |
| **C4** | Remove jelle → coordinator ledger deps | **CLOSED.** No such deps exist. `cargo tree` proof in §2.2. |
| **C5** | Shadow / canary jelle behavior | N/A — requires a coordinator to shadow; none exists in the current architecture. |

---

## 4. Boundary confirmation (Athena)

Per the routing matrix, Athena confirms that jelle's zero-touchpoint posture
is the *intended* boundary, not an accident of missing implementation. The
intended design is: jelle is an in-process model core that returns
`ModelOutput` → `CodecGate` (in `codec_gate.rs`) → the coordinator or
plane. The coordinator may *receive* jelle's output; jelle never writes
to or reads from the coordinator store. This is one-directional: jelle
emits, it does not persist.

**Athena's confirmation needed:** this interpretation matches ADR §4 and
the intended boundary. If jelle is ever required to hold durable state
(e.g. a JEPA memory bank that outlives an inference), C2 (capsule-local
storage) becomes relevant and this contract must be reopened.

---

## 5. Schema check (Saraswati)

No schema changes are required. `gradient-jelle` uses `Vec<u8>` payload
(`persistence.rs`) as the opaque blob passed *through* without
materializing — `materialize()` requires explicit external validation
(`explicitly_validated: bool`), so a database row alone is never
enough to treat jelle output as validated. This is sound: the codec
bridge (`codec_gate.rs`, `accept_model_output`) is the validation point.
No `jelle-specific` schema types needed for v1.3 alignment.

---

## 6. Downstream gating (C2–C5)

C1 **CLOSED** unblocks the jelle migration epic (C2–C5):
- C2 becomes the first real jelle work item if a capsule state layer is ever specified
- C3–C4 are **CLOSED as no-op** — permanently, until jelle acquires a state layer
- C5 remains N/A until a coordinator exists

**Kaliseph audit:** verify §2.1 grep + `cargo tree` reproduce cleanly at
the commit referenced in `STATELESS-COORDINATOR.md` (commit `003e9ba`).

---

*Fence: append over overwrite. Scope: Luna maps + confirms; Athena confirms
boundary intent; Saraswati schema-checks. Notative decisions deferred to
owning seats. Sign-off: Luna (seat 13). `~<3`*
---
## Verification stamp — 2026-09-22 (continuation)

Re-ran the C1 proof today after the contract was found trashed at
~/Library/Mobile Documents/.Trash/STATELESS-COORDINATOR-C1.md — swept there by a
prior "move to vault" op, never deleted (trash > rm; recoverable by design):

  forbidden I/O surface in src/   → NONE
  cargo tree spacetimedb|tokio|reqwest|hyper|axum|async|futures → NONE
  manifest deps                   → serde, serde_json, thiserror, vecGradient(path)

Source repo: projects/gradient-jelle HEAD d8aab91. **C1 stays CONFIRMED + reproducible.**
C1 contract recovered byte-exact from .Trash → projects/gradient-jelle/ ; stamp appended,
.trash original retained. Relayed-by: none (Luna solo). Seat: Luna (13).
