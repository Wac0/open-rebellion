---
title: "F-011B4 Native/WASM Replay Equivalence Proof"
description: "Exact replay artifact, nine checkpoint comparisons, fail-closed browser diagnostics, and normal-startup evidence"
category: qa
created: 2026-09-09
updated: 2026-09-09
tags: [qa, determinism, replay, native, wasm, browser, astra, f-011]
---

# F-011B4 Native/WASM Replay Equivalence Proof

F-011B4 proves replay-executor equivalence for one original-data campaign. It
does not yet prove that the interactive app, playtest runner, automatic combat,
and tactical combat share one authoritative engine path.

## Exact artifact contract

Both runtimes embed and decode
`crates/rebellion-data/tests/fixtures/replay_seed42_v1.json` without regenerating
expectations. The reviewed artifact is 13,482 bytes with SHA-256
`2bb6fc3eb117c6076d29ea518ce5fdf27032355a5ba372d150e91a3aea319ab7`.
It identifies 51 simulation DAT inputs totaling 50,597 bytes with aggregate
fingerprint `5facb1c7ba0e81ad`.

The native `replay-gate` binary and query-gated WASM runner independently load
and seed the 200-system campaign. Their reports echo the exact artifact text
and compare the initial state, every declared command prefix, and the final
state.

| Prefix | Tick | Native and WASM fingerprint |
|---:|---:|---|
| 1 | 0 | `v1:48f9a97ca6dec269` |
| 2 | 0 | `v1:357708f65600307b` |
| 3 | 5 | `v1:c3782c7f2ba38f9d` |
| 4 | 10 | `v1:772525cf399636af` |
| 5 | 15 | `v1:ef31005899c83de6` |
| 6 | 20 | `v1:cca303d469e957c8` |
| 7 | 25 | `v1:012c1b55b88248d6` |
| 8 | 25 | `v1:8fbffe578f9319e4` |
| 9 | 25 | `v1:8fbffe578f9319e4` |

The initial fingerprint is `v1:765c9318acb5cb50`. The final state is tick 25
with fingerprint `v1:8fbffe578f9319e4`. F-007A intentionally changed the
tick-15 onward checkpoints by rejecting active-fleet AI redispatch. The same
nine-command artifact profile was re-reviewed and the native/WASM gate passed.

## Browser boundary

`?replay-check=seed42-v1` activates the diagnostic runner before ordinary game
startup. It strictly parses `runtime.orpk`, computes the DAT manifest before
installing the WASM cache, strictly decodes `textstra.json`, executes the
fixture, and publishes `window.__openRebellionReplay`. It then remains in a
terminal diagnostic frame loop.

The repeatable checker also proves:

- an unknown replay query fails in the `query` phase after three requests;
- an unavailable runtime pack fails in `runtime_pack_fetch`;
- neither failure retains initial, checkpoint, or final replay state;
- normal startup does not create a replay report;
- normal startup still makes exactly four requests and reaches the semantic
  bitmap main menu; and
- success, failure, and normal-startup cases report zero browser errors.

## Verification

```bash
cargo test --workspace
cargo test -p rebellion-data --test replay_manifest -- --ignored
cargo check -p rebellion-app --target wasm32-unknown-unknown
python3 scripts/check-replay-equivalence.py --skip-build --json
go test ./...
bash scripts/package-web.sh
```

| Gate | Result |
|---|---|
| Workspace unit tests | 524 passed, 0 failed, 4 intentionally ignored |
| Focused replay tests | 14 passed, 0 failed |
| Original-data fixtures | 2 passed, 0 failed |
| Exact artifact text | Native = WASM = committed 13,482 bytes |
| Cross-runtime checkpoints | 9 matched |
| Browser success requests | 4 |
| Normal startup requests | 4 |
| Browser errors | 0 |
| Invalid query | Failed closed in `query` |
| Missing runtime pack | Failed closed in `runtime_pack_fetch` |
| Go UI extractor | Passed |
| Web package | Passed with hash manifest |

The final staged WASM is 4,934,894 bytes with SHA-256
`04e5137b4b4be2bd14b6ef2d2748a21bf118cf0e36e35a97e59f4f83fd0175d7`.
The runtime pack remains 29,096,058 bytes with SHA-256
`6123deec14cfbfd070573c7001ef1123bd52067273498ad0c36cb7fdb108f509`.

GPT-6 Astra at medium effort independently inspected the implementation and
ran the browser checker. It returned PASS with no P0 or P1 findings. Its two
nonblocking harness recommendations, server ownership validation and complete
empty-state assertions for failures, were incorporated before the final pass.

Strict repository-wide clippy still fails on documented pre-existing findings
outside this tranche. `rebellion-data` passes after those baseline lint classes
are explicitly exempted; the new replay files add no strict-clippy finding.

## Remaining boundary

F-012 must converge app and playtest commands, simulation ticks, events, and
combat state. M1 must then extend the proof across five 5,000-tick seeds. This
fixture is a precise regression gate, not a full-game or multiplayer parity
claim.
