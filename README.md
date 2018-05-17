# t4lwm
Tiled 4 Life Window Manager
A window manager I am writing to help me learn Rust. Uses XRB (X Rust Bindings) for a pure rust implementation.

# How To Run / Develop
```
git clone https://github.com/DaMrNelson/t4lwm # This repo
git clone https://github.com/DaMrNelson/xrb # X11 bindings, still in development so I am using them locally
cd t4lwm
cargo run
```

# TODO
- Display window with title
- Tile
- Move around the tiles
- Resize the tiles
- Workspaces
- Different tiling layouts
- Time in bottom right corner
- Config options
- Customizable in the bottom right corner. Maybe markdown or something? Idk, just something with buttons that you can customize and use well.
- Include compositor (later, but make it optional)
    - When you do this, make sure you use the X Composite Extension
