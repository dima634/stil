- Rewrite hyprland events listener in form of :
  ```rust
  for ev in hypr_events {
      // handle
  }
  ```
- Consider removing broadcast from system events dispatcher
- Batching requests to Hyprland
- PopupWindow background blur
- Anyhow?
- refactor init.rs
- Dispatch system events from Desktop?
- get rid of csv crate
- Rename QSystem -> QDesktop