use std::collections::HashMap;
use std::io;
use super::receipt::Receipt; 
use super::period; 
use super::super::system; 
use std::fmt;
use std::path::Path;
use toml::Decoder;
use rustc_serialize::Decodable;
use super::super::utils;

pub type Id = String;

///Holds all data about a runnable process as well
///as information on how to execute it and when.
///
///It creates the job's logging folder in /var/log/lbd if necessary
///and if stdout parameter is set.
///
///It exposes run() method. This method executes the job as
///a subprocess and redirects its output to the log file.
#[derive(RustcDecodable, RustcEncodable)]
struct Job {
    id: self::Id,
    host: system::Id,
    period: period::Id,
    cmd: String,
    desc: String, 
    spec_path: String,
    env: Option<HashMap<String, String>>,
    deps: Option<Vec<Job>>,
    args: Option<Vec<String>>,
    silent: Option<bool>
}

///sets up environment variables and logging file
fn prepare_env(for_job: &self::Id, argso: &Option<Vec<String>>) -> io::Result<()> {
    argso.map( |args: Vec<String>| {
        args.map( |arg: String| {
            //TODO  continue here
        })
    }).or_else(|| info!("No args found for job with id {}", for_job))
}


impl Job {

    pub fn from_toml(table: ::toml::Value) -> Job {
        get!(Decodable::decode(&mut Decoder::new(table)), 
             "Could not decode toml table into Job")
    }

    pub fn new(id: self::Id, host: system::Id, period: period::Id, 
               cmd: String, args: Option<Vec<String>>, desc: String, 
               env: HashMap<String, String>, 
               deps: Vec<Job>, spec: Option<String>) -> Job {
        unimplemented!()
    }

    fn run(&self, silent: bool/*, TODO noti*/) -> Receipt {
        unimplemented!()
    }
}

impl fmt::Display for Job {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "job[{}]", &self.id)
    }
}
