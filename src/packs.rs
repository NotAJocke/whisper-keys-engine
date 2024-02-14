use anyhow::{Context, Result};
use rayon::prelude::*;
use rodio::{source::Buffered, Decoder, Source};
use rustc_hash::FxHashMap;
use std::{
    ffi::OsString,
    fs::{self, File},
    io::BufReader,
    path::{Path, PathBuf},
};

pub fn list_available(path: &PathBuf) -> Result<Vec<String>> {
    /*let local_dir = home_dir().context("Couldn't get your local dir.")?;
    let path = match OS {
        "windows" => Path::new(&local_dir)
            .join("AppData")
            .join("Roaming")
            .join(APP_NAME),
        _ => Path::new(&local_dir).join(APP_NAME),
    };*/
    let items = fs::read_dir(&path).context("Local dir do not exist or is unreadable.")?;
    let subdirs: Vec<OsString> = items
        .filter_map(|d| {
            let entry = d.ok()?;
            let path = entry.path();
            if path.is_dir() {
                Some(entry.file_name())
            } else {
                None
            }
        })
        .collect();

    let mut packs: Vec<String> = Vec::new();
    for dir in subdirs.iter() {
        let path = Path::new(&path).join(dir);
        let files = fs::read_dir(path).unwrap();
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
            packs.push(dir.to_str().unwrap().to_owned())
        }
    }

    let subdirs_str: Vec<String> = subdirs
        .iter()
        .map(|d| d.to_str().unwrap().to_owned())
        .collect();

    Ok(subdirs_str)
}

#[derive(serde::Deserialize, Debug, serde::Serialize)]
pub struct Config {
    pub creator: String,
    pub source: String,
    pub keys_default_volume: String,
    pub keys: FxHashMap<String, String>,
}

pub struct Pack {
    pub name: String,
    pub keys_default_volume: f32,
    pub keys: FxHashMap<String, Buffered<Decoder<BufReader<File>>>>,
}

pub fn load_pack(folder: &PathBuf, pack_name: &str) -> Result<Pack> {
    let path = Path::new(&folder).join(pack_name);
    let config = match fs::read_to_string(path.join("config.json5")) {
        Ok(config) => config,
        Err(_) => fs::read_to_string(path.join("config.json"))?,
    };
    let parsed_config: Config = json5::from_str(&config).context("Original config isn't valid")?;

    let pack_keys = parsed_config
        .keys
        .par_iter()
        .map(|(key, value)| {
            let filepath = path.join(value);
            let file =
                File::open(&filepath).context(format!("Couldn't load file: {:?}", filepath))?;
            let buf = BufReader::new(file);
            let source = Decoder::new(buf).context("Couldn't decode file")?;
            let buffered = Decoder::buffered(source);
            Ok((key.to_owned(), buffered))
        })
        .collect::<Result<FxHashMap<String, Buffered<Decoder<BufReader<File>>>>>>()?;

    let pack = Pack {
        name: pack_name.to_owned(),
        keys_default_volume: parsed_config
            .keys_default_volume
            .parse::<f32>()
            .context("Couldn't parse default volume")?,
        keys: pack_keys,
    };

    Ok(pack)
}
