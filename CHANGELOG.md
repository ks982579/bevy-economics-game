# Changelog

All notable changes and ideas for this project will be tracked here.

---

## [Unreleased]

### Added
- Green 32×32 player square spawned at world origin
- WASD movement with delta-time scaling (200 px/s), diagonal movement normalized
- `Player` component for future query filtering; NPC color convention: blue squares

### Ideas / Backlog
- Add NPC entities (blue squares) with basic behavior
- Basic tile-based or free-movement world
- Economy entities: shops, traders, resources
- Simple supply/demand simulation
- UI overlay showing economy stats

---

## [0.1.0] — 2026-05-08

### Added
- Initial Bevy 0.18.1 project scaffold
- Window titled "Economy Sim" at 1280×720
- 2D camera spawned at startup
- Cargo features scoped to 2D rendering, UI/text, and X11 only (WSL2-compatible)
- Fast debug compile profile (`opt-level = 1` own code, `opt-level = 3` deps)
