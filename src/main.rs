mod settings;
mod telegram;
mod datastore;

fn main() {
    let s = settings::load().expect("Invalid settings");
    let d = datastore::load().expect("Datastore failied to init");

    println!("Starting telegram listener...");
    telegram::listen(s.api_key, d);
}
