## Features
- Notifications
- App launcher
- Pin apps
  - Launch apps

## Misc
- Rewrite hyprland events listener using iterator:
  ```rust
  for ev in hypr_events {
      // handle
  }
  ```
- Batching requests to Hyprland
- Anyhow?
- refactor init.rs
- get rid of csv crate
- remove reexport of tracing: add subscriber to taskbar crate, remove from core
