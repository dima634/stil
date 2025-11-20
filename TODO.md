- Rewrite hyprland events listener in form of :
  ```rust
  for ev in hypr_events {
      // handle
  }
  ```
- Consider removing broadcast from system events dispatcher
- Batching requests to Hyprland
- Figure out better way for global init
