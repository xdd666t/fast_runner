mod args;

use clap::Parser;
use crate::args::arg::RunnerArgs;


fn main() {
    let args = RunnerArgs::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}