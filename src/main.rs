mod settings;
mod telegram;
mod datastore;

fn main() {
    let s = settings::load().expect("Invalid settings");
    let d = datastore::test().expect("Failed to init datastore");

    println!("api key: {}", s.api_key);
}
