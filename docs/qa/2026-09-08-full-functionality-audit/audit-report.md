---
title: "Open Rebellion Full Functionality Audit"
description: "Repository status, verified evidence, release blockers, and feature-by-feature acceptance plan"
category: qa
created: 2026-09-08
updated: 2026-09-08
commit: fc25634be905839dfa6fb477d5fff0faa49d8ae9
tags: [qa, audit, functionality, parity, bitmap, wasm, astra, fable]
---

# Open Rebellion Full Functionality Audit

## Executive conclusion

Open Rebellion is feature-rich and has substantial unit-test coverage, but it
is not yet demonstrably 100% functional. The repository is at post-implementation
integration and acceptance closeout. Several user-facing paths are incomplete,
the long-running campaign simulation exhibits runaway behavior, and the existing
CI pipeline does not establish browser or visual correctness.

This audit was performed against:

- Commit: `fc25634be905839dfa6fb477d5fff0faa49d8ae9`
- Branch: `main`, synchronized with `origin/main`
- Audit date: 2026-09-08
- Platform: macOS, Apple Silicon
- Independent reviews:
  - GPT-6-Astra through `codex-orchestrator`, medium effort, read-only
    researcher profile
  - Claude Fable 5.1 through the verified Fable CLI lane, repository-read-only
    audit

No tracked source files were modified during the audit. Four pre-existing
evaluation files were untracked: `EVALUATE.md`, `edd.config.json`, `program.md`,
and `run-eval.sh`.

## Where work left off

The latest commit added the ordinary WASM BMP cache pipeline and closed the
remaining AI parity tracker entries. The preceding work added HD upscale assets
and the Tammuz story, combat, advisor, and telemetry changes.

The repository's records describe different scopes and are not a unified
acceptance record:

| Record | Actual status |
|--------|---------------|
| `README.md` | Advertises Core 100%, Combat 100%, UI 99%; these are claims rather than reproduced acceptance results. |
| `progress.archived-2026-04-07-native-video.json` | Archived record of the April 7 native-video task; it reports lint as false and is superseded by this audit. |
| April 12 Tammuz plan | Described as completed elsewhere, but its functional and quality acceptance checkboxes remain open. |
| April 6 test-infrastructure plan | Baseline investigation is complete; implementation milestones remain open. |
| March 24 QA inventory | Historical v0.15 browser observations, not acceptance evidence for the current build. |

## Reproduced baseline

| Gate | Result | Evidence |
|------|--------|----------|
| Workspace tests | PASS | 465 passed, 0 failed, 17 ignored. |
| App/playtest automated tests | GAP | Neither binary has direct test coverage. |
| Native BMP decode | PASS | All 2,231 staged BMP files decoded with ImageMagick. |
| HD PNG decode | PASS | All 234 PNG files decoded with ImageMagick. |
| Production HD replacements | PARTIAL | 74 production DLL PNGs: Common 53, Gokres 13, Strategy 7, Tactical 1. |
| Raw WASM release compile | PASS WITH WARNINGS | `wasm32-unknown-unknown` build completed with 30 warnings. |
| Formatting | FAIL | Formatting drift was detected across approximately 83 files. |
| Strict clippy | FAIL | At least 98 errors in `rebellion-core`, with additional workspace findings. |
| Packaged WASM browser boot | NOT RUN | The raw compilation result does not exercise `scripts/build-wasm.sh` or a browser. |
| Visual bitmap correctness | NOT PROVEN | Decode success establishes file integrity, not in-game display or identity. |

The local `cc` command is a tmux/Claude launcher rather than the system C
compiler. Verification commands must use a sanitized `PATH` so Rust links with
Apple clang:

```bash
env PATH=/Users/tomdimino/.cargo/bin:/opt/homebrew/bin:/usr/bin:/bin:/usr/sbin:/sbin \
  cargo test --workspace
```

## Campaign evaluation

A fresh 5,000-tick dual-AI campaign was run with seed 42.

| Metric | Observed |
|--------|----------|
| Quality score | 0.2851 |
| Parity result | FAIL: 19 pass, 5 fail, 1 skip |
| Victory | None by tick 5,000 |
| Total events | 2,001,963 |
| Initial fleets | 5 |
| Fleets in transit at end | 980 |
| AI attack orders | 306,012 |
| Empire AI actions | 326,092 |
| Alliance AI actions | 6,332 |
| Space/ground/bombardment events | 655 / 2 / 1 |
| Battles at Yavin | 655 of 658 combat events |

The run also reported missing `TroopClassDef` entries. Parity failures covered
the original AI interval, Death Star construction/fire/shield events, and
research completion events; victory evidence was skipped because no victory
occurred.

This is release-blocking behavior even though the evaluator did not classify
the run as formally degenerate.

## Confirmed findings

### F-001: Save, Load, and Delete commands are discarded

- Severity: P0
- Status: confirmed
- Evidence: the backend and panel exist, but `PanelAction::SaveGame`,
  `LoadGame`, and `DeleteSave` reach empty match arms in
  `crates/rebellion-app/src/main.rs`.
- Acceptance: perform UI-driven save, restart, load, compare full campaign
  state, delete the slot, and verify disk/browser storage changes.

### F-002: Native HD bitmap root does not match the asset layout

- Severity: P1
- Status: confirmed
- Evidence: the app configures `data/hd/ui`, while production assets are under
  `data/hd/{common,gokres,strategy,tactical}-dll/`.
- Acceptance: log and render a known HD override, then remove it and prove BMP
  fallback on the same resource.

### F-003: WASM never prefetches HD override bytes

- Severity: P1
- Status: confirmed
- Evidence: startup inserts `{dll}/{id}` BMP keys, while the renderer searches
  for `hd/{dll}/{id}` first. The build script does not stage or manifest HD
  entries.
- Acceptance: observe a browser HTTP request for a known HD PNG, a successful
  `hd/...` cache hit, correct dimensions, and the correct on-screen image.

### F-004: WASM omits troop data prefetch

- Severity: P1
- Status: confirmed
- Evidence: `TROOPSD.DAT` is not in the WASM prefetch list although the loader
  consumes it for troop combat statistics.
- Acceptance: native and browser worlds report matching troop-class counts and
  no fallback diagnostics.

### F-005: Victory modal is unreachable in normal play

- Severity: P1
- Status: confirmed
- Evidence: normal victory handling transitions directly to a cutscene or main
  menu; the compiler reports `VictoryModal` is never constructed.
- Acceptance: every win and loss route reaches the result UI, freezes play,
  follows the accepted cutscene policy, and returns cleanly to the menu.

### F-006: Interactive combat does not establish core combat parity

- Severity: P1
- Status: confirmed design divergence
- Evidence: tactical space and interactive ground combat use simplified
  calculations separate from core auto-resolution.
- Acceptance: deterministic fixtures prove that all entry paths use the
  accepted mechanics and apply identical permanent losses and conquest state.

### F-007: Long-running AI simulation exhibits runaway fleet behavior

- Severity: P0
- Status: reproduced
- Evidence: seed 42 grew from 5 initial fleets to 980 fleets in transit,
  generated 306,012 attack orders, and produced no victory by tick 5,000.
  In-transit fleets remain eligible for AI dispatch, a replacement order resets
  their travel, production creates replacement one-ship fleets at vacated
  shipyards, arrivals do not merge, and only one fleet pair per system is
  resolved on a five-tick cadence.
- Acceptance: multi-seed bounds for fleet counts, orders, event volume, target
  diversity, faction balance, battle spread, and victory timing all pass.

### F-008: Browser media and mods are incomplete

- Severity: P1 if browser parity is claimed
- Status: confirmed platform gap
- Evidence: browser advisor frames, EData images, cutscene playback, mods, and
  parts of audio integration use stubs, empty collections, or immediate
  completion behavior.
- Acceptance: implement and demonstrate each feature, or explicitly exclude it
  from the supported browser contract and qualify all completion claims.

### F-009: Release quality gates are not enforced

- Severity: P1
- Status: confirmed
- Evidence: current CI runs native check/tests and raw WASM compilation only.
  Formatting, strict clippy, packaging, browser execution, screenshots, parity,
  coverage, and asset integrity are absent.
- Acceptance: the release pipeline runs the same versioned commands and fixtures
  used by local acceptance, retains artifacts, and blocks regressions.

### F-010: Bitmap files decode, but display correctness is unproven

- Severity: P1
- Status: open acceptance gap
- Evidence: all staged files decode, but no current resource ledger connects
  every consumer to a runtime cache hit and screenshot.
- Acceptance: complete the bitmap proof protocol below for every image-bearing
  surface on every claimed platform.

### F-011: Deterministic replay is not established

- Severity: P0 for multiplayer; P1 for reproducible single-player acceptance
- Status: confirmed architecture gap
- Evidence: unordered map iteration can affect movement and AI tie-breaking;
  headless and interactive paths consume randomness differently; interactive
  seeds are wall-clock-derived; and save state omits RNG, AI runtime, combat
  cooldown, configuration, and data-hash state.
- Acceptance: repeated native runs and native-versus-WASM runs produce the same
  versioned state fingerprints for the same seed and command stream, including
  after save/load.

### F-012: Interactive and headless simulation loops diverge

- Severity: P0
- Status: confirmed design divergence
- Evidence: the app duplicates simulation sequencing instead of calling the
  headless tick entry point. Ground follow-up, bombardment, battle-memory
  updates, AI availability, Death Star effects, telemetry, and `AdvanceTicks`
  behavior differ between paths.
- Acceptance: one authoritative simulation entry point produces identical
  results from the same seed and commands in app, playtest, native, and WASM.

### F-013: The packaged browser artifact omits runtime data

- Severity: P0 for browser release
- Status: confirmed
- Evidence: `scripts/package-web.sh` packages the page, JavaScript glue, and
  WASM but not the runtime `web/data` payload required by startup.
- Acceptance: a clean unpacked artifact boots offline from its own contents,
  loads the expected data hashes, and passes browser smoke tests.

### F-014: Browser startup, memory, and rendering are not release-scaled

- Severity: P1
- Status: confirmed optimization gap
- Evidence: approximately 2,231 BMP and 40 DAT resources are fetched serially;
  raw bytes and decoded textures have no eviction policy; high-DPI mode is not
  enabled; the galaxy can run two egui passes per frame; and sector hulls are
  recomputed each frame.
- Acceptance: the browser asset pack, lazy decode, LRU limits, one UI pass, and
  cached geometry meet the budgets below in Chrome, Firefox, and Safari.

### F-015: Browser persistence is not production-safe

- Severity: P1
- Status: confirmed design gap
- Evidence: synchronous base64-encoded bincode in `localStorage` is vulnerable
  to quota limits and main-thread stalls, and the save schema does not contain
  all state required for deterministic continuation.
- Acceptance: versioned, compressed, asynchronous IndexedDB saves round-trip
  complete state, expose quota/corruption errors, and preserve fingerprints.

## Fable 5.1 audit synthesis

The Fable review confirmed the original blockers and sharpened several
interpretations:

- The `19 pass / 5 fail / 1 skip` parity score is not five equivalent gameplay
  failures. The original AI interval is a documented augmentation, three Death
  Star checks expose structural AI/event gaps, and the research mismatch may be
  an oracle-ID mismatch. The run remains practically degenerate despite the
  evaluator's `false` flag because it only detects the absence of combat.
- Save/load is more incomplete than an unwired in-game panel: the main-menu
  Load Game route starts a default Alliance world without a slot picker or
  faction restoration.
- Browser parity currently excludes the entire audio engine, not only selected
  sounds. Native advisor frames also depend on a gitignored reference-art path,
  so release packaging must stage owned runtime assets explicitly.
- Auto-resolve, tactical space, and ground combat can apply three materially
  different outcomes. Interactive result application also omits officer capture.
- Autoresearch parameter tuning should pause until fleet reordering, unbounded
  spawning, combat backlog, and deterministic replay are corrected; tuning
  around those feedback defects would optimize an invalid simulation.

## Optimization and parity roadmap

The order below makes the acceptance ledger executable. Milestones are gated;
later work must not hide failures in an earlier invariant.

| Milestone | Scope | Exit criteria |
|-----------|-------|---------------|
| M0 — Truth and bleeding | Wire save/load/delete and Load Game selection; fix native HD root and `TROOPSD.DAT`; ship browser data; attach evidence to README claims; add a two-run fingerprint check. | Persistence works on native/WASM, the packaged site boots from a clean directory, and claims link to current evidence. |
| M1 — Simulation correctness | Prevent in-transit redispatch; model fleet position; merge arrivals; attach production correctly; impose stable ordering/RNG; aggregate system combat; enable Death Star production/fire; repair oracle checks. | Across five 5,000-tick seeds: transit ≤10% of fleets, move orders ≤1.5× arrivals, fleet arena ≤3× initial, 50–400 battles over ≥8 systems, top system ≤40%, and at least one Death Star victory where the fixture permits. |
| M2 — One game engine | Route app and playtest through one tick API and event sink; make combat resumable from core state; construct victory UI; remove or correctly simulate `AdvanceTicks`. | Same seed plus command stream yields identical checkpoints and final state across interactive, headless, native, WASM, auto, and tactical paths. |
| M3 — Browser excellence | Create a Brotli-compressed indexed `ui.pak`; lazy-decode and LRU-cache textures; ship HD entries; enable high DPI; use one egui pass; cache geometry; move saves to IndexedDB; unlock audio after gesture; stage advisor frames; run real-browser input suites. | Cold start ≤3 s at 50 Mbps/30 ms, ≤4 requests before menu, combined heap/WASM ≤256 MB after 10 minutes, no visual/input failures in current Chrome/Firefox/Safari. |
| M4 — Multiplayer | Introduce validated, tick-stamped commands; authoritative host simulation; faction-filtered fog-safe deltas and snapshots; secure WSS transport; prediction/reconciliation; reconnect; persistence and observability. | Two clients run 5,000 ticks with matching server checkpoints every 250 ticks; at 200 ms RTT there are no input stalls and ≤1 reconciliation per 100 commands; reconnect within 60 s; all illegal commands rejected; hidden state absent from client memory. |
| M5 — Continuous proof | Enforce format/clippy/build/browser checks; short and long campaign gates; resource and screenshot ledgers; app integration tests; package boot and data/save hashes. | Every supported P00–P39 pass is green from release artifacts, with reproducible evidence retained by CI. |
| v1.0 — Protected Cloudflare release | Deploy the self-contained browser build to Cloudflare Pages with Functions middleware, `SITE_PASSWORD` and `SESSION_SECRET` secrets, signed secure cookies, asset headers, preview/production environments, and rollback instructions. | Anonymous requests cannot retrieve HTML, WASM, DAT, bitmap, save, or multiplayer endpoints; valid login survives navigation; invalid/expired/tampered sessions fail closed; logout works; Astra medium verifies gameplay and bitmap evidence through the deployed URL in current Chrome, Firefox, and Safari. |

### Browser performance budgets

- Startup: menu interactive within 3 seconds at 50 Mbps and 30 ms RTT.
- Simulation: ≤4 ms per native tick and ≤12 ms per WASM tick at 1,000 fleets.
- Rendering: ≤8 ms per 1280×800 galaxy frame at the reference fixture.
- Distribution: optimized WASM ≤5 MB; initial resource requests ≤4.
- Memory: combined JS heap and WASM memory ≤256 MB after 10 minutes.
- Regression policy: fail CI when a tracked metric regresses by more than 10%.

### Multiplayer target architecture

Single-player should execute through the same validated `Command` boundary used
by multiplayer. A server owns the authoritative simulation and emits
fog-filtered `Delta` or `Snapshot` messages per faction; clients never receive a
complete hidden `GameWorld`. Each command carries `{player, tick, sequence}` and
is validated and rate-limited server-side. Begin with an in-process transport
for deterministic tests and secure WebSockets for deployment. Add WebRTC only
as an optional trusted lockstep mode after deterministic replay is proven.

The existing `net_protocol.rs` is a notification vocabulary, not a complete
network envelope. Isolate protocol and server responsibilities in dedicated
`rebellion-net` and `rebellion-server` crates rather than coupling sockets to UI
panel actions.

## Feature-by-feature acceptance plan

Each row is a bounded pass. A pass closes only after the user-visible behavior
and underlying state mutation are both demonstrated.

| Pass | Features | Required acceptance |
|------|----------|---------------------|
| P00 | Supported scope | Define native and WASM contracts. Explicitly include or exclude browser audio, video, advisor, EData, and mods. |
| P01 | Build infrastructure | Formatting, warning-free all-target check, strict clippy, all required tests, native build, actual WASM packaging, and artifact inspection. |
| P02 | Data and startup | Every required/optional DAT table, string names, entity counts, troop classes, clean boot, and missing/corrupt-data errors. |
| P03 | Main menu | New Game, Load Game, Quit, version display, keyboard/mouse focus, and responsive layout. |
| P04 | Game setup | Both factions, three difficulties, every supported galaxy size, disabled states, Back, Start Campaign, and clean second-campaign reset. |
| P05 | Clock | Pause and every speed, focus loss, browser background/resume, modal/combat/cutscene tick behavior. |
| P06 | Galaxy navigation | Pan, wheel zoom, reset, system selection, right-click menu, resizing, high-DPI scale, and cockpit input boundaries. |
| P07 | Fog and overlays | Both factions, sensor radius, recon intelligence, fleet/facility/system overlays, labels, grid, and no hidden-information leakage. |
| P08 | Officers | Every character portrait/state, selection, assignment, availability, captivity, injury, death, and detail refresh. |
| P09 | Fleets | Fleet and fighter icons, selection, move, cancel, split/merge, invalid operations, arrivals, duplicate prevention, and post-combat refresh. |
| P10 | Economy | Income, collection, support drift, maintenance, shortfall, incidents, nonnegative invariants, and faction ownership effects. |
| P11 | Manufacturing | Enqueue, cancel, prioritize, capacity, costs, each product category, blocked production, completion, and usable world insertion. |
| P12 | Diplomacy | Legal targets, probability boundaries, success/failure, support/control change, cancellation, messages, and persistence. |
| P13 | Recruitment | Eligibility, success/failure, unique recruitment, repeated-order prevention, cancellation, telemetry, and persistence. |
| P14 | Espionage | Legal targeting, intelligence visibility, success/failure, foiling, informants, messages, and persistence. |
| P15 | Sabotage | Normal and Death Star sabotage, damage/delay, defense interaction, success/failure, telemetry, and persistence. |
| P16 | Character operations | Assassination, abduction, rescue, capture, health, death cleanup, escape, decoys, foiling, and autoscrap. |
| P17 | Uprisings and betrayal | Incite, subdue, thresholds, occupation, control transitions, allegiance changes, suppression, and telemetry. |
| P18 | Research | Ship/troop/facility trees, assignment exclusivity, progression, unlock events, no duplicate projects, and mid-project save/load. |
| P19 | Jedi | Force discovery, eligibility, training tiers, trainer loss, captive/dead states, story interactions, and persistence. |
| P20 | Movement | Distance timing, Han bonus, order/cancel/reorder, references, arrival, battle triggering, and no duplicate orders. |
| P21 | Blockade and repair | Enter/exit, ownership/economy effects, breach outcomes, hull recovery, cost/cap, interruptions, and persistence. |
| P22 | AI | Both factions; validator pass/reject boundaries; budgets; research; production; troop deployment; recon; defense; retreat; target deconfliction; Death Star escort/targeting. |
| P23 | Core space combat | Seven phases, weapon classes, shields, ion effects, recharge, carriers, fighters, officers, Emperor, retreat, destruction, and result application. |
| P24 | Tactical space combat | Placement, selection, formations, movement, focus fire, pause/speed, retreat, visual state, accepted formulas, and galaxy result application. |
| P25 | Ground combat | Troop attack/defense, facilities, officers/difficulty, selection, casualties, conquest, visuals, and parity between automatic and interactive paths. |
| P26 | Bombardment | Eligibility, shields, losses, popularity, ownership, messages, persistence, and visual feedback. |
| P27 | Death Star | Construction, sabotage, escort, retreat, shielding, targeting, firing, cooldown, destruction, cleanup, and both victory interactions. |
| P28 | Generic events | Every condition/action branch, one-shot behavior, simultaneous events, notification art, state changes, and save/load. |
| P29 | Story and cutscenes | Every story chain, all eight cutscene mappings, heritage branches, queueing, audio/video sync, skip/end/error behavior, and replay prevention. |
| P30 | Victory and defeat | Every win/loss condition for both player factions, simulation freeze, result screen, cutscene policy, Continue, and clean restart. |
| P31 | Save/load/delete | UI actions, slot refresh, populated round-trip, native restart, browser restart, corruption, compatibility, quota errors, delete, and deterministic continuation. |
| P32 | Mods | Discovery, dependency order, cycles, versions, enable/disable/reload, New Game reapplication, save mismatch, hot reload, and additive-feature scope. |
| P33 | Audio | Music, SFX, voices, context transitions, gain/mute, missing files/devices, browser user-gesture policy, and platform scope. |
| P34 | Droid advisors | Both factions, every decoded sequence format, correct frame IDs, timing, priority, missing frames, messages, and browser behavior. |
| P35 | Encyclopedia and EData | Every tab/category/entity, correct identity, search/sort, navigation, system focus, HD/original fallback, and browser loading. |
| P36 | Bitmap sweep | Every bitmap-bearing screen using the protocol below, original-only and partial-HD configurations, both factions, supported resolutions, and both platforms. |
| P37 | Campaign acceptance | Short smoke runs and long multi-seed campaigns for both factions/difficulties with bounded fleet/event growth, balance, diversity, victory, and full parity reports. |
| P38 | Release artifacts | Fresh native install and deployed browser package, exact artifact contents, startup/storage/media/input tests, and documentation generated from results. |
| P39 | Protected Cloudflare deployment | Preview and production Pages deployments, secret-backed password gate, signed session cookie, logout/expiry/tamper tests, cache/security headers, asset/API access denial before authentication, deployed single-player/multiplayer smoke tests, rollback, and retained Astra evidence. |

## Bitmap proof protocol

A bitmap passes only when all of the following evidence exists:

1. A resource-ledger entry identifies the screen, state, DLL source, resource ID,
   expected entity or artwork, dimensions, and platform.
2. The manifest entry reconciles with an existing file and the file decodes.
3. Runtime logs show the correct native path or browser HTTP request and cache key.
4. A screenshot shows the correct image, aspect ratio, state, layering, and fallback.
5. The related control or state transition works; rendering alone is insufficient.
6. Original-only, valid-HD, missing-HD, and corrupt-HD cases are exercised.

Required bitmap-bearing surfaces:

- Galaxy cockpit background and both factions' cockpit buttons
- Officer portraits
- Capital-ship and fighter miniatures in fleet panels
- Encyclopedia miniatures and EData art
- Story and generic event artwork
- Tactical task-force panels, gauges, hull/shield detail, and ship art
- Alliance and Imperial droid-advisor frames
- Cutscene display frames where applicable

## Evidence contract

Every feature result must record:

- Feature ID and commit SHA
- Platform, OS/browser version, build profile, data hashes, and asset hashes
- Faction, difficulty, galaxy size, seed, and fixture
- Initial state and exact user inputs
- Expected and observed state mutation
- Command, output, and exit status
- Browser console and network logs when applicable
- Screenshot or recording for visual/input behavior
- Negative and error-path case
- Final disposition: `pass`, `fail`, `blocked`, or explicitly `excluded`

No feature passes merely because a panel opens, code compiles, or a placeholder
or fallback appears.

## Astra orchestration loop

Use GPT-6-Astra at low effort for routine reproduction and medium effort for
cross-system diagnosis. Reviewer and debugger runs remain read-only until a fix
is explicitly authorized.

```bash
ASTRA=/Users/tomdimino/.claude/skills/codex-orchestrator/scripts/codex-astra.sh

$ASTRA reviewer \
  "Audit FEATURE_ID using its acceptance case. Do not modify files. Reproduce it on native and WASM and attach command, state, console, network, and screenshot evidence." \
  --reasoning low --service-tier default --no-approve

$ASTRA debugger \
  "Diagnose failed FEATURE_ID from the attached reproduction. Identify the smallest coherent fix and adjacent regression tests. Do not modify files." \
  --reasoning medium --service-tier default --no-approve
```

After an authorized fix:

1. Rerun the original reproduction.
2. Run adjacent regression cases.
3. Have an Astra reviewer confirm the evidence independently.
4. Update the JSON ledger.
5. Close only that feature ID.

## Definition of 100% functional

The claim is permitted only when:

- Every supported feature ID through P39 is `pass` on every claimed platform.
- All P0 and P1 findings are closed.
- Required tests have no failures or unexplained skips.
- Format, warning-free check, strict clippy, native build, and packaged WASM
  build all pass.
- Bitmap/resource reconciliation has zero unexplained misses.
- Multi-seed campaigns satisfy bounded-growth, balance, diversity, parity, and
  victory criteria.
- The same results are reproduced from release artifacts rather than development
  directories.
- Any unsupported platform feature is explicitly documented and excluded from
  the corresponding completion percentage.
