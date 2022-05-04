#![allow(unused)]

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    each_line(args.path, |line: &str| {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    });
}

fn each_line<F>(path: std::path::PathBuf, callback: F) -> Result<(), std::io::Error>
where
    F: Fn(&str),
{
    let file = File::open(&path).expect("can't open file");
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    loop {
        match reader.read_line(&mut line) {
            Ok(0) => {
                // EOF
                break;
            }
            Ok(bytes_read) => {
                callback(&line);
                // do not accumulate data
                line.clear();
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    Ok(())
}
