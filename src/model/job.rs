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
    deps: Option<Vec<self::Id>>,
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
        let err_msg = "Could not encode job to toml format";
        get!(self.encode(&mut encoder),err_msg);
        ::toml::Value::Table(encoder.toml)
    }

    pub fn from_toml(table: ::toml::Value) -> Job {
        get!(Decodable::decode(&mut Decoder::new(table)),
        "Could not decode toml table into Job")
    }

    pub fn new(id: self::Id, host: system::Id, period: period::Id,
               cmd: String, desc: String, spec: Option<String>,
               env: Option<HashMap<String, String>>, deps: Option<Vec<self::Id>>,
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
        let deps = vec!["job_a".to_string(), "job_b".to_string(), "job_c".to_string()];
        let job = Job::new("id".to_string(), "localhost".to_string(), 
                           "daily".to_string(), "ls -l".to_string(), 
                           "list files in cwd".to_string(),
                           None, None, Some(deps), None, None);
        let tm = job.to_toml();
        println!("{:?}", tm);
        let cmd = tm.lookup("cmd").unwrap();
        let host = tm.lookup("host").unwrap();
        let period = tm.lookup("period").unwrap();
        let desc = tm.lookup("desc").unwrap();
        assert_eq!(cmd.as_str().unwrap(), "ls -l");
        assert_eq!(host.as_str().unwrap(), "localhost");
        assert_eq!(period.as_str().unwrap(), "daily");
        assert_eq!(desc.as_str().unwrap(), "list files in cwd");
        
        println!("{:?}", ::toml::encode_str(&tm));
        assert!(tm == r#"
            cmd = "ls -l"
            desc = "list files in cwd"
            host = "localhost"
            id = "id"
            period = "daily"
            deps = [ "job_a", "job_b", "job_c" ]
        "#.parse().unwrap());
    }

    #[test]
    fn deserialize_toml() {
        let tm = r#"
            id = "id"
            host = "localhost"
            period = "daily"
            cmd = "ls -l"
            desc = "list files in cwd"
            spec_path = "/etc/longboard/"

            [env]
            DATECUT = "2015-10-04"
        "#;

        let value: ::toml::Value = tm.parse().unwrap();

        let job = Job::from_toml(value);
        assert_eq!(job.id, "id");
        assert_eq!(job.period, "daily");
        assert_eq!(job.cmd, "ls -l");
        assert!(job.deps.is_none());
        assert!(job.args.is_none());
        assert_eq!(job.host, "localhost");
        assert_eq!(job.spec_path.unwrap(), "/etc/longboard/");
        assert_eq!(job.env.unwrap().get("DATECUT").unwrap(), "2015-10-04");
    }
}
