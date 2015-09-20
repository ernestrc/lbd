use rustc_serialize::json;
use std::error::Error;
use std::collections::HashMap;
use std::fs;
use std::fs::{File, OpenOptions};
use std::path::Path;
use super::super::utils::*;
use std::io;
use std::io::prelude::*;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Config {
    etcd_url: Option<String>,
    slack_channel: Option<String>,
    admins: Vec<String>,
    bin: HashMap<String, String>,
}

static CONFIG_DIR: &'static str = "/etc/lbd";
static CONFIG_FILE: &'static str = "/etc/lbd/config.json";

fn touch_file() -> io::Result<Config> {
    match OpenOptions::new().create(true).write(true).open(CONFIG_FILE) {
        Ok(mut f) => {
            let empty = Config::empty();
            let encoded = get!(json::encode(&empty), "There was an error when encoding JSON to file");
            get!(f.write_all(encoded.as_bytes()), "There was an error when writing JSON to config file");
            Ok(empty)
        },
        Err(e) => Err(e)
    }
}

impl Config{

    fn empty() -> Config {
        Config {
            etcd_url: None,
            slack_channel: None,
            admins: vec![],
            bin: HashMap::new()
        }
    }

    pub fn load() -> Config {
        //first check if config folder exists
        match fs::metadata(CONFIG_DIR) {
            Ok(ref attrs) if attrs.is_dir() =>
                match fs::metadata(CONFIG_FILE) {
                    Ok(ref cfg_attr) if cfg_attr.is_file() => {
                        let mut file =
                            get!(File::open(&Path::new(CONFIG_FILE)), "Could not open config file");
                        let mut contents = String::new();
                        get!(file.read_to_string(&mut contents), "Could not read config file");
                        get!(json::decode(&contents.to_string()), "Could not decode config file to JSON")
                    },
                    _ => touch_file().unwrap()
                },
            _ => {
                get!(fs::create_dir(CONFIG_DIR), &format!("Could not create config dir in {}", CONFIG_DIR));
                get!(touch_file(), &format!("Could not create config file in {}", CONFIG_FILE))
            }
        }
    }
}
