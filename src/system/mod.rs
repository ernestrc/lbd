pub mod config;

use super::dal::Dal;
use self::config::Config;

pub type Id = String;

pub struct System <'a>{
    host: &'a self::Id,
    dal: Dal,
    config: &'a Config
}

impl <'a>System <'a> {
    pub fn new(dal: Dal, config: &'a Config) -> System {
        let host: &'a self::Id = &config.pub_host;
        System {
            host: host,
            config: config,
            dal: dal
        }
    }
}
