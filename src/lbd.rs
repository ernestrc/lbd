#![feature(plugin)]
#![plugin(docopt_macros)]
extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

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
  lbd ps
  lbd (-h | --help)
  lbd --version

Options:
  -h --help     Show this message.
  --version     Show version.
  --speed=<kn>  Speed in knots [default: 10].
  --moored      Moored (anchored) mine.
  --drifting    Drifting mine.
");

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
}
