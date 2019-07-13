use std::{thread, time};

mod settings;
mod telegram;
mod datastore;
mod display;

fn main() {
    display::renderMessage("Hello from rust!".to_string());

    let s = settings::load().expect("Invalid settings");
    let d = datastore::load().expect("Datastore failied to init");

    println!("Starting telegram listener...");
    telegram::listen(s.api_key, &d);

    println!("Starting display refresh...");
    let y = thread::spawn(move|| {
      while(true) {
        thread::sleep(time::Duration::from_secs(60 * 10));
        let db = d.lock().unwrap();

        match db.get(b"current_status") {
                Ok(Some(value)) => display::renderMessage("Hello from rust!".to_string()),
                Ok(None)        => (),
                Err(e)          => (),
            }
      }
    });

    y.join();
}
