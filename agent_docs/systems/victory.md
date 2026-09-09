---
title: "Victory System"
description: "Game-ending condition detection for Alliance and Empire"
category: "agent-docs"
created: 2026-03-14
updated: 2026-09-09
tags: [victory, hq-capture, death-star, simulation]
---

# Victory System

`victory.rs` — Game-ending condition detection.

## Types

| Type | Purpose |
|------|---------|
| `VictoryState` | `{ alliance_hq, empire_hq, death_star_active, death_star_location, resolved }` |
| `VictoryOutcome` | HqCaptured / DeathStarVictory / DeathStarDestroyed |

## API

```rust
// Each tick: check all victory conditions
if let Some(outcome) = VictorySystem::check(
    &victory_state,
    &world,
    &tick_events,
    campaign_config.victory_conditions,
) {
    victory_state.resolved = true;
    // show victory/defeat screen
}
```

## Victory Conditions

The active `VictoryConditions` selects one of two rule sets.

### Standard

Checked in priority order (first match wins):

### 1. Death Star Conditions (checked first, supersede fleet-presence)

- **DeathStarVictory**: Death Star at Alliance HQ AND `sys.is_destroyed` → Empire wins
- **DeathStarDestroyed**: No Empire Death Star fleet at `death_star_location` → Alliance wins

Only checked when `death_star_active = true`.

### 2. HQ and principal-leader capture

- **Empire victory**: Empire occupies `alliance_hq` and holds both Luke Skywalker and Mon Mothma captive.
- **Alliance victory**: Alliance occupies `empire_hq` and holds both Emperor Palpatine and Darth Vader captive.

Holding the leaders without the headquarters, using the wrong captor, or
occupying the headquarters without both leaders does not end a Standard game.

### Headquarters Only

- Empire occupation of `alliance_hq` is sufficient for an Empire victory.
- Alliance occupation of `empire_hq` is sufficient for an Alliance victory.
- Death Star outcomes are ignored; “Only” is enforced literally.

## Initialization

`VictoryState::new(alliance_hq, empire_hq)` — both SystemKeys found at startup by scanning for `sys.is_headquarters && controlling_faction == Alliance/Empire`.

## Lifecycle

1. `VictorySystem::check()` returns `None` every tick until a terminal condition is met
2. First `Some(VictoryOutcome)` → caller sets `resolved = true`
3. Subsequent calls return `None` (resolved flag prevents re-firing)

## Source

- `entity-system.md §4.2` — `SideVictoryConditionsNotif`, `FinalBattle`
- Event IDs `0x12c`/`0x180`
- `System::is_headquarters` flag
- [`main-menu-parity.md`](../main-menu-parity.md) — original game-type selector and setup contract
- [F-016B evidence](../../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-game-setup-propagation.md)
