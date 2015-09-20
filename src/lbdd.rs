extern crate iron;
extern crate logger;
extern crate term;
#[macro_use]
extern crate router;
extern crate rustc_serialize;

#[macro_use]
mod utils;
mod api;
mod dal;
mod system;

use iron::prelude::Iron;
use logger::Logger;
use logger::format::Format;
use api::Api;
use system::System;
use dal::Dal;

static FORMAT: &'static str =
"Api: Iron: @[blue]{method} @{uri} - {status} in  @[brightgreen]{response-time}@";

fn main() {
    let dal = Dal::new();
    let system = System::new(dal);
    let mut api = Api::new(system);

    //logging
    let format = Format::new(FORMAT, vec![], vec![]);
    let (logger_before, logger_after) = Logger::new(Some(format.unwrap()));
    api.chain.link_before(logger_before);
    api.chain.link_after(logger_after);

    Iron::new(api.chain).http("localhost:3000").unwrap();
}
