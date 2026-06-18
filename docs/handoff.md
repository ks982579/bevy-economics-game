# Handoff

> **Superseded by CHANGELOG.md** — use that for current status, bug history, and backlog.
> This file is kept for historical reference only.

Quick summary of where the project is and what to do next.

---

## What is done

### Game loop (playable)
- **Overworld** (`src/overworld.rs`) — player (green square) walks around. A building sits left of centre. Walking into the door entrance zone transitions to the Office.
- **Office** (`src/office.rs`) — interior with 4 desks, walls, and a bright-blue player computer. WASD to move, `E` near the computer to open email. Walking south exits back to the overworld.
- **Email minigame** (`src/email.rs`) — full-screen inbox UI. Shows one email at a time (from/subject/body). Press `1`, `2`, or `3` to pick a reply and advance. `Esc` closes. 4 emails currently in the inbox.

### Infrastructure
- `GameState` enum in `src/state.rs` drives all scene transitions
- `src/shared.rs` holds `Player`, `Collider`, `resolve_aabb` — shared across scenes
- Each state is a self-contained plugin: owns its `OnEnter` setup, `OnExit` cleanup, and `Update` systems
- 13 passing tests covering collision, state transitions, interaction triggers, and email flow

---

## Bugs fixed

**Grey screen when pressing E at the computer.**
Cause: each scene was spawning and despawning its own `Camera2d`. When leaving `Office`, the camera was despawned; `EmailMinigame`'s `OnEnter` spawned a new one in the same frame but the UI layout pass ran before it was ready — grey screen.
Fix: `Camera2d` is now spawned once in `main.rs` `Startup` and is never owned by a scene. Scene plugins must not spawn or despawn the camera.

**Cannot exit the building (walk south).**
Cause: the bottom wall had a `Collider` component tagged `OfficeFurniture`. The AABB resolution pushed the player back before they could reach the exit threshold (`y < -OFFICE_H / 2.0`), so the trigger could never fire.
Fix: the bottom wall is now visual-only (no `Collider`). The exit fires when `new_pos.y < -OFFICE_H / 2.0 + PLAYER_SIZE`, which the player can reach normally by walking south.

---

## What to do next

### High priority
- [ ] **`EmailState` is never reset** — replying to all 4 emails sets `all_done = true` permanently. Re-entering the minigame shows an empty inbox. Reset `EmailState` on `OnEnter(EmailMinigame)` or when leaving.
- [ ] **Player position not preserved across state transitions** — the player always respawns at a fixed position. Consider passing spawn position through a resource so the player reappears where they entered.
- [ ] **Refactor: email module is getting complex** — `update_email_display` has interleaved concerns (body text + reply buttons + done state). Consider splitting into smaller focused systems.

### Features (backlog order)
- [ ] NPC entities (blue squares) that wander the overworld
- [ ] More buildings / world objects on the overworld
- [ ] Economy layer: money resource, displayed in a HUD
- [ ] Job progression: more minigames (spreadsheet puzzle, scheduling, etc.)
- [ ] Dialogue system for talking to NPCs
- [ ] Save/load `EmailState` and economy state

---

## Architecture reminder

Each new state = new file = new plugin. See `CLAUDE.md` → *Code Style & Modularity* for the full rule. The camera rule above is the most important one to follow.
