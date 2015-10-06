#[macro_use] extern crate lbd;
#[macro_use] extern crate log;

use lbd::prelude::*;

static FORMAT: &'static str =
"Api: Iron: @[blue]{method} @{uri} - {status} in  @[brightgreen]{response-time}@";

fn main() {

    let config: &Config = &Config::load();

    info!("Loaded config successfully. Booting up now..");

    let dal = Dal::new();
    let system = System::new(dal, config);
    let mut api = Api::new(system);

    //http logging
    let format = Format::new(FORMAT, vec![], vec![]);
    let (logger_before, logger_after) = Logger::new(Some(format.unwrap()));
    api.chain.link_before(logger_before);
    api.chain.link_after(logger_after);

    //start http server
    Iron::new(api.chain).http("localhost:3000").unwrap();
}
