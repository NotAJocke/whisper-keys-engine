use rdev::{listen, EventType, Key};
use rodio::{self, Decoder, OutputStream, Sink};
use std::{
    collections::HashMap,
    ffi::OsString,
    fmt::Display,
    fs::{self, File},
    io::BufReader,
    path::Path,
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
            dbg!(&key_lowercase);

            let filename = match config.get(&key_lowercase) {
                Some(f) => f.to_string(),
                None => config.get("unknown").unwrap().to_string(),
            };
            dbg!(&filename);

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

fn list_available_packs(folder: &str) -> anyhow::Result<Vec<OsString>> {
    let files = fs::read_dir(folder)?;

    let subdirs = files
        .filter_map(|d| {
            let entry = d.ok()?;
            let path = entry.path();
            if path.is_dir() {
                Some(entry.file_name())
            } else {
                None
            }
        })
        .collect::<Vec<OsString>>();

    let mut packs: Vec<OsString> = Vec::new();
    for dir in subdirs.iter() {
        let path = Path::new(folder).join(dir);
        let files = fs::read_dir(path)?;
        let filesnames = files
            .filter_map(|f| {
                let entry = f.ok()?;
                let path = entry.path();
                if path.is_file() {
                    Some(entry.file_name())
                } else {
                    None
                }
            })
            .collect::<Vec<OsString>>();
        let has_config_file = filesnames.contains(&OsString::from("config.json"));

        if has_config_file {
            packs.push(dir.to_owned());
        }
    }

    Ok(packs)
}

pub struct KeyWrapper(pub Key);

impl Display for KeyWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_packs() {
        let packs = list_available_packs("assets");

        match packs {
            Err(e) => panic!("{}", e),
            Ok(p) => {
                assert_eq!(p.len(), 2);
            }
        }
    }
}
