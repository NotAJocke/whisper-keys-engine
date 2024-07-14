use anyhow::Result;
use rdev::{self, EventType, Key};
use std::{sync::mpsc::Sender, thread};

use crate::key_wrapper::KeyWrapper;

pub fn listen(tx: Sender<String>) -> Result<()> {
    thread::spawn(move || {
        let mut last_key: Option<Key> = None;
        let mut last_event_type: Option<EventType> = None;

        rdev::listen(move |event| {
            if let EventType::KeyPress(key) = event.event_type {
                if let (Some(last_k), Some(last_e)) = (last_key, last_event_type) {
                    if last_k == key {
                        if let EventType::KeyPress(_) = last_e {
                            return;
                        }
                    }
                }

                let key_lowercase = KeyWrapper(key).to_lowercase();

                tx.send(key_lowercase).unwrap();

                last_key = Some(key);
            }

            last_event_type = Some(event.event_type);
        })
        .map_err(|e| format!("Couldn't start the keylogger.\n\nErr: {e:?}"))
        .unwrap();
    });

    Ok(())
}
