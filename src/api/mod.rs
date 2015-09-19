use iron::middleware::Chain;
use iron::{ Response, Request };
use iron::status;
use super::system::System;

pub struct Api {
    pub chain: Chain
}

impl Api {
    pub fn new(system: System) -> Api {
        Api {
            chain: Chain::new(router!(
                get "/" => { |_: &mut Request| Ok(Response::with(status::Ok)) },
                get "/:query" => { |_: &mut Request| Ok(Response::with(status::Ok)) }))
        }
    }
}
