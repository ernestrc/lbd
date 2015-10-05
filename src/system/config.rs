use rustc_serialize::json;
use std::error::Error;
use std::collections::HashMap;
use std::fs;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io;
use std::io::prelude::*;
use std::default::Default;
use std::str::FromStr;
use log4rs;
use env_logger;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Config {
    pub pub_host: String,
    etcd_url: Option<String>,
    slack_channel: Option<String>,
    admins: Vec<String>,
    bin: HashMap<String, String>,
}

static CONFIG_DIR: &'static str = "/etc/lbd";
static CONFIG_FILE: &'static str = "/etc/lbd/config.json";
static CONFIG_LOG: &'static str = "/etc/lbd/log.toml";
static DEFAULT_HOST: &'static str = "localhost:2846";

fn touch_cfg() -> io::Result<Config> {
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

    pub fn load() -> Config {
        //check if config folder exists
        match fs::metadata(CONFIG_DIR) {
            Ok(ref attrs) if attrs.is_dir() => {
                //check if log configuration file exists
                match fs::metadata(CONFIG_LOG) {
                    Ok(ref log_cfg_attr) if log_cfg_attr.is_file() =>
                        log4rs::init_file(CONFIG_LOG, Default::default()).unwrap(),
                    _ => {
                        println!("Could not find LOG cfg in {}. Using env logger instead (remember to run setting env var RUST_LOG=level)", CONFIG_LOG);
                        env_logger::init().unwrap();
                    }
                };
                match fs::metadata(CONFIG_FILE) {
                    Ok(ref cfg_attr) if cfg_attr.is_file() => {
                        let mut file =
                            get!(File::open(&Path::new(CONFIG_FILE)), "Could not open config file");
                        let mut contents = String::new();
                        get!(file.read_to_string(&mut contents), "Could not read config file");
                        get!(json::decode(&contents.to_string()), "Could not decode config file to JSON")
                    },
                    _ => touch_cfg().unwrap()
                }
            }
            _ => {
                get!(fs::create_dir(CONFIG_DIR), &format!("Could not create config dir in {}", CONFIG_DIR));
                get!(touch_cfg(), &format!("Could not create config file in {}", CONFIG_FILE))
            }
        }
    }

    fn empty() -> Config {
        Config {
            pub_host: String::from_str(DEFAULT_HOST).unwrap(),
            etcd_url: None,
            slack_channel: None,
            admins: vec![],
            bin: HashMap::new()
        }
    }
}
