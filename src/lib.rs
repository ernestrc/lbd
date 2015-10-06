#![feature(plugin)]
#[macro_use] extern crate router;
#[macro_use] extern crate log;
extern crate iron;
extern crate logger;
extern crate term;
extern crate rustc_serialize;
extern crate toml;
extern crate env_logger;
extern crate log4rs;

#[macro_use] pub mod utils;
pub mod api;
pub mod dal;
pub mod system;
pub mod model;

pub mod prelude {
    pub use iron::prelude::Iron;
    pub use logger::Logger;
    pub use logger::format::Format;
    pub use api::Api;
    pub use system::System;
    pub use dal::Dal;
    pub use system::config::Config;
}

