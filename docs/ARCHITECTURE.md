# Architecture

This document describes the structural design of the game as it evolves.

---

## Overview

Top-down 2D economy simulation built with Bevy 0.18.1.

---

## Entities & Components

| Component | Description |
|-----------|-------------|
| `Player`  | Marker component on the player entity. Green 32×32 sprite. |

> NPC entities will use a `Npc` marker component. Convention: blue 32×32 sprites.

---

## Systems

| System | Schedule | Description |
|--------|----------|-------------|
| `setup` | `Startup` | Spawns camera and player |
| `move_player` | `Update` | Reads WASD input, moves player transform |

---

## Modules

Currently all code lives in `src/main.rs`. As the project grows, split into:

- `src/player.rs` — player component, spawn, movement
- `src/npc.rs` — NPC components and behavior
- `src/economy.rs` — supply/demand simulation logic
- `src/ui.rs` — HUD and economy stat overlays

---

## Constants

| Constant | Value | Purpose |
|----------|-------|---------|
| `PLAYER_SPEED` | `200.0` px/s | Player movement speed |
| `PLAYER_SIZE`  | `32.0` px    | Player sprite dimensions |
