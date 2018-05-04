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
    
    // Get an available window ID (however the fuck this works)
    // Vars defined as below
    // See libxcb/src/xcb_xid.c xcb_generate_id for this source
    let mut last = 0;
    let mut max = 0;
    let mut base = client.connect_info.resource_id_base;
    let mut inc = client.connect_info.resource_id_mask & -(client.connect_info.resource_id_mask as i32) as u32;
    
    if last >= max + 1 - inc {
        if last == 0 {
            max = client.connect_info.resource_id_mask;
        } else {
            // Doesn't go here
            panic!("But it did!");
        }
    } else {
        // Doesn't go here
        panic!("But it did!");
    }

    let window_id = last | base;

    // What I am doing:
    //     CreateWindow
    // What I maybe should be doing?
    //     CreatePixmap
    //     CreateGC
    //     PolyFillRectangle
    //     CreateWindow

    // Create a pixmap
    let mut pixmap = Pixmap {
        depth: client.connect_info.screens[0].root_depth,
        pid: 0x0, // TODO: Dynamic // TODO: Copy this from the working request
        drawable: client.connect_info.screens[0].root,
        width: 20,
        height: 20
    };

    client.create_pixmap(pixmap);

    // Create GC (graphics context)
    let mut gc = GraphicsContext {
        cid: 0, // TODO: Dynamic
        drawable: client.connect_info.screens[0].root,
        values: vec![
            GraphicsContextValue::Background(client.connect_info.screens[0].black_pixel),
            GraphicsContextValue::Foreground(client.connect_info.screens[0].white_pixel)
        ]
    };

    client.create_gc(gc);

    // Create a window
    //println!("{:#?}", client.connect_info);
    println!("Win ID:  {}", window_id);
    let mut window = Window {
        depth: client.connect_info.screens[0].root_depth,
        //wid: window_id, // Window's ID
        wid: 0x00200002, // TODO: Dynamic
        //parent: client.connect_info.screens[0].root,
        parent: 0x0000026d, // TODO: Dynamic
        x: 20,
        y: 200,
        width: 500,
        height: 500,
        border_width: 0,
        class: WindowInputType::InputOutput,
        //visual_id: client.connect_info.screens[0].root_visual,
        visual_id: 0, // CopyFromParent
        values: vec![]
    };
    client.create_window(&window);

    thread::sleep(time::Duration::from_secs(60*60*60));
}
