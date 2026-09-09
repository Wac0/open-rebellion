---
title: "Save-State Fingerprint Proof"
description: "Canonical fingerprint, save-integrity, browser persistence, bitmap, and Astra evidence for F-011A"
category: qa
created: 2026-09-08
updated: 2026-09-08
tags: [qa, determinism, save, wasm, bitmap, astra]
---

# F-011A Save-State Fingerprint Proof

F-011A establishes the measurement primitive needed for deterministic replay.
It does not claim that the complete simulation is deterministic yet.

## Implementation

Save format v9 records a versioned `StateFingerprint` in the native header and
in browser metadata. Fingerprint v1 applies domain-separated FNV-1a to a
canonical JSON projection of the logical `SaveState`: object keys and known
set fields are sorted, while meaningful vector and queue order is retained.
The fingerprint is an informational determinism and corruption signal, not a
cryptographic authenticator.

The first seeded-campaign probe confirmed the audit's predicted failure mode:
hashing raw bincode produced `a466b3816a1f4940` and `f8b41644f508c148` for two
fresh seed-42 worlds because randomized map iteration changed the byte order.
After canonicalization, the same independent two-run test passes.

Native and browser loads recompute the logical fingerprint and reject a v9
save when it differs from the stored value. Native v8 saves remain compatible;
their fingerprint is computed during load and explicitly marked unverified
because v8 did not persist one.

Browser metadata is versioned JSON rather than the former `name|tick` string.
It preserves arbitrary player names and stores the unsigned 64-bit fingerprint
as a validated decimal string, avoiding JavaScript's 53-bit safe-integer limit.

## Astra medium browser acceptance

`codex-orchestrator` ran GPT-6 Astra at medium effort against three release
packages. r1 found that Rust `println!` did not emit the required WASM console
records. The app switched those records to Macroquad's browser-aware logger.
r2 passed the end-to-end flow. r3 repeated it after the lossless decimal-string
metadata hardening and passed all 34 assertions.

The r3 run used a fresh nonpersistent Chromium context and performed this real
canvas journey:

1. Boot the packaged game and start an Alliance Standard campaign.
2. Open Save Game with `S`, choose slot 1, and save the exact name
   `Fingerprint | Astra`.
3. Verify the v9 body and metadata keys, parsed JSON schema, tick, decimal
   string, and matching browser console record.
4. Reload in the same context, use main-menu Load Game, and load slot 1.
5. Verify the identical fingerprint with `verified=true`, resume the campaign,
   dismiss an event, and pause at Day 17.

| Gate | r3 result |
|------|-----------|
| Metadata name | Exact `Fingerprint | Astra`, including `|` |
| Metadata schema | v1 |
| Fingerprint | `v1:e2ad73730e68434a` |
| Decimal value | `16333838361542804298` stored as a string |
| Save/load fingerprint match | Pass |
| Load integrity flag | `verified=true` |
| Save survived same-context reload | Pass |
| Loaded campaign resumed | Day 0 to Day 17 |
| Authentic bitmap cockpit and map | Pass before and after load |
| Runtime pack loads | 2: initial boot and reload |
| Loose DAT/BMP requests | 0 |
| HTTP/loading failures | 0 |
| Page exceptions / console errors | 0 / 0 |
| Missing assets / invalid or deleted textures | 0 |

The only console warnings were the already tracked optional audio/JavaScript
hooks and software-renderer `ReadPixels` messages.

## Artifact and automated verification

| Measurement | Verified result |
|-------------|-----------------|
| Optimized r3 WASM | 4,504,917 bytes |
| r3 WASM SHA-256 | `3ad0e01c67d20e8c700c012bf8398e9e38e932e780ae1075114cc250aee3167a` |
| Runtime-pack SHA-256 | `491fd403efb4bbb8210331c535626d9f10f8efb568a2f0a70f7cafc963c208ca` |
| Save unit tests | 16 passed, 0 failed |
| Seeded two-run integration test | 1 passed, 0 failed |
| WASM check and release package | Pass with recorded baseline warnings |

The tests also prove fingerprint sensitivity to state changes, normalization of
set insertion order, v9 save/load preservation, rejection of a valid but
fingerprint-mismatched body, and v8 compatibility.

## Evidence files

- [Astra r3 machine report](state-fingerprint-r3/open-rebellion-f011a-astra-r3-summary.json)
- [Astra r3 console/network events](state-fingerprint-r3/open-rebellion-f011a-astra-r3-events.json)
- [Populated Save Game modal](state-fingerprint-r3/save-populated.png)
- [Persisted Load Game slot](state-fingerprint-r3/load-persisted.png)
- [Resumed Day 17 campaign](state-fingerprint-r3/resumed-campaign.png)
- [Astra r2 passing report](state-fingerprint-r3/open-rebellion-f011a-astra-r2-summary.json)
- [Astra r1 failing report](state-fingerprint-r3/open-rebellion-f011a-astra-r1-failure.json)

F-011 remains open. The next determinism tranches must include missing
continuation state (RNG, second-AI/runtime state, repair/combat cooldowns,
configuration, and data hashes), record a replayable command stream, converge
interactive and headless simulation, and compare checkpoints across native,
WASM, automatic combat, and tactical combat.
