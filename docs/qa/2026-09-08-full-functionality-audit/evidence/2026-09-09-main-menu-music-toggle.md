---
title: "F-016E Menu Music and Final Main-Menu Acceptance"
description: "Original-button absence proof, documented holographic extension, native visual acceptance, and Astra-medium browser evidence"
category: qa-evidence
created: 2026-09-09
updated: 2026-09-09
tags: [qa, main-menu, audio, wasm, astra, fable, bitmap]
---

# F-016E Menu Music and Final Main-Menu Acceptance

F-016E closes P03 without rewriting original history. `REBEXE.EXE` constructs
exactly 14 cockpit controls and has no standalone music button; the original
uses `MusicSwitch` and `MusicVolume` through Save/Load and Options. Open
Rebellion adds a fifteenth, explicitly documented convenience control at
logical `(594,10,30,22)`.

## Design and behavior

- The 30×22 painted housing and hit target are the same device-pixel-aligned
  rectangle and do not intersect an original hotspot.
- Music starts enabled for players. The control mutes only music; original
  cockpit effects remain audible. Test sessions verify the initial state once,
  then remain muted.
- The physical bevel inherits the Rebellion cockpit. The restrained cyan
  projection, clipped scanlines, and inset status lamp follow the Jiff
  Gorda/SWG Project Thorn reference.
- Verified Fable 5.1 review required six-pixel canopy clearance, cyan confined
  to the projection, a rectangular red/green lamp, static rest art, restrained
  hover interference, pressed displacement, and gold reserved for focus.
- The lamp was moved wholly inside the dark aperture and enlarged after native
  inspection. The user accepted the final native enabled and muted rendering
  on 2026-09-09.

## Verification

- Runner: `codex-orchestrator`, GPT-6 Astra, medium effort
- Verdict: **SAFE TO COMMIT**; 10 passed, 0 failed, 0 unverified
- Workspace: 504 passed, 0 failed, 2 intentionally ignored
- Browser: Chromium 141.0.7390.37 at 640×480, 1280×960, 1280×800, and
  1440×900
- Startup: 4 successful requests; 0 failures; 0 console, page, WebGL,
  missing-asset, deleted-texture, panic, plugin, or import errors
- Semantics: 15 buttons in stable order—14 original plus `Menu music` at index
  14—with exact single pointer and keyboard activation
- Geometry: 24 one-device-pixel outside-edge clicks were inert; the housing
  stayed clear of the canopy strut at every viewport
- Narrow regression: a follow-up at 320×240 and 480×360 found centered bevel
  strokes painting one pixel outside the smaller hitbox. The complete device
  is now clipped to that shared rectangle; Astra R2 passed 10/10, including
  8/8 outside-edge probes and zero bright pixels in the surrounding ring.
- Audio: muted music gain `0`; `COMMON.DLL` MenuSelect resource 8004 still
  played at SFX gain `1`
- Regression: Intermediate selection and Credits navigation still worked; the
  final browser state was muted

Chromium emitted four informational `ReadPixels` GPU-stall warnings while the
test captured pixels. They did not accompany a rendering or runtime failure.

## Artifact identity

| Artifact | Bytes | SHA-256 |
|---|---:|---|
| `web/open-rebellion.wasm` | 4,682,288 | `3619ff51bf4b3a5137b8672204b454b632b040a1d2fa2bd642e02ff07957a9fa` |
| `web/data/runtime.orpk` | 29,096,058 | `6123deec14cfbfd070573c7001ef1123bd52067273498ad0c36cb7fdb108f509` |

The machine-readable results are the
[broad matrix](main-menu-music-toggle-r1/result.json) and final
[narrow R2](main-menu-music-toggle-r1/narrow-r2-result.json). Compare the final
[enabled](main-menu-music-toggle-r1/enabled-320x240.png) and
[muted](main-menu-music-toggle-r1/muted-320x240.png) narrow states; the final
[focus capture](main-menu-music-toggle-r1/focused-480x360.png) shows the
external gold outline without obscuring or enlarging the control.
