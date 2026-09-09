---
title: "Fleet Miniature Browser Proof"
description: "Resource mapping, transparency, interaction, and browser evidence for F-010B"
category: qa
created: 2026-09-08
updated: 2026-09-08
tags: [qa, bitmap, fleet, gokres, astra, wasm]
---

# Fleet Miniature Browser Proof

## Verdict

F-010B passes. Commit `4589d2e` replaces the invalid compound-DAT-ID offset
with explicit record-order mappings for all 30 capital-ship classes and all
eight fighter classes. Known GOKRES fleet miniatures now apply the original
palette-blue transparency matte without removing blue pixels from unrelated
resources.

This closes only the fleet-list miniature tranche. F-010 remains open until
every image-bearing surface passes the same resource, runtime, screenshot,
interaction, and fallback protocol on every supported platform.

## Mapping ledger

| Consumer | DAT records | GOKRES resources | Coverage |
|---|---:|---:|---|
| Alliance fighters | `FIGHTSD` 1–4 | 17984–17987 | All four mappings unit-tested. |
| Empire fighters | `FIGHTSD` 5–8 | 18048–18051 | All four mappings unit-tested. |
| Alliance capital ships | `CAPSHPSD` 64–78 | 18240–18254 | All 15 mappings unit-tested. |
| Empire capital ships | `CAPSHPSD` 128–142 | 18304–18318 | All 15 mappings unit-tested. |

Unknown records return no resource ID instead of requesting an unrelated
bitmap. The previous `dat_id.raw() + 17000` calculation was invalid because a
compound `DatId` is not a GOKRES resource number.

## Verification

- Implementation commit: `4589d2ecb6399eabda79fb57e048672f2e214365`
- Packaged and served WASM SHA-256:
  `d549f4fca8fb4bd9ef81a487ae4b656b1c9fa37ad8fd0e79013736ced5d6c323`
- `cargo test -p rebellion-render --lib`: 64 passed, 0 failed.
- `cargo test --workspace --all-targets`: core 355, data 50, render 64;
  0 failed, with one telemetry test ignored.
- Touched-file `rustfmt --check`: passed.
- Package manifest and SHA-256 verification: passed.

The first GPT-6 Astra medium run (R8) correctly matched the resource identity,
but failed the visual gate because the source bitmaps' solid-blue matte was
still visible. R9 repeated the run after the source-scoped transparency fix
and passed:

- Alliance: Corellian Corvette 18245, Medium Transport 18246, X-wing 17986,
  and Y-wing 17987.
- Empire: Imperial Star Destroyer 18309, TIE Fighter 18048, and Star Galleon
  18316.
- Every request returned HTTP 200 and matched its packaged file and manifest
  entry.
- All seven edge-inclusive screenshot scans found zero matte-blue pixels while
  retaining legitimate blue-gray ship detail.
- The 66×25 sources rendered at approximately 53×20 without aspect distortion.
- Fleet expand, collapse, and **Go to System** interactions passed for both
  factions.
- No failed bitmap request, loading failure, or runtime exception occurred.

The run also observed the existing favicon 404, audio/glue fallback messages,
and software-renderer `ReadPixels` warnings. These did not affect this tranche
and remain outside its acceptance boundary.

## Evidence files

- [R8 Alliance failure](fleet-r9/open-rebellion-fleet-r8-alliance-yavin.png)
- [R9 Alliance screenshots](fleet-r9/open-rebellion-fleet-r9-alliance-yavin.png)
- [R9 Empire screenshots](fleet-r9/open-rebellion-fleet-r9-empire-coruscant.png)
- [Package integrity](fleet-r9/open-rebellion-fleet-r9-integrity.json)
- [Pixel scans](fleet-r9/open-rebellion-fleet-r9-pixels.json)
- [Browser summary](fleet-r9/open-rebellion-fleet-r9-summary.json)
- [Render test log](fleet-r9/open-rebellion-fleet-r9-render-tests.log)
