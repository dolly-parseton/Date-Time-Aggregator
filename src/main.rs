#[macro_use]
extern crate serde;
extern crate serde_json;

use crate::round::Round;
use std::path::PathBuf;
use structopt::StructOpt;

mod round;
mod timespan;

#[derive(Debug, StructOpt)]
#[structopt(name = "dta-cli", about = "...")]
struct Opt {
    /// Input file(s)
    #[structopt(parse(from_os_str))]
    input: Vec<PathBuf>,

    #[structopt(short = "t", long = "target")]
    target_field: String,

    // Subcommands
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    Round(Round),
}

fn validate_input(opt: &Opt) -> Result<bool, &'static str> {
    for input_file in &opt.input {
        if !input_file.is_file() {
            return Err("Input path/file/glob does not exist");
        }
    }
    Ok(true)
}

fn main() {
    let opt = Opt::from_args();
    if let Err(err) = validate_input(&opt) {
        eprintln!("{}", err);
        std::process::exit(1);
    } else {
        println!("{:#?}", opt.input);
    }
    //
}
