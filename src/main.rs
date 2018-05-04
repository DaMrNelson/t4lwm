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

    // Create a window
    //println!("{:#?}", client.connect_info);
    println!("Win ID:  {}", window_id);
    let mut window = Window {
        depth: 255,
        wid: window_id, // Window's ID
        parent: client.connect_info.screens[0].root,
        x: 20,
        y: 200,
        width: 500,
        height: 500,
        border_width: 0,
        class: WindowInputType::InputOutput,
        visual_id: client.connect_info.screens[0].root_visual,
        bitmask: WINDOW_BITMASK_BACKGROUND_PIXMAP,
        values: vec![WindowValue {
            background_pixmap: client.connect_info.screens[0].white_pixel, // 0 = None, 1 = Parent Relative
            background_pixel: 0,
            border_pixmap: 0, // 0 = CopyFromParent
            border_pixel: 0,
            bit_gravity: BitGravity::Center,
            win_gravity: WindowGravity::Center,
            backing_store: WindowValueBackingStore::NotUseful,
            backing_planes: 0,
            backing_pixel: 0,
            override_redirect: false,
            save_under: false,
            event_mask: 0,
            do_not_propagate_mask: 0,
            colormap: 0, // 0 = CopyFromParent
            cursor: 0 // 0 = None
        }]
    };
    client.create_window(&window);

    thread::sleep(time::Duration::from_secs(60*60*60));
}
