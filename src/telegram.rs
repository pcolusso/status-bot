use telebot::Bot;
use futures::stream::Stream;
use std::sync::{Arc, Mutex};
use futures::Future;
use rocksdb::DB;

use telebot::functions::*;

pub fn listen(api_key: String, db: &Arc<Mutex<DB>>) {
    let mut bot = Bot::new(&api_key).update_interval(200);

    // Register a reply command which answers a message
    let handleDB = Arc::clone(&db);
    let handle = bot.new_cmd("/read")
        .and_then(move |(bot, msg)| {
            let db = handleDB.lock().unwrap();

            match db.get(b"current_status") {
                Ok(Some(value)) => bot.message(msg.chat.id, value.to_utf8().unwrap().to_string()).send(),
                Ok(None)        => bot.message(msg.chat.id, "No status defined.".to_string()).send(),
                Err(e)          => bot.message(msg.chat.id, "Unable to read DB".to_string()).send(),
            }
        })
        .for_each(|_| Ok(()));

    let handle2DB = Arc::clone(&db);
    let handle2 = bot.new_cmd("/write")
        .and_then(move |(bot, mut msg)| {
            let db = handle2DB.lock().unwrap();

            if let Some(status) = msg.text.take() {
                db.put(b"current_status", status);
                match db.get(b"current_status") {
                    Ok(Some(value)) => bot.message(msg.chat.id, value.to_utf8().unwrap().to_string()).send(),
                    Ok(None)        => bot.message(msg.chat.id, "No status defined.".to_string()).send(),
                    Err(e)          => bot.message(msg.chat.id, "Failed to re-read status.".to_string()).send(),
                }
            } else {
                bot.message(msg.chat.id, "idk".to_string()).send()
            }
        })
        .for_each(|_| Ok(()));

    bot.run_with(handle.join(handle2));
}