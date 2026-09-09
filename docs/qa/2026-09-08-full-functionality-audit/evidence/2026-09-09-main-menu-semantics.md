---
title: "F-016D Main-Menu Browser Semantics"
description: "Astra-medium acceptance evidence for the 14 authentic cockpit controls in the browser accessibility tree"
category: qa-evidence
created: 2026-09-09
updated: 2026-09-09
tags: [qa, main-menu, accessibility, wasm, astra, bitmap]
---

# F-016D Main-Menu Browser Semantics

F-016D adds a dependency-free bridge between the browser accessibility tree and
the existing Rust bitmap controls. The visible cockpit is unchanged: the DOM
buttons are clipped to 1×1 pixels, do not accept pointer events, and invoke the
same focus, selection, routing, and recovered `COMMON.DLL` sound paths as the
canvas controls.

## Verification

- Runner: `codex-orchestrator`, GPT-6 Astra, medium effort
- Browser: Chromium 151.0.7922.34, 1280×960, two isolated contexts
- Verdict: **SAFE TO COMMIT**; 9 passed, 0 failed, 0 unverified
- Workspace: 502 passed, 0 failed, 2 intentionally ignored
- Package: 4 successful startup requests, 0 failed requests, 0 page/console
  errors, and 0 plugin/import warnings
- Accessibility: 1 named navigation landmark, 14 controls, correct default and
  changed `aria-pressed` state, 84/84 cyclic focus transitions, and 6/6 exact
  Enter/Space activations
- Lifecycle: Load/Options, Credits, Multiplayer, and an Alliance campaign all
  hid the landmark outside the menu and restored 14/14 controls on return
- Audio: semantic Tab started the 35.340770833-second `MDATA.300` loop; effects
  8000, 8001, 8002, and 8004 played on their assigned control families

Chromium reported four informational GPU-stall warnings from `ReadPixels`; no
rendering failure accompanied them. Physical speaker output and native
screen-reader speech were outside this browser pass.

## Artifact identity

| Artifact | Bytes | SHA-256 |
|---|---:|---|
| `web/open-rebellion.wasm` | 4,677,433 | `caa999e986be6c90d9c28bfbc6794c4656f7f88a8dd0d61b9f9715ff50f20d96` |
| `web/data/runtime.orpk` | 29,096,058 | `6123deec14cfbfd070573c7001ef1123bd52067273498ad0c36cb7fdb108f509` |
| Packaged `music/main_theme.wav` | 2,438,956 | `7d1212e1a91eb8d88ebaf0ce59000753561e4066425a24177baf1b04d07447f5` |

The machine-readable result is [result.json](main-menu-semantics-r1/result.json).
The [focus screenshot](main-menu-semantics-r1/easy-focus.png) shows the gold
outline on the authentic Easy bitmap with no visible HTML replacement text.
