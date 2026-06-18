# Changelog

All notable changes and ideas for this project will be tracked here.

---

## [Unreleased]

### Added
- `GameState` enum (`Overworld`, `Office`, `EmailMinigame`) in `src/state.rs`
- `src/shared.rs` — shared `Player`, `Collider`, `resolve_aabb`, movement constants
- Codebase split into modules: `overworld`, `office`, `email`, `shared`, `state`
- **Office scene**: interior with 4 desks, walls, a bright-blue player computer
- **Building entry**: walking into the door on the overworld transitions to `Office`
- **Office exit**: walking south out of the office returns to `Overworld`
- **Computer interaction**: press `E` near the player's computer → `EmailMinigame`
- **Email minigame**: full-screen UI showing inbox emails; press `1/2/3` to reply, `Esc` to close
- 4 sample emails in the inbox (`INBOX` constant in `src/email.rs`)
- `EmailState` resource tracks current email index and completion
- Scene cleanup via `*Entity` marker components + `OnExit` systems

### Ideas / Backlog
- Add NPC entities (blue squares) with basic pathfinding
- Add more buildings and world objects to the overworld
- Economy entities: shops, traders, resources, inventory
- Simple supply/demand simulation
- UI overlay showing economy stats / money
- Job progression: more minigames beyond email (e.g. spreadsheet puzzle, meetings)
- Dialogue system for talking to NPCs
- Save/load game state

---

## [0.2.0] — 2026-06-18

### Added
- Green 32×32 player square spawned at world origin
- WASD movement with delta-time scaling (200 px/s), diagonal movement normalized
- `Player` component; NPC color convention: blue squares
- Dark grey building (120×80) positioned left of centre at x=-480
- `Collider` component (AABB half-extents) on player and building
- AABB collision resolution — player cannot walk through the building

---

## [0.1.0] — 2026-05-08

### Added
- Initial Bevy 0.18.1 project scaffold
- Window titled "Economy Sim" at 1280×720
- 2D camera spawned at startup
- Cargo features scoped to 2D rendering, UI/text, and X11 only (WSL2-compatible)
- Fast debug compile profile (`opt-level = 1` own code, `opt-level = 3` deps)
