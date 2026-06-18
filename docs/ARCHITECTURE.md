# Architecture

This document describes the structural design of the game as it evolves.

---

## Overview

Top-down 2D economy simulation built with Bevy 0.18.1. The player starts in an overworld, walks into a building, sits at a computer, and works a job (currently: reading and replying to emails).

---

## Game States

```
Overworld  →(walk into building)→  Office  →(press E at computer)→  EmailMinigame
              ←(walk south)←                     ←(Esc)←
```

Defined in `src/state.rs` as `GameState { Overworld, Office, EmailMinigame }`.

Each scene owns an `*Entity` marker component. An `OnExit` cleanup system despawns all marked entities when leaving the scene — no manual tracking needed.

---

## Modules

| Module | Responsibility |
|--------|---------------|
| `src/main.rs` | `App` entry point, plugin registration |
| `src/state.rs` | `GameState` enum |
| `src/shared.rs` | `Player`, `Collider`, `resolve_aabb`, `PLAYER_SPEED`, `PLAYER_SIZE` |
| `src/overworld.rs` | `OverworldPlugin` — exterior scene, door trigger |
| `src/office.rs` | `OfficePlugin` — interior scene, desk layout, computer interaction |
| `src/email.rs` | `EmailPlugin` — email UI, `EmailState` resource, reply handling |

---

## Entities & Components

| Component | Module | Description |
|-----------|--------|-------------|
| `Player` | `shared` | Marker on the player entity. Green 32×32. |
| `Collider` | `shared` | AABB half-extents (`half_w`, `half_h`). Any solid entity. |
| `Building` | `overworld` | Marker on the exterior building. |
| `OverworldEntity` | `overworld` | Tags all entities spawned in the overworld scene for cleanup. |
| `OfficeFurniture` | `office` | Tags solid obstacles in the office (walls, desks). |
| `PlayerComputer` | `office` | Marks the interactable computer (bright blue, player's desk). |
| `OfficeEntity` | `office` | Tags all entities spawned in the office scene for cleanup. |
| `EmailUiRoot` | `email` | Root UI node; despawned on `OnExit(EmailMinigame)`. |
| `EmailBodyText` | `email` | The text node showing current email from/subject/body. |
| `ReplyText(i)` | `email` | One of the three reply option labels. |
| `DoneText` | `email` | "Inbox empty" message, hidden until all emails are replied. |

> NPC entities will use an `Npc` marker component. Convention: blue 32×32 sprites.

---

## Systems

| System | Schedule | State | Description |
|--------|----------|-------|-------------|
| `setup_overworld` | `OnEnter` | `Overworld` | Spawns camera, player, building, door highlight |
| `cleanup_overworld` | `OnExit` | `Overworld` | Despawns all `OverworldEntity` entities |
| `move_player_overworld` | `Update` | `Overworld` | WASD movement + AABB collision vs. building |
| `check_building_entry` | `Update` | `Overworld` | Detects player in entrance zone → transitions to `Office` |
| `setup_office` | `OnEnter` | `Office` | Spawns camera, player, floor, walls, desks, computers, hint |
| `cleanup_office` | `OnExit` | `Office` | Despawns all `OfficeEntity` entities |
| `move_player_office` | `Update` | `Office` | WASD movement + AABB collision vs. furniture; south edge → `Overworld` |
| `check_computer_interact` | `Update` | `Office` | `E` key within range of `PlayerComputer` → `EmailMinigame` |
| `setup_email_ui` | `OnEnter` | `EmailMinigame` | Builds full-screen email UI |
| `cleanup_email_ui` | `OnExit` | `EmailMinigame` | Despawns `EmailUiRoot` |
| `handle_reply_input` | `Update` | `EmailMinigame` | `1/2/3` advance email, `Esc` exits |
| `update_email_display` | `Update` | `EmailMinigame` | Refreshes UI text when `EmailState` changes |

---

## Resources

| Resource | Module | Description |
|----------|--------|-------------|
| `EmailState` | `email` | `current: usize` index into inbox, `all_done: bool` |

---

## Data

`INBOX` in `src/email.rs` is a `&[Email]` constant. Each `Email` has `from`, `subject`, `body`, and `replies: [&str; 3]`. Currently 4 emails. Adding more is a one-liner in the array.

---

## Constants

| Constant | Value | Location |
|----------|-------|----------|
| `PLAYER_SPEED` | `200.0` px/s | `shared` |
| `PLAYER_SIZE` | `32.0` px | `shared` |
| `BUILDING_W/H` | `120×80` px | `overworld` |
| `BUILDING_X/Y` | `(-480, 0)` | `overworld` |
| `OFFICE_W/H` | `800×500` px | `office` |
| `DESK_W/H` | `80×50` px | `office` |
| `INTERACT_RANGE` | `60.0` px | `office` |

---

## Planned Modules

- `src/npc.rs` — NPC components and behavior
- `src/economy.rs` — supply/demand simulation logic
- `src/ui.rs` — HUD and economy stat overlays
