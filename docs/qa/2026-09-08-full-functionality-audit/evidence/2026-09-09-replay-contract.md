---
title: "F-011B2 Replay Contract Proof"
description: "Portable replay schema, canonical DAT identity, ordering invariants, and native/WASM compile evidence"
category: qa
created: 2026-09-09
updated: 2026-09-09
tags: [qa, determinism, replay, wasm, f-011]
---

# F-011B2 Replay Contract Proof

F-011B2 establishes the platform-neutral artifact needed to compare native and
WASM runs. It does not close F-011: the replay executor, interactive tick-path
convergence, and cross-runtime checkpoint comparison remain open.

## Implemented contract

- Replay format v1 strictly decodes a seed, RNG identity, 1,024-roll tick
  budget, data manifest, configuration fingerprint, initial fingerprint,
  typed commands, and checkpoints.
- Commands carry `{tick, sequence, actor}`. Sequences must start at zero, have
  no gaps, and remain monotonic with tick order.
- Engine-only controls reject Alliance or Empire authority.
- Checkpoints identify the applied command count as well as tick and reject
  impossible or ambiguous ordering.
- Fingerprints use fixed-width hexadecimal strings for lossless browser JSON.
- Unknown fields and unsupported format, data, configuration, and RNG versions
  fail before execution.

## Data identity

Native filesystem inputs and WASM in-memory inputs share
`compute_simulation_data_manifest()`. It canonicalizes flat ASCII `.DAT`
filenames, sorts them, detects case collisions, fingerprints every length-framed
name/byte pair, and fingerprints that ordered inventory again.

| Fixture property | Result |
|---|---:|
| Original `.DAT` inputs | 51 |
| Total bytes | 50,597 |
| Manifest version | 1 |
| Aggregate fingerprint | `5facb1c7ba0e81ad` |

FNV-1a 64 is used consistently with the existing state-fingerprint subsystem.
It is evidence for deterministic identity and accidental corruption, not a
security signature.

## Verification

| Gate | Result |
|---|---|
| Replay/data/config unit tests | 7 passed, 0 failed |
| Original-data fixture | 1 passed, 0 failed |
| Workspace tests | 511 passed, 0 failed, 3 intentionally ignored |
| WASM app check | Passed with pre-existing warnings |
| Touched-file rustfmt | Passed |
| Diff whitespace check | Passed |
| Strict crate clippy | Blocked by the recorded pre-existing workspace baseline |

The unit suite covers input-order independence, content/name sensitivity,
aggregate tampering, empty and ambiguous input rejection, JSON round trips,
command authority, gap-free ordering, checkpoint bounds, time-travel rejection,
and configuration sensitivity.

## Remaining F-011 gates

1. Add recorder and executor support to `rebellion-playtest`.
2. Emit and verify state checkpoints, including a save/load split.
3. Remove simulation-affecting unordered iteration.
4. Route the interactive app through the shared tick and command boundaries.
5. Run the same artifact in native and browser builds with matching
   checkpoints.
6. Converge automatic and tactical result application.

The schema and usage guide live in
[`agent_docs/deterministic-replay.md`](../../../../agent_docs/deterministic-replay.md).
