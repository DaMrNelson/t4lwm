#!/usr/bin/python3
# pip install xlib

from Xlib import X, display, Xutil
from time import sleep

def main():
    d = display.Display(":9")
    screen = d.screen()

    # Background
    bgsize = 20
    bgpm = screen.root.create_pixmap( # Is this the step I am missing?
        bgsize,
        bgsize,
        screen.root_depth
    )
    bggc = screen.root.create_gc(
        foreground=screen.white_pixel,
        background=screen.black_pixel
    )
    bgpm.fill_rectangle(bggc, 0, 0, bgsize, bgsize)

    # Create the window
    window = screen.root.create_window(
        20, 200, 500, 500, 0,
        screen.root_depth,
        X.InputOutput,
        X.CopyFromParent,

        background_pixmap=bgpm,
        event_mask=(
            X.StructureNotifyMask |
            X.ButtonReleaseMask
        ),
        colormap=X.CopyFromParent
    )

    WM_DELETE_WINDOW = d.intern_atom("WM_DELETE_WINDOW")
    WM_PROTOCOLS = d.intern_atom("WM_PROTOCOLS")

    window.set_wm_name("Xlib example: childwin.py")
    window.set_wm_icon_name("childwin.py")
    window.set_wm_class("childwin", "XlibExample")

    window.set_wm_protocols([WM_DELETE_WINDOW])
    window.set_wm_hints(
        flags=Xutil.StateHint,
        initial_state=Xutil.NormalState
    )

    window.set_wm_normal_hints(
        flags=(Xutil.PPosition | Xutil.PSize | Xutil.PMinSize),
        min_width=50,
        min_height=50
    )

    # Map the window, making it visible
    window.map()

    # Sleep for a while
    sleep(60 * 60 * 60)

if __name__ == "__main__":
    main()
