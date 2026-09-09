---
title: "Save/Delete UI Acceptance Proof"
description: "F-001 wiring, empty-slot guard, browser storage deletion, and Astra evidence"
category: qa
created: 2026-09-09
updated: 2026-09-09
tags: [qa, save, delete, wasm, bitmap, astra]
---

# F-001 Save/Delete UI Acceptance Proof

F-001 originally reported that Save, Load, and Delete actions reached empty
match arms. The current app handles all three before its general panel-action
dispatcher, and the main-menu Load Game route opens the real slot picker.

## Defect found and fixed

The first independent Astra-medium Delete pass proved that the save and its
metadata were removed, but failed 1 of 28 assertions: an empty Load Game row
could still be selected, enabling Load and producing `no save in slot 0`.

The panel now disables empty rows in load mode and independently requires the
selected slot to exist before enabling Load. Empty slots remain selectable in
save mode. Three focused tests cover empty-load rejection, occupied-load
acceptance, and named saves into empty slots.

## Verification

| Gate | Result |
|------|--------|
| Workspace tests | 486 passed, 0 failed |
| Render tests | 67 passed, including 3 new save-panel tests |
| WASM release package | Pass with recorded baseline warnings |
| Astra r1 | 27/28; exposed the empty-load-slot defect |
| Astra r2 | 33/33 |

The Astra r2 journey used a fresh isolated browser context and the real UI:

- Started an Alliance Standard campaign with the authentic bitmap cockpit and
  galaxy map visible.
- Saved slot 1 under the exact name `Delete | Astra r2`.
- Verified only the v10 body and metadata keys, including the lossless name,
  tick, fingerprint version, and decimal-string fingerprint value.
- Deleted through the visible control and observed removal of both v10 keys;
  v9 fallback keys were absent.
- Proved deletion cleared selection by entering another name and attempting
  Save without reselecting a slot; no storage operation occurred.
- Reloaded in the same context, opened Load Game, and proved pointer and
  keyboard attempts could not select the empty slot or activate Load.
- Observed two successful `runtime.orpk` loads and zero loose DAT/BMP requests,
  HTTP failures, request failures, page exceptions, console errors,
  missing-asset warnings, invalid/deleted texture warnings, or favicon errors.

The optional audio/JavaScript-hook warnings and four software-renderer
`ReadPixels` warnings remain separately classified baseline diagnostics.

The rebuilt 4,607,698-byte WASM has SHA-256
`454f3f3b2d2741c8c1e490ff579b9128a1280ff2a9372ca52ae627fd87e030cd`.
The runtime pack remains 28,285,762 bytes with SHA-256
`491fd403efb4bbb8210331c535626d9f10f8efb568a2f0a70f7cafc963c208ca`.

## Evidence files

- [Astra r2 machine report](save-delete-r2/astra-r2-summary.json)
- [Sanitized r2 event ledger](save-delete-r2/astra-r2-events.json)
- [Rendered campaign](save-delete-r2/campaign.png)
- [Populated slot](save-delete-r2/populated-slot.png)
- [Slot immediately after deletion](save-delete-r2/post-delete-slot.png)
- [Empty Load Game panel after reload](save-delete-r2/post-reload-empty-load.png)
- [Empty-slot click probe](save-delete-r2/empty-load-click-probe.png)
- [Astra r1 failure report](save-delete-r2/astra-r1-failure.json)
- [Astra r1 defect screenshot](save-delete-r2/empty-load-enabled-r1.png)

The original command-discarding defect and the browser Delete path are closed.
Native backend round-trip/delete tests pass; a native GUI restart smoke test
remains in the cross-platform P31 release matrix. IndexedDB, quota handling,
and asynchronous persistence remain separate F-015 work.
