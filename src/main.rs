use rdev::{self, Event, grab};

fn main() {
    //if let Err(err) = grab(callback) {
    //    println!("{:?}", err)
    //};

    match grab(callback) {
        Ok(key) => println!("{:?}", key),
        Err(e) => println!("{:?}", e),
    }
}

fn callback(event: Event) -> Option<Event> {
    Some(event)
}
