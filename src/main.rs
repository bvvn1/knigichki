use std::hash;

use rdev::{self, Event, EventType, listen};
use tokio::sync::mpsc;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    //if let Err(err) = grab(callback) {
    //    println!("{:?}", err)
    //};

    let (tx, rx) = mpsc::channel::<u8>(2);

    match listen(callback) {
        Ok(key) => println!("{:?}", key),
        Err(e) => println!("{:?}", e),
    }
}

fn callback(event: Event) {
    //println!("My callback {:?}", event);
    match event.event_type {
        //Some(string) => println!("User wrote {:?}", string),
        EventType::KeyPress(keypress) => println!("{:?}", keypress),
        _ => (),
    }
}
