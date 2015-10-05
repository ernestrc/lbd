use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;

static VAR_DIR: &'static str = "/var/lbd";
static LOCK_FILE: &'static str = "/var/lbd/lock";

pub struct Dal;

fn lock_file() {
    get!(OpenOptions::new().create(true).write(true).open(LOCK_FILE),
        "There was an error when trying to create lock file");
}

impl Dal {
    pub fn new() -> Dal {
        match fs::metadata(VAR_DIR) {
            Ok(ref attrs) if attrs.is_dir() => Dal::init(),
            _ => {
                get!(fs::create_dir(VAR_DIR), &format!("Couldn not create var dir in {}", VAR_DIR));
                Dal::init()
            }
        }
    }

    fn init() -> Dal {
        lock_file();
        Dal
    }
}
