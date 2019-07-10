use std::thread;

mod settings;
mod telegram;
mod datastore;
mod display;

fn main() {
    let s = settings::load().expect("Invalid settings");
    let d = datastore::load().expect("Datastore failied to init");

    println!("Starting display refresh...");
    let y = thread::spawn(|| {
      display::run();
    });

    println!("Starting telegram listener...");
    telegram::listen(s.api_key, d);
    y.join();
}
