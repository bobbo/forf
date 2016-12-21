extern crate getopts;
extern crate glob;

use getopts::Options;
use std::env;
use glob::glob;

struct Args {
    glob: String,
    command: String
}

fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();
    let opts = Options::new();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.free.len() == 0 {
        invalid_args(&opts, &args);
    }

    let glob_input = matches.free[0].clone();
    let command = matches.free[1..].join(" ");

    if glob_input.is_empty() || command.is_empty() {
        invalid_args(&opts, &args);
    }

    Args{ glob: glob_input, command: command }
}

fn invalid_args(opts: &Options, args: &Vec<String>) {
    let usage = opts.short_usage(&args[0]);
    panic!(usage);
}

fn expansion_value<'a>(expr: &str, path: &'a str) -> &'a str {
    match expr {
        ":f" => path,
        _ => ""
    }
}

fn build_command(path: &str, args: &Args) -> String {
    let mut command = args.command.clone();

    for expr in vec!(":f") {
        if command.contains(expr) {
            command = command.replace(expr, expansion_value(expr, path));
        }
    }

    command
}

fn main() {
    let args = parse_args();

    for entry in glob(&args.glob).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let command = build_command(path.to_str().unwrap(), &args);
                println!("{}", command);
            },
            Err(e) => panic!(e)
        }
    }
}
