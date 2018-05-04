# t4lwm
Tiled 4 Life Window Manager
A window manager I am writing to help me learn Rust.

XRS will be moved to its own library and repository in the future. It is just too convenient to keep them together right now.

# TODO
    - XRB: XCB, but for Rust
        - Fix LISTofVALUE and BITMASK usage (once I figure out how bitmasks work too, I don't fully understand them yet)
        - Give all enums get_value for mappings (sample in BitGravity)
        - Convert that shit to Rust
        - Don't unwrap and panic everywhere
    - Window Manager
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
