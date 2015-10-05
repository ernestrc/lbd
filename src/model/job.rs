//! Holds all data about a runnable process as well
//! as information on how to execute it and when.
//!
//! It creates the job's logging folder in /var/log/lbd if necessary
//! and if stdout parameter is set.
//!
//! It exposes run() method. This method executes the job as
//! a subprocess and redirects its output to the log file.

use std::collections::HashMap;
use std::io;
use super::receipt::Receipt;
use super::period;
use super::super::system;
use std::fmt;
use std::path::Path;
use toml::*;
use rustc_serialize::{ Decodable, Encodable };
use super::super::utils;

pub type Id = String;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Job {
    id: self::Id,
    host: system::Id,
    period: period::Id,
    cmd: String,
    desc: String,
    spec_path: Option<String>,
    env: Option<HashMap<String, String>>,
    deps: Option<Vec<Job>>,
    args: Option<Vec<String>>,
    silent: Option<bool>
}

//TODO
///sets up args paths and logging file
//fn prepare_env(for_job: &self::Id, argso: &Option<Vec<String>>) -> io::Result<()> {
//    argso.map( |args: Vec<String>| {
//        args.map( |arg: String| {
//            unimplemented!()
//        })
//    }).or_else(|| info!("No args found for job with id {}", for_job))
//}


impl Job {

    pub fn to_toml(&self) -> ::toml::Value {
        let mut encoder = Encoder::new();
        let errMsg = "Could not encode job to toml format";
        get!(self.encode(&mut encoder),errMsg);
        ::toml::Value::Table(encoder.toml)
    }

    pub fn from_toml(table: ::toml::Value) -> Job {
        get!(Decodable::decode(&mut Decoder::new(table)),
             "Could not decode toml table into Job")
    }

    pub fn new(id: self::Id, host: system::Id, period: period::Id,
               cmd: String, desc: String, spec: Option<String>,
               env: Option<HashMap<String, String>>, deps: Option<Vec<Job>>,
               args: Option<Vec<String>>, silent: Option<bool>) -> Job {
        //TODO prepare env and log files
        Job {
            id: id,
            host: host,
            period: period,
            cmd: cmd,
            desc: desc,
            spec_path: spec,
            env: env,
            deps: deps,
            args: args,
            silent: silent
        }
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serialize_toml() {
        let job = Job::new("1".to_string(), "1".to_string(), "1".to_string(),
                           "ls -l".to_string(), "list files in cwd".to_string(),
                           None, None, None, None, None);
        let tm = job.to_toml();
        println!("{:?}", tm);
        assert!(tm.get("cmd")  == ::toml::Value::String("ls -l".to_string()));
    }

    #[test]
    fn deserialize_toml() {
        let tm = "";
        //TODO TODO TODO
        assert!(1 == 1);
    }
}
