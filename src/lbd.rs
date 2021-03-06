#![feature(plugin)]
#![plugin(docopt_macros)]
extern crate docopt;
extern crate rustc_serialize;
#[macro_use] extern crate lbd;
#[macro_use] extern crate log;

use docopt::Docopt;
use lbd::prelude::*;

docopt!(Args derive Debug, "
.___                           __                                 __
/\\_ \\                         /\\ \\                               /\\ \\
\\//\\ \\     ___     ___      __\\ \\ \\____    ___      __     _ __  \\_\\ \\
  \\ \\ \\   / __`\\ /' _ `\\  /'_ `\\ \\ '__`\\  / __`\\  /'__`\\  /\\`'__\\/'_` \\
   \\_\\ \\_/\\ \\L\\ \\/\\ \\/\\ \\/\\ \\L\\ \\ \\ \\L\\ \\/\\ \\L\\ \\/\\ \\L\\.\\_\\ \\ \\//\\ \\L\\ \\
   /\\____\\ \\____/\\ \\_\\ \\_\\ \\____ \\ \\_,__/\\ \\____/\\ \\__/.\\_\\\\ \\_\\\\ \\___,_\\
   \\/____/\\/___/  \\/_/\\/_/\\/___L\\ \\/___/  \\/___/  \\/__/\\/_/ \\/_/ \\/__,_ /
                            /\\____/
                            \\_/__/

 Usage:
  lbd new (schedule|job)
  lbd submit <path> [ --all ]
  lbd run <job_id>
  lbd inspect <job_id>
  lbd rm (<job_id>| <schedule_id>)
  lbd kill <job_id> [ --all ]
  lbd tree <schedule_id>
  lbd ps
  lbd (-h | --help)
  lbd --version

Options:
  -h --help     Show this message.
  --version     Show version.
  --verbose     Turn on debugging messages
");

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
}
