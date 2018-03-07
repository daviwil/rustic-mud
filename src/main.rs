#[macro_use] extern crate log;
extern crate ws;
extern crate env_logger;

use std::thread;
use std::thread::sleep;
use std::time::Duration;

use ws::{ connect, listen, CloseCode };

fn main() {
    env_logger::init();

    // Server thread
    let server = thread::spawn(move || {
        info!("Creating listener...");
        listen("127.0.0.1:3012", |out| {
            move |msg| {
                debug!("Server got message '{}'. ", msg);
                out.send(msg)
            }
        }).unwrap()
    });

    // Give the server a little time to get going
    sleep(Duration::from_millis(10));

    // Client thread
    let client = thread::spawn(move || {
        connect("ws://127.0.0.1:3012", |out| {
            out.send("Hello WebSocket").unwrap();
            move |msg| {
                debug!("Client got message '{}'. ", msg);
                out.close(CloseCode::Normal)
            }
        }).unwrap()
    });

    server.join().unwrap();
    client.join().unwrap();

    info!("Exiting...")
}
