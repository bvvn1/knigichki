use rdev::{self, Event, listen};

fn main() {
    //if let Err(err) = grab(callback) {
    //    println!("{:?}", err)
    //};

    match listen(callback) {
        Ok(key) => println!("{:?}", key),
        Err(e) => println!("{:?}", e),
    }
}

fn callback(event: Event) {
    println!("My callback {:?}", event);
    match event.name {
        Some(string) => println!("User wrote {:?}", string),
        None => (),
    }
}
