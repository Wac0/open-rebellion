---
title: "Original Main-Menu Cockpit Browser Proof"
description: "Binary-mapped controls, responsive bitmaps, direct faction starts, and MDATA.302 WebAudio evidence"
category: qa
created: 2026-09-09
updated: 2026-09-09
tags: [qa, main-menu, bitmap, wasm, audio, astra]
---

# Original Main-Menu Cockpit Browser Proof

The custom text menu and separate setup page have been replaced in the browser
path by the original 640x480 shuttle cockpit described in
[`agent_docs/main-menu-parity.md`](../../../../agent_docs/main-menu-parity.md).
This is a verified tranche of P03/P04, not closure of either pass.

## Implemented tranche

- COMMON bitmap 20001 is composed with the binary-mapped resting, selected,
  and animated control resources; their palette-blue matte is transparent.
- Pointer activation uses the original 14 logical regions and a centered 4:3
  transform. Keyboard focus remains available without replacement text buttons.
- Difficulty, galaxy size, and game type visibly transition through the mapped
  resources. Alliance and Empire start directly without showing the custom
  setup page; Load/Options opens the slot picker and Quit exits.
- The deterministic runtime pack now contains the original owned MDATA.302
  bytes. WebAudio decodes it once, begins one loop after a user gesture, avoids
  overlap, and stops the source on Quit.

## Verification

| Gate | Result |
|------|--------|
| Workspace tests | 491 passed, 0 failed, 2 ignored |
| Focused menu tests | 5 passed, including exact outside-edge regression |
| Runtime-pack parser/builder | 3 Rust + 2 Python tests passed |
| WASM release package | Pass with recorded baseline warnings |
| Astra medium R2 | 43 focused passes, 1 focused failure; found the URL-plugin ABI mismatch; also found Quit audio cleanup |
| Astra medium R3 | 11 passed, 0 failed; focused tranche safe to commit |

Astra R2 exercised every control family, eight just-outside difficulty clicks,
four viewports, both factions, Load/Options, Quit, and WebAudio. R3 independently
confirmed both corrections and ran boundary and Alliance-start smoke tests.

The browser cold-start contract remains four successful requests: document,
`gl.js`, optimized WASM, and one `runtime.orpk`. The pack contains 52 game-data
files, 2,231 bitmaps, and one audio file. The MDATA.302 payload is 2,438,956
bytes with SHA-256
`fbd5e5772e5bbe7b3d82157cc60722972782aae22dd67cc7ff3259d370d078e3`.
The runtime pack SHA-256 is
`472c3c8ce7968e34e250bbd97ae1e1cc5830a3d2e08b21ee5dd1d5ce2d4c5a18`;
the optimized 4,612,625-byte WASM SHA-256 is
`63f984a77002edb29692660da0bb65b1d8fbf495452a18cc88c04584a28029d7`.

## Remaining P03/P04 gates

- Credits and Multiplayer still have no destinations.
- Headquarters Only is visible but does not reach active victory rules, save
  state, or `VictorySystem`.
- Difficulty and galaxy size reach `SeedOptions` in source, but live downstream
  values and persistence still need observable browser proof.
- Native parity, keyboard/accessibility acceptance, gain/mute behavior,
  return-to-menu audio lifecycle, and clean second-campaign reset remain open.

## Retained evidence

- [Astra R2 ledger](main-menu-r3/astra-r2-ledger.json) and
  [raw instrumentation](main-menu-r3/astra-r2-raw.json)
- [Astra R3 ledger](main-menu-r3/astra-r3-ledger.json) and
  [raw instrumentation](main-menu-r3/astra-r3-raw.json)
- [Bitmap-family comparison](main-menu-r3/bitmap-comparison.json) and
  [viewport measurements](main-menu-r3/viewport-measurements.json)
- [Native-size cockpit](main-menu-r3/resting-640x480.png),
  [wide responsive cockpit](main-menu-r3/resting-1280x800.png),
  [Alliance](main-menu-r3/alliance-galaxy.png),
  [Empire](main-menu-r3/empire-galaxy.png), and
  [Load/Options](main-menu-r3/load-picker.png)
- [R3 exact-boundary result](main-menu-r3/boundary-after.png) and
  [Quit result](main-menu-r3/quit-exit.png)
