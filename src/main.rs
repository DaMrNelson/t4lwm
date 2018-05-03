mod xrs;

use xrs::XClient;

fn main() {
    let mut client = XClient::new(String::from("/tmp/.X11-unix/X1"));
    //let mut client = XClient::new(String::from("/tmp/.X11-unix/X9"));
    client.connect();
}
