mod config;

use super::dal::Dal;
use self::config::Config;

pub struct System {
    dal: Dal,
    config: Config
}

impl System {
    pub fn new(dal: Dal) -> System {
        System {
            config: Config::load(),
            dal: dal
        }
    }
}
