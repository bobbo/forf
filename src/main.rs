extern crate getopts;

use getopts::Options;
use std::env;

fn invalid_args(opts: &Options, args: &Vec<String>) {
    let usage = opts.short_usage(&args[0]);
    panic!(usage);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let opts = Options::new();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.free.len() == 0 {
        invalid_args(&opts, &args);
    }

    let glob_input = &matches.free[0];
    let command = matches.free[1..].join(" ");

    if glob_input.is_empty() || command.is_empty() {
        invalid_args(&opts, &args);
    }

}
