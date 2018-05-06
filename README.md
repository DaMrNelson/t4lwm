# t4lwm
Tiled 4 Life Window Manager
A window manager I am writing to help me learn Rust.

XRS will be moved to its own library and repository in the future. It is just too convenient to keep them together right now.

# Usage
1. Do initial setup (create windows, subscribe to events, etc)
2. Run an event loop using client.wait_for_message()
    - Responds with replies, errors, and events

# TODO
    - XRB: XCB, but for Rust
        - read_keymap_notify
        - Thread lock when creating new resource IDs. Or maybe just thread lock the entire thing? Idk yet.
        - Allow re-use of used resource IDs
        - Don't unwrap and panic everywhere
        - Write some examples
        - Write some docs
            - Write manual docs for the important stuff
                - Search for things like "[len] [type] [name]\n[special index] [special value]
                    - Ie SelectionNotify's time and property. You can specify it, or you can leave it blank
                    - You can tell the difference because enums are "[len] [BLANK] [name]" while these are "[len] [type] [name]"
            - Use autodocs for the rest. The poor saps can rely on examples and intuition for a bit
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
