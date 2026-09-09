---
title: "Deterministic Save Continuation Proof"
description: "Save v10 continuation envelope, historical migration, browser persistence, and Astra evidence for F-011B1"
category: qa
created: 2026-09-09
updated: 2026-09-09
tags: [qa, determinism, save, wasm, bitmap, astra]
---

# F-011B1 Deterministic Save Continuation Proof

F-011B1 closes the known save-state omissions that caused a loaded campaign to
diverge immediately from an uninterrupted campaign. It does not yet prove the
larger F-011 command-replay and native-versus-WASM acceptance gate.

## Implementation

Save format v10 adds the exact simulation RNG position, optional second-AI
state, repair state, automatic-combat cooldowns, and active `GameConfig` to
`SaveState`. The interactive app snapshots and restores all five instead of
resetting second AI, repair, and combat memory on load.

Typed-key `HashMap` values could not previously enter the canonical JSON
fingerprint once populated because JSON object keys must be strings. The new
serde adapters encode unordered maps and sets as key-sorted sequences only for
human-readable serializers. Bincode retains its historical map/set wire shape.
This makes current fingerprints deterministic for populated manufacturing,
movement, AI, uprising, betrayal, economy, fog, blockade, and combat state.

Native v8/v9 bodies deserialize through the exact historical `SaveStateV9`
layout. A v9 file has its stored v9 fingerprint checked before migration; its
new continuation fields receive explicit safe defaults and the migrated v10
fingerprint is marked unverified. The regression fixture
`crates/rebellion-data/tests/fixtures/v9-minimal-save.reb` is the actual
718-byte output of commit `355715b` and has SHA-256
`a667db5092ddb6bea0b3729db6bcfefc73916609c373ed6f2a10f88a36b3127d`.
Browser storage writes v10 keys, can read v9 keys as a migration fallback, and
deletes both generations.

## Automated verification

| Gate | Result |
|------|--------|
| Workspace tests | 483 passed, 0 failed |
| Core tests | 356 passed, including typed-key JSON ordering/round-trip |
| Data tests | 60 passed, including 21 save tests |
| Historical v9 artifact load | Pass |
| Exact RNG continuation after v10 round-trip | Pass, 8 subsequent `u64` values identical |
| Second AI, combat cooldown, and config round-trip | Pass |
| Populated typed-map insertion-order normalization | Pass |
| Fingerprint sensitivity to RNG/config changes | Pass |
| Independent seed-42 initial-state probe | Pass |
| Native workspace build/test | Pass with recorded baseline warnings |
| WASM check and release package | Pass with recorded baseline warnings |

The optimized browser artifact is 4,607,142 bytes with SHA-256
`6bc4ef3e0d173cc347c4e9341cabd3686d556c72928989bb3c6f44c8b90a5992`.
The 28,285,762-byte runtime pack remains
`491fd403efb4bbb8210331c535626d9f10f8efb568a2f0a70f7cafc963c208ca`.

## Astra medium browser acceptance

The first strict pass proved the v10 journey but failed 2 of 36 assertions
because the page requested an absent `/favicon.ico`. The page now declares an
empty data-URL icon, removing that unrelated HTTP and console error.

The fresh r2 pass completed 40 of 40 assertions:

- Started an Alliance Standard campaign and saved slot 1 at Day 84 as the exact
  name `Continuation | Astra r2`.
- Wrote only `rebellion_save_v10_0` and `rebellion_meta_v10_0`; no v9 key was
  created. Metadata schema 1 retained the decimal-string fingerprint.
- Reloaded in the same isolated context, used main-menu Load Game, and matched
  `v1:6bb217229d3c4c60` with `verified=true`.
- Resumed from Day 84 to Day 88 with the authentic cockpit and galaxy-map
  bitmaps intact.
- Loaded `runtime.orpk` twice with zero loose DAT/BMP requests, HTTP failures,
  request failures, page exceptions, console errors, missing-asset warnings,
  invalid/deleted texture warnings, or favicon requests.

Only the already tracked optional audio/JavaScript hook and software-renderer
`ReadPixels` warnings occurred.

## Evidence files

- [Astra r2 machine report](state-continuation-r2/astra-r2-summary.json)
- [Sanitized console/network/storage events](state-continuation-r2/astra-r2-events.json)
- [Fingerprint records](state-continuation-r2/fingerprints.log)
- [Initial bitmap campaign](state-continuation-r2/campaign-initial.png)
- [Persisted v10 slot](state-continuation-r2/load-persisted.png)
- [Resumed Day 88 campaign](state-continuation-r2/resumed-campaign.png)
- [Astra r1 favicon failure](state-continuation-r2/astra-r1-failure.json)

F-011 remains open. Subsequent tranches must record and replay a versioned
command stream, remove simulation-time unordered iteration effects, identify
game-data inputs by hash, converge interactive/headless/automatic/tactical
paths, and compare native and WASM checkpoints across long seeded runs.
