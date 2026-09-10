---
title: "F-011B3 Replay Execution Proof"
description: "Recorder, executor, deterministic iteration fixes, save continuation, and native golden evidence"
category: qa
created: 2026-09-09
updated: 2026-09-09
tags: [qa, determinism, replay, native, wasm, f-011]
---

# F-011B3 Replay Execution Proof

F-011B3 makes the format-v1 artifact executable. It closes the native
recorder, playback, checkpoint-diagnostic, and save-continuation tranche. It
does not yet prove native/WASM execution equality or converge the interactive
app, playtest, automatic-combat, and tactical-combat loops.

## Implemented contract

- `record_replay()` derives ticks and gap-free sequences from runtime state,
  applies typed commands, and fingerprints `SaveState` after each command.
- `execute_replay()` independently validates engine, seed, all 51 simulation
  DAT inputs, active configuration, initial state, command positions, and every
  checkpoint.
- Playback stops on the first checkpoint mismatch and reports its index, tick,
  command prefix, artifact fingerprint, and runtime fingerprint.
- Format v1 consumes exactly 1,024 Xoshiro256++ values per simulated tick and
  rejects a single advance larger than 1,000,000 ticks.
- The runtime executes speed, dual-AI, visibility, victory-check, and tick
  commands through `run_simulation_tick()`.

## Deterministic-order correction

The initial real-data gate exposed two valid native executions that diverged
at tick 20. The cause was state-affecting iteration over process-randomized
maps and sets. F-011B3 now orders:

- manufacturing queues by `SystemKey` before applying completions;
- movement orders by `FleetKey` before applying simultaneous arrivals;
- started and cleared blockades by `SystemKey`; and
- equal-count AI reinforcement choices by `SystemKey`.

Three focused regression tests prove the manufacturing, movement, and blockade
event order even when entries are inserted in reverse order.

## Original-data golden

The ignored fixture loads the actual 200-system campaign with seed 42 and the
canonical 51-file data manifest. It records nine commands over 25 ticks,
persists the initial state through save v11, reloads it, and checks all nine
command-prefix checkpoints during playback.

| Prefix | Tick | Fingerprint |
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

The initial fingerprint is `v1:765c9318acb5cb50`. Five fresh native test
processes originally established this gate. F-007A then intentionally changed
tick-15 onward state by preventing active-fleet redispatch; the same reviewed
command stream now ends at `v1:8fbffe578f9319e4` and passes the native fixture.

## Verification

| Gate | Result |
|---|---|
| Workspace tests | 518 passed, 0 failed, 4 intentionally ignored |
| Focused replay unit tests | 11 passed, 0 failed |
| Original-data fixtures | 2 passed, 0 failed |
| Fresh native golden processes | 5 passed with identical checkpoints |
| Supported WASM app check | Passed with pre-existing warnings |
| Core/data clippy | Passed with documented baseline lint exemptions |
| Go UI extractor | Passed |

The repository-wide format and strict-clippy baselines remain open audit work;
this tranche introduces no new warning in the production replay path.

## Remaining gate

F-011B4 must execute one serialized artifact in native and WASM and compare
every checkpoint. F-012 must then route interactive and playtest execution
through one authoritative tick/event path before automatic and tactical combat
can join the same equivalence proof.
