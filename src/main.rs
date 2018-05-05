mod xrs;

use xrs::XClient;
use xrs::models::*;

use std::{thread, time};

fn main() {
    // Connect
    //let mut client = XClient::new(String::from("/tmp/.X11-unix/X1"));
    let mut client = XClient::new(String::from("/tmp/.X11-unix/X9"));
    client.connect();

    ///////////////////////////////////
    //// TESTING
    ///////////////////////////////////

    // Create a pixmap
    let pixmap = Pixmap {
        depth: client.connect_info.screens[0].root_depth,
        pid: client.new_resource_id(),
        drawable: client.connect_info.screens[0].root,
        width: 20,
        height: 20
    };

    client.create_pixmap(pixmap);

    // Create GC (graphics context)
    let gc = GraphicsContext {
        cid: client.new_resource_id(),
        //drawable: client.connect_info.screens[0].root,
        drawable: 0x0000026d, // TODO: Determine, it isn't far off the above
        values: vec![
            GraphicsContextValue::Background(client.connect_info.screens[0].black_pixel),
            GraphicsContextValue::Foreground(client.connect_info.screens[0].white_pixel)
        ]
    };

    client.create_gc(gc);

    // Create a window
    let window = Window {
        depth: client.connect_info.screens[0].root_depth,
        wid: client.new_resource_id(),
        parent: client.connect_info.screens[0].root,
        x: 20,
        y: 200,
        width: 500,
        height: 500,
        border_width: 0,
        class: WindowInputType::InputOutput,
        visual_id: 0, // CopyFromParent
        values: vec![
            WindowValue::BackgroundPixmap(0x00200000),
            WindowValue::EventMask(Event::ButtonRelease.val() | Event::StructureNotify.val()),
            WindowValue::Colormap(0x0)
        ]
    };
    client.create_window(&window);

    // Map the window (make it visibile)
    client.map_window(window.wid);

    thread::sleep(time::Duration::from_secs(60*60*60));
}
