# Changelog

All notable changes, bug fixes, and ideas are tracked here.
Version stays at 0.1.0 until explicitly bumped.

---

## [Unreleased]

### Added

- Context-save position system: `OverworldContext` and `OfficeContext` resources in `shared.rs` persist the player's last position across state transitions
- Each scene now spawns the player at the saved position instead of a hardcoded default
- Position is saved at the moment of transition (building entry, office exit, computer interaction)
- 4 new tests covering context save and context restore for both overworld and office

### Ideas / Backlog

- Reset `EmailState` when re-entering the minigame (currently `all_done` stays true permanently)
- Preserve player position across state transitions (always respawns at fixed position)
- Add NPC entities (blue squares) wandering the overworld
- Add more buildings and world objects to the overworld
- Economy layer: money resource displayed in a HUD
- Job progression: more minigames beyond email (spreadsheet puzzle, meeting scheduler, etc.)
- Dialogue system for talking to NPCs
- Save/load game state

---

## [0.1.0] — 2026-06-18

### Summary

First playable loop: walk into a building, sit at a computer, read and reply to emails.

### Added

- Bevy 0.18.1 project scaffold; window "Economy Sim" 1280×720; WSL2/X11-compatible feature flags
- Fast debug compile profile (`opt-level = 1` own code, `opt-level = 3` deps)
- `GameState` enum (`Overworld`, `Office`, `EmailMinigame`) driving all scene transitions
- `src/shared.rs` — `Player`, `Collider`, `resolve_aabb`, movement constants shared across scenes
- Plugin-per-scene architecture: each state is a self-contained Bevy plugin with its own setup, cleanup, and update systems
- **Overworld**: green 32×32 player, dark-grey building at x=-480, WASD movement, AABB collision, door entry trigger
- **Office**: interior with 4 desks, solid top/left/right walls, bright-blue player computer, `E` interaction trigger, south-wall exit
- **Email minigame**: full-screen UI inbox; shows one email at a time (from/subject/body); press `1`/`2`/`3` to reply and advance, `Esc` to close; 4 sample emails
- `EmailState` resource tracking current email index and completion
- Scene cleanup via `*Entity` marker components + `OnExit` despawn systems
- Persistent `Camera2d` spawned once in `main.rs` (never per-scene)
- 13 unit tests covering collision, state transitions, interaction range, and email flow

### Fixed

- **Grey screen on `E`**: each scene was spawning/despawning its own `Camera2d`; a one-frame gap with no camera caused the renderer to produce a grey frame. Fixed by making the camera persistent in `main.rs`.
- **Cannot exit building**: bottom office wall had a `Collider` blocking the player before they could reach the exit threshold. Fixed by making the bottom wall visual-only (no collider).
- **Email UI invisible (grey screen)**: `bevy_ui` only provides the layout engine — the GPU draw calls live in the separate `bevy_ui_render` feature. Adding `"bevy_ui_render"` to `Cargo.toml` fixed it. No compile errors; the symptom was total silence from the renderer.
