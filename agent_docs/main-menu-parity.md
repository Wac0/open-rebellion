---
title: "Original Shuttle Cockpit Main Menu"
description: "Binary-confirmed 640x480 geometry, bitmap resources, settings, actions, and acceptance criteria for the Star Wars Rebellion main menu"
category: agent-docs
created: 2026-09-09
updated: 2026-09-09
tags: [main-menu, ui, common-dll, ghidra, parity, bitmap]
---

# Original Shuttle Cockpit Main Menu

This is the canonical implementation reference for the original *Star Wars
Rebellion* shuttle-cockpit menu. Use it for P03 (main menu), P04 (game setup),
and every visual test of `crates/rebellion-render/src/main_menu.rs`.

## Ground truth

The original menu is not a text menu placed over a background. `COMMON.DLL`
bitmap 20001 is only the empty 640x480 cockpit shell. `REBEXE.EXE` creates
separate controls over its monitor apertures, loads the resting and animated
bitmaps listed below, and sends commands when those controls are clicked.

The mapping was recovered from these owned original files and checked against
the repository reference screenshot:

| Source | SHA-256 | What it establishes |
|---|---|---|
| `REBEXE.EXE` | `b3fe3997cab9a6e96403d638875dcba25484e4d8601751afec748471ac0ed6ab` | Control construction, geometry, command handling, settings passed to game initialization |
| `COMMON.DLL` | `33bfbc7593f25971d63fe53b0122b8a9ae30823982e921584ce0cd34892671b8` | Resting, selected, pressed, and animation frames |
| [`data/base/ui/common-dll/BMP/20001.bmp`](../data/base/ui/common-dll/BMP/20001.bmp) | `3e5b89c0745596d2850f25b7c5d4008e1d77662f0e9966593ed192b61242fc2d` | Empty cockpit shell |
| [`assets/rebellion-menu.jpg`](../assets/rebellion-menu.jpg) | `069bb9ff9d3f3c99337e4a04a8b917bc255d433723972c203273da7697a266e8` | Fully assembled reference appearance |
| `star-wars-rebellion/MDATA/MDATA.302` (owned install, not tracked) | `fbd5e5772e5bbe7b3d82157cc60722972782aae22dd67cc7ff3259d370d078e3` | Original main-title/Tatooine soundtrack used for the cockpit menu |

Relevant decompiled functions are `FUN_00405560` (control construction),
`FUN_00405050` (window and galaxy-size pointer handling), `FUN_00406000`
(commands), `FUN_00603870` (animated strobe control), and `FUN_00603aa0`
(sequential frame loading). The checked-in constructor is
[`ghidra/notes/FUN_00405560.c`](../ghidra/notes/FUN_00405560.c).

## Coordinate contract

All coordinates are logical pixels in the original 640x480 canvas. A modern
viewport must preserve 4:3 aspect ratio, center the canvas, letterbox surplus
space, and apply the same scale and offset to drawing and hit testing.

```text
┌────────────────────────────── 640 ──────────────────────────────┐
│  [Easy] [Intermediate] [Expert]                                │
│                                                                 │
│                              [Credits] [Multiplayer]             │
│                         [galaxy size screens]                    │
│             [Empire/start] [game type] [Alliance/start]         │
│ [Load/options]                                  [Quit/eject]     │
└────────────────────────────── 480 ──────────────────────────────┘
```

## Complete control map

`x`, `y`, `w`, and `h` come directly from `FUN_00405560`. Ranges are
inclusive resource sequences loaded by the original animated-control class.

| Control | Rect `(x,y,w,h)` | Resting/selected | Hover animation | Command | Result |
|---|---:|---:|---:|---:|---|
| Easy difficulty (X-wing) | `(61,41,51,36)` | `11273` | `11061–11090` | `0x6e` | difficulty `0`; scenario `1` or `4` |
| Intermediate difficulty (Star Destroyer) | `(124,40,49,36)` | `11275` | `11091–11120` | `0x6f` | difficulty `1`; scenario `2` or `5` |
| Expert difficulty (Death Star) | `(187,41,45,36)` | `11274` | `11121–11150` | `0x70` | difficulty `2`; scenario `3` or `6` |
| Galaxy-size lever | `(242,271,44,47)` | `10001–10003`, `10014–10015` | n/a | `0x6a` | cycles galaxy size |
| Small galaxy screen | `(290,293,24,21)` | `10017` | n/a | pointer region | encoded size `1` |
| Medium galaxy screen | `(326,293,24,21)` | `10018` | n/a | pointer region | encoded size `2` |
| Large galaxy screen | `(362,293,24,21)` | `10019` | n/a | pointer region | encoded size `3` |
| Game type (Cloud City) | `(305,333,42,30)` | `10158–10159` | n/a | `0x71` | Standard / Headquarters Only |
| Empire faction/start | `(153,308,62,55)` | `10009` | `11001–11015` | `0x66` | starts as Empire |
| Alliance faction/start | `(437,307,62,55)` | `10007` | `11031–11045` | `0x65` | starts as Alliance |
| Save/load and options | `(67,381,51,61)` | `10005` | `11151–11180` | `0x67` | opens save/options |
| Credits (CD) | `(411,232,40,37)` | `10013` | `11241–11255` | `0x68` | opens credits |
| Multiplayer hologram | `(459,242,33,28)` | `11271–11272` | two-state | `0x73` | enters head-to-head setup |
| Quit/ejection handle | `(536,393,63,64)` | `10011` | `11181–11210` | `0x69` | exits the game |

The original labels galaxy sizes Small, Medium, and Large. The current Rust
data enum retains the encoded values as `Standard = 1`, `Large = 2`, and
`Huge = 3`; UI parity maps original Small/Medium/Large to current
Standard/Large/Huge respectively without changing the data encoding.

## Defaults and transitions

| Setting | Original field | Default | Open Rebellion mapping |
|---|---:|---|---|
| Difficulty | `+0xf8` | `0`, Easy/X-wing | `Difficulty::Easy` |
| Galaxy size | `+0xec` | `1`, Small | `GalaxySize::Standard` |
| Game type | `+0xf4` | `1`, Standard Game | full victory conditions |
| Entry mode | `+0xf0` | `1`, new game | main cockpit |

The faction controls start immediately; there is no second custom setup page.
`FUN_00406000` passes scenario `difficulty + 1` for Alliance and
`difficulty + 4` for Empire. Game type `1` becomes Standard and `2` becomes
Headquarters Only. Galaxy selections use encoded values `1`, `2`, and `3`.

## Rendering and input requirements

1. Draw bitmap 20001 at native color without a dimming tint.
2. Populate every control aperture before pointer movement.
3. Treat palette-blue matte in the Common menu sprites as transparent.
4. Animate the matching sequence on hover and show a pressed state on pointer down.
5. Transform the exact logical rectangles above for pointer hit testing.
6. Preserve mouse behavior and add accessible keyboard focus without replacement text buttons.
7. Keep difficulty, galaxy size, and game type selection visible.
8. Start the selected faction directly; do not show the custom setup page.

## Feature boundaries

- **P03 — Main menu:** cockpit assembly, animation, save/options, credits,
  multiplayer, quit, accessibility, and responsive hit testing.
- **P04 — Game setup:** difficulty, galaxy size, game type, faction start,
  state propagation, and clean subsequent-campaign reset.

Headquarters Only affects victory rules, not only its label. P04 cannot pass
until it reaches active configuration, save data, and `VictorySystem`.

## Music contract

The cockpit menu uses `MDATA.302`, already named `MusicTrack::MainTheme` by
the audio layer. The licensed source is staged locally as
`data/sounds/music/main_theme.wav` and remains ignored by Git. The native menu
starts it when the cockpit becomes visible. The browser may preload it only
after the four-request startup gate and must begin looped playback on the first
pointer or keyboard gesture, as required by browser autoplay policy. Returning
to the menu restarts the same context; leaving for a campaign changes context
without overlapping tracks. A missing or undecodable file emits one bounded
diagnostic and leaves the menu usable.

## Astra acceptance matrix

Run through `codex-orchestrator` with GPT-6 Astra at medium effort. Retain
screenshots and a JSON result for every run.

| Gate | Required proof |
|---|---|
| Asset identity | 20001 and every listed control resource loads from `runtime.orpk`; zero missing-asset or deleted-texture errors |
| Resting composition | All apertures are populated before pointer movement and match the reference layout |
| Hover animation | Each animated control changes only through its correct resource family |
| Settings | All 3 difficulties, 3 galaxy sizes, and 2 game types visibly select and carry correct state |
| Campaign start | Both factions initialize selected settings without the custom setup screen |
| Navigation | Load/options, credits, multiplayer, and quit reach the correct destination or remain explicit failures |
| Pointer geometry | Center and edge clicks pass at 640x480, 1280x960, 1280x800, 1440x900, and a narrow supported viewport |
| Accessibility | Every control is keyboard reachable, named, single-activation, and visibly focused without covering art |
| Visual quality | No blank aperture, blue matte, stretch, clip, wrong sprite family, or text-button overlay |
| Runtime quality | Zero page, console, request, WebGL, missing-asset, or panic errors |
| Music | MDATA.302 loads once, begins after the browser gesture, loops without overlap, obeys gain/mute, and resumes correctly on return to menu |

P03 and P04 remain open until all applicable rows pass in native and browser
release artifacts.
