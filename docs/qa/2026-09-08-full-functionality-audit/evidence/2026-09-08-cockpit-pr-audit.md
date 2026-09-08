---
title: "Cockpit PR #2 Audit"
description: "Disposition and browser evidence for cockpit texture mapping and missing-asset diagnostics"
category: qa
created: 2026-09-08
updated: 2026-09-08
tags: [qa, cockpit, bitmap, astra, pull-request]
---

# Cockpit PR #2 Audit

## Verdict

PR #2 correctly identified silent bitmap failures, but its proposed DLL remap
does not identify the intended control art. The bounded missing/read/decode
diagnostics were integrated. The remap was not: a browser run proved that
STRATEGY resources 11016–11027 load, but render character portraits. The
controls now fail closed to readable labels until the original
command-to-animation table is established under F-010.

The PR's later EData modulo mapping and machine-specific MDATA decoder were
also excluded because neither is release-safe or browser-complete.

## Resource evidence

- COMMON resources 11016, 11019, 11022, and 11025 do not exist in the staged
  extraction.
- STRATEGY resources with those IDs exist as 25×25 bitmaps, but visual
  inspection and Astra R6 showed character portraits rather than cockpit
  control symbols.
- COMMON resources 11001–11275 form grouped animations of faction emblems,
  spacecraft, planets, starfields, insignia, and disc controls. They are not a
  sequence of nine logical three-state panel buttons.
- `ghidra/notes/FUN_00405560.c` corroborates non-contiguous animation bases,
  including 11001, 11031, 11061, 11091, 11121, 11151, 11181, and 11241.

## Integrated behavior

- `BmpCache` logs each unavailable `(DLL, resource ID)` only on its first
  lookup because the failed result is negatively cached.
- Native file-read and image-decode failures include DLL, resource ID, path,
  and error details.
- Successful loads are not logged, avoiding the thousands of messages emitted
  by the PR branch's request/success tracing.
- The authentic faction cockpit frame remains visible while the nine working
  controls display explicit labels rather than unrelated art.

## Verification

- `cargo test -p rebellion-render`: 59 passed, 0 failed, including one-time
  negative-cache coverage for a missing resource.
- `cargo test --workspace`: 467 passed, 0 failed, 17 ignored documentation or
  telemetry cases; existing compiler warnings remain tracked by F-009.
- `cargo check -p rebellion-render --target wasm32-unknown-unknown`: passed
  with pre-existing warnings.
- Browser package: 2,288 SHA-256 manifest entries verified.
- Served/package WASM SHA-256:
  `5d5dfa96750b33e580e8d4bfc9559b6f0aa0c3ded6a240c5e56eab4b7e55802f`.
- Astra R7, GPT-6 Astra at medium effort: Strategy 900 cockpit frame and
  transparent aperture passed; all nine labels were readable; Officers,
  Encyclopedia, Save/Load, Faster, and Slower worked; control clicks emitted
  no asset requests; no `bmp_cache` or page exception occurred.
- The asset manifest still prefetches every staged bitmap at startup. Removing
  that 2,278-request behavior remains browser-performance work under F-014.

The Astra transcript described a build-identity mismatch because its prompt
contained an incorrect tentative hash. Its measured browser hash is the exact
hash of both `web/open-rebellion.wasm` and the packaged WASM above.

## Screenshots

- [Alliance cockpit](cockpit-r7-alliance.png)
- [Officers control](cockpit-r7-officers.png)
- [Encyclopedia control](cockpit-r7-encyclopedia.png)
- [Save/Load control](cockpit-r7-save-load.png)
- [Faster control](cockpit-r7-faster.png)
- [Slower control](cockpit-r7-slower.png)

## Follow-up

F-010 remains open. Resolve the original control handler and animation-state
semantics, map faction-specific hotspots, then repeat the full bitmap proof
protocol on native and browser builds before claiming cockpit-art parity.
