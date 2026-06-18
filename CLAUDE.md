# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
cargo build              # compile
cargo run                # launch the game (requires X11 display)
cargo test               # run all tests (headless, no display needed)
cargo test <name>        # run a single test by name substring
cargo clippy             # lint
```

## Architecture

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for component/system design.
See [CHANGELOG.md](CHANGELOG.md) for the backlog and ideas list.

### Module structure

| Module | Purpose |
|--------|---------|
| `src/main.rs` | App entry point, plugin registration |
| `src/state.rs` | `GameState` enum (`Overworld`, `Office`, `EmailMinigame`) |
| `src/shared.rs` | `Player`, `Collider`, `resolve_aabb`, shared constants |
| `src/overworld.rs` | Exterior scene — building, door trigger, WASD movement |
| `src/office.rs` | Interior scene — desks, player computer, `E` interaction |
| `src/email.rs` | Email minigame UI, `EmailState` resource, reply logic |

### Scene flow

```
Overworld  →(walk into building)→  Office  →(press E at computer)→  EmailMinigame
              ←(walk south)←                     ←(Esc)←
```

Scenes use an `*Entity` marker component (`OverworldEntity`, `OfficeEntity`) so `OnExit` cleanup systems can despawn everything with a single query.

**Entity rendering convention:** player = green 32×32, player's computer = bright blue 30×20, other computers = dark grey, desks = brown, buildings = dark grey. All rendered as `Mesh2d` + `MeshMaterial2d(ColorMaterial)`.

**Collision:** `Collider` stores AABB half-extents. `resolve_aabb` in `shared.rs` pushes out along the axis of least penetration. Any solid entity needs a `Collider`.

## Code Style & Modularity

Keep each game state in its own file. A new feature should touch at most one existing file plus its own new file. If a system or type is needed by two or more modules, put it in `shared.rs`.

**Each state is fully self-contained:**
- Tags every entity it spawns with a scene marker (`OfficeEntity`, etc.)
- Cleans up all its entities in `OnExit` via a single query on that marker
- Registers all its systems, resources, and events inside its own `Plugin::build`

The **camera is persistent** — spawned once in `main.rs` `Startup`, never owned by a scene. Do not spawn or despawn `Camera2d` inside scene plugins; doing so causes a one-frame gap where the renderer has no camera and the screen goes grey.

New states go in a new file (`src/<state_name>.rs`) registered as a plugin in `main.rs`. Keep files small enough that a single coding session can read the whole file in context.

## Platform Constraints (WSL2)

- Always keep `default-features = false` in Cargo.toml for Bevy
- Never add `wayland` or `bevy_gilrs` features — missing system libs in this environment
- X11 only; `bevy_winit` with `x11` feature is the correct window backend

## Bevy 0.18 Notes

- `Query::single_mut()` and `Query::single()` return `Result` — use `let Ok(val) = query.single_mut() else { return };`
- Colored rectangles use `Mesh2d` + `MeshMaterial2d(ColorMaterial)` — not `Sprite` (which requires an image handle)
- `ColorMaterial` and `MeshMaterial2d` are in `bevy::prelude::*` but require the `bevy_sprite_render` feature
- `Rectangle::new(w, h)` — no `splat` method; use `Rectangle::new(s, s)` for a square
- `StateScoped` component does not exist in 0.18 — use an `*Entity` marker + `OnExit` cleanup system instead
- Tests with `init_state` need `bevy::state::app::StatesPlugin` added alongside `MinimalPlugins`
- State transitions (`NextState::set`) take effect after the frame — assert the new state after **two** `app.update()` calls

## Controls Convention

- WASD — player movement
- `E` — interact with nearby object (computer)
- `1` / `2` / `3` — select reply in email minigame
- `Esc` — close/back
