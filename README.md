# bevy-economy-sim

A top-down 2D game built with Rust and Bevy.

## Project Setup

### Step 1 — Initialize the Rust project

`cargo init` creates the binary package and `Cargo.toml` in the current directory.

1. `cargo init`

---

### Step 2 — Add Bevy with explicit features

Bevy's default feature set includes both `wayland` and `bevy_gilrs` (gamepad), which require system libraries (`libwayland-client`, `libudev`) that are not present in a WSL2 environment. Disabling defaults and listing only what a 2D game needs avoids those missing dependencies.

1. Open `Cargo.toml` and replace the `[dependencies]` section with:
   ```toml
   bevy = { version = "0.18.1", default-features = false, features = [
     # app scaffolding
     "bevy_asset", "bevy_state", "bevy_window", "bevy_winit", "bevy_log",
     "default_font", "multi_threaded", "async_executor", "reflect_auto_register",
     # 2D rendering
     "bevy_core_pipeline", "bevy_render", "bevy_sprite",
     # UI & text
     "bevy_ui", "bevy_text",
     # X11 only (no wayland)
     "x11",
   ] }
   ```

---

### Step 3 — Configure fast debug compile times

Bevy has a large dependency tree. Setting `opt-level = 1` on your own code keeps debug info usable, while `opt-level = 3` on all dependencies means they compile fast and run at full speed during development.

1. Add the following to the bottom of `Cargo.toml`:
   ```toml
   [profile.dev]
   opt-level = 1

   [profile.dev.package."*"]
   opt-level = 3
   ```

---

### Step 4 — Write the entry point

A minimal Bevy app needs `DefaultPlugins` (window, input, renderer, etc.) and a `Camera2d` spawned at startup. Everything else is built on top of this.

1. Replace the contents of `src/main.rs` with:
   ```rust
   use bevy::prelude::*;

   fn main() {
       App::new()
           .add_plugins(DefaultPlugins.set(WindowPlugin {
               primary_window: Some(Window {
                   title: "Economy Sim".into(),
                   resolution: (1280_u32, 720_u32).into(),
                   ..default()
               }),
               ..default()
           }))
           .add_systems(Startup, setup)
           .run();
   }

   fn setup(mut commands: Commands) {
       commands.spawn(Camera2d);
   }
   ```

---

### Step 5 — Verify the build

Confirm everything compiles before running. On WSL2 you will need an X server forwarded from Windows (e.g. WSLg or VcXsrv with `DISPLAY` set) to actually open a window.

1. `cargo check`
