use rocksdb::{DB, Options};
use std::error::Error;

pub fn test() -> Result<(), Box<Error>> {
    let db = DB::open_default("data")?
}