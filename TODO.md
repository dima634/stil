## Features
- Power Ctl
- Notifications
- App launcher
- Keyboard layout

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