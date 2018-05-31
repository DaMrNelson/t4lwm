extern crate xrb;

use xrb::XClient;

mod manager;
mod settings;
mod tiling;

fn main() {
    // Connect
    //let mut client = XClient::new(String::from("/tmp/.X11-unix/X1"));
    let mut manager = manager::WindowManager::new(
        XClient::connect(String::from("/tmp/.X11-unix/X9")),
        String::from(":9")
    );
    manager.run();
}