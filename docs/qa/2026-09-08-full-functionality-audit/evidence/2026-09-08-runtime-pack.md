---
title: "Runtime Pack Browser Proof"
description: "Deterministic asset-pack, request-count, bitmap, interaction, and WebGL evidence for F-014A"
category: qa
created: 2026-09-08
updated: 2026-09-08
tags: [qa, wasm, runtime-pack, performance, bitmap, astra]
---

# F-014A Runtime Pack Browser Proof

F-014A replaces the browser's thousands of loose startup fetches with one
deterministic, self-describing `runtime.orpk` file. The package retains the
legacy loose-file loader for development, while release artifacts contain only
five runtime files.

## Artifact integrity

| Measurement | Verified result |
|-------------|-----------------|
| Pack contents | 52 game-data entries and 2,231 bitmap entries |
| Pack size | 28,285,762 bytes |
| Pack SHA-256 | `491fd403efb4bbb8210331c535626d9f10f8efb568a2f0a70f7cafc963c208ca` |
| Optimized WASM size | 4,369,091 bytes |
| WASM SHA-256 | `3b144223ccf5f6cb7f3d29e3f8625162a67a0f1cecccf22f754472e7525b73f5` |
| Release package files | `index.html`, `gl.js`, `open-rebellion.wasm`, `data/runtime.orpk`, `SHA256SUMS` |
| ZIP size | 8.1 MiB |
| Reproducibility | Independent r6 and r7 builds produced identical WASM, pack, and `SHA256SUMS` bytes |

The pack writer sorts entries, canonicalizes `textstra.json`, validates every
written entry byte-for-byte, and rejects duplicate or malformed records. The
Rust parser bounds entry counts and key lengths and rejects unsupported flags,
unknown kinds, invalid UTF-8, truncation, duplicates, and trailing data.

## Astra medium browser acceptance

`codex-orchestrator` ran GPT-6 Astra at medium effort against the r6 release
package in two fresh nonpersistent Chromium 141 contexts. Request, response,
console, page-error, and request-failure listeners were installed before each
navigation.

| Gate | Alliance | Empire |
|------|----------|--------|
| Total cold-load requests | 4 | 4 |
| `data/runtime.orpk` requests | 1 | 1 |
| Loose DAT/BMP/manifest requests | 0 | 0 |
| HTTP or loading failures | 0 | 0 |
| Page exceptions / console errors | 0 / 0 | 0 / 0 |
| Pack count log | 51 game files, 2,231 bitmaps | 51 game files, 2,231 bitmaps |
| Standard/Medium campaign | Pass | Pass |
| Correct cockpit and transparent fleet art | Pass | Pass |
| Fleet expand/collapse/re-expand | Pass | Pass |
| Go to System | Pass | Pass |
| Five zoom-in plus five zoom-out steps | Pass | Pass |
| Invalid or deleted WebGL textures | 0 | 0 |

The Alliance proof shows a transparent Corellian Corvette and navigation to
Yavin. The Empire proof shows transparent Imperial Star Destroyer and TIE
Fighter miniatures and navigation to Coruscant. Neither faction displayed a
blue matte, placeholder, missing bitmap, or wrong-faction asset.

The browser still reports known non-blocking glue/audio-hook warnings; the
Alliance software renderer also emitted `ReadPixels` performance warnings.
Neither class is counted as a runtime error. Browser audio remains a separate
open feature pass.

## WebGL regression found and closed

r5 revealed `glBindTexture called with an already deleted texture ID 17!` when
wheel input selected a previously unseen map-label size. This matched
Macroquad's [font-atlas growth defect](https://github.com/not-fl3/macroquad/issues/1054),
whose [upstream fix](https://github.com/not-fl3/macroquad/pull/1063) is merged
but absent from the published crate used by the project. The app now caches the
Galaxy layer's actual character set for the current and both possible next
wheel-zoom sizes before drawing. r6 checked the console after every zoom step
and reproduced zero deleted-texture errors.

## Automated verification

- Workspace: 3 app, 355 core, 50 data, and 64 render tests passed; no failures.
- Documentation tests: 3 passed; expected ignored tests remain recorded.
- Runtime-pack Python unit test and bytecode compilation passed.
- WASM target check, release build, `wasm-opt`, shell syntax, pack self-check,
  package manifest inspection, and `git diff --check` passed.
- Repository-wide format and strict-clippy gates remain red from the previously
  recorded baseline drift; the new standalone Rust parser passes `rustfmt`.

## Evidence files

- [Astra r6 machine report](runtime-pack-r6/open-rebellion-runtime-pack-r6-summary.json)
- [Alliance final screenshot](runtime-pack-r6/open-rebellion-runtime-pack-r6-alliance.png)
- [Empire final screenshot](runtime-pack-r6/open-rebellion-runtime-pack-r6-empire.png)
- [Astra r5 failing report](runtime-pack-r6/open-rebellion-runtime-pack-r5-failure.json)

F-014A is complete. F-014 remains open for Brotli compression, bounded raw and
decoded-asset caches, HD entries, high-DPI rendering, single-pass egui,
geometry caching, memory/frame budgets, and Firefox/Safari acceptance.
