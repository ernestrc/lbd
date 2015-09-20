pub mod config;

use super::dal::Dal;
use self::config::Config;

pub struct System {
    dal: Dal,
    config: Config
}

impl System {
    pub fn new(dal: Dal, config: Config) -> System {
        System {
            config: config,
            dal: dal
        }
    }
}
