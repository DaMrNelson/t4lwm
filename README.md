# t4lwm
Tiled 4 Life Window Manager
A window manager I am writing to help me learn Rust. Uses XRB (X Rust Bindings) for a pure rust implementation.

Not much happening here right now, since I am still focusing on getting XRB working.
XRB will probably be used as a crate and not packaged in in the future. Right now it is just too convenient to keep it here, though.

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
