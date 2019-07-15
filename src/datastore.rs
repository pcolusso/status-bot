use rocksdb::DB;
use std::sync::{Arc, Mutex};
use std::error::Error;

pub fn load() -> Result<Arc<Mutex<DB>>, Box<Error>> {
    let db = DB::open_default("data")?;
    let locker = Arc::new(Mutex::new(db));

    Ok(locker)
}
