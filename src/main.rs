use anyhow::Ok;
use rdev::{listen, EventType, Key};
use rodio::{self, Decoder, OutputStream, Sink};
use std::{
    collections::HashMap,
    fmt::Display,
    fs::{self, File},
    io::BufReader,
};

fn main() -> anyhow::Result<()> {
    let config = load_config_file()?;

    let (_stream, stream_handle) = OutputStream::try_default()?;

    let mut last_key: Option<Key> = None;
    let mut last_event_type: Option<EventType> = None;

    listen(move |event| {
        if let EventType::KeyPress(key) = event.event_type {
            println!("KeyPress: {:?}", key);

            if let (Some(lastk), Some(laste)) = (last_key, last_event_type) {
                if lastk == key {
                    if let EventType::KeyPress(_) = laste {
                        return;
                    }
                }
            }

            let key_lowercase = format!("{}", KeyWrapper(key)).to_lowercase();

            let filename = config.get(&key_lowercase).unwrap().to_string();
            let file = File::open(format!("assets/Cream/{}", filename)).unwrap();
            let data = BufReader::new(file);
            let source = Decoder::new(data).unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();

            sink.set_volume(1.0);
            sink.append(source);
            sink.detach();

            last_key = Some(key);
        }

        last_event_type = Some(event.event_type);
    })
    .map_err(|e| anyhow::Error::msg(format!("{:#?}", e)))?;

    Ok(())
}

fn load_config_file() -> anyhow::Result<HashMap<String, String>> {
    let file_content = fs::read_to_string("./assets/Cream/config.json")?;
    let parsed_content: HashMap<String, String> = serde_json::from_str(&file_content)?;

    Ok(parsed_content)
}

pub struct KeyWrapper(pub Key);

impl Display for KeyWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
