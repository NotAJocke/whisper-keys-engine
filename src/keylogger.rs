use anyhow::Result;
use std::{fmt::Display, sync::mpsc::Sender, thread};

use rdev::{self, EventType, Key};

pub struct KeyWrapper(pub Key);

impl Display for KeyWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

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

                let key_lowercase = format!("{}", KeyWrapper(key)).to_lowercase();

                tx.send(key_lowercase).unwrap();

                last_key = Some(key);
            }

            last_event_type = Some(event.event_type);
        })
        .map_err(|e| anyhow::Error::msg(format!("{:#?}", e)))
        .unwrap();
    });

    Ok(())
}
