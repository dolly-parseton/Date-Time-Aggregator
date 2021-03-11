#[allow(unused_imports)]
#[macro_use]
extern crate log;
extern crate date_time_aggregator;
extern crate simplelog;

// Conditional Imports
#[allow(unused_imports)]
use date_time_aggregator::input::file::FileSource;
use date_time_aggregator::input::json::JsonParser;
use date_time_aggregator::input::stdin::StdinSource;
// use date_time_aggregator::input::stdin::StdinSource;

// Imports
use date_time_aggregator::{
    aggregators::{maximum::MaximumAggregator, Aggregator},
    input::{simple::SimpleParser, Parser, Source},
};
use log::LevelFilter;
use simplelog::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "dta", about = "A date time aggreator.")]
struct Opt {
    /// Activate debug mode.
    #[structopt(short, long)]
    debug: bool,

    // Might change this one to just an option in subcommand.
    /// Input file(s) glob option. When selected data is read in from the files identified by the glob and Stdin is ignored.
    #[structopt(long, short = "R")]
    glob: Option<String>,

    /// Provide input options, CSV / JSON are the options, default is none and will just work on timestamps provided.
    #[structopt(short, long = "input-format", parse(try_from_str = parse_input_format))]
    input: Option<Inputs>,

    /// Provide a TZ to be used. If the timefield includes a timestamp this field will be disregarded.
    #[structopt(long = "tz")]
    timezone: Option<String>,

    /// Provide a datetime format used to parse timestamps, if not specified dta will try to parse the format.
    #[structopt(short = "f", long = "datetime-format")]
    date_format: Option<String>,
}

/// Function provides parsing functionality to read Input options from argument.
fn parse_input_format(s: &str) -> Result<Inputs, Box<dyn std::error::Error>> {
    match s.to_ascii_lowercase().as_ref() {
        "csv" => Ok(Inputs::CSV),
        "json" => Ok(Inputs::JSON),
        _ => Err("Could not parse input-format string.".into()),
    }
}

#[derive(Debug, PartialEq, StructOpt)]
enum Inputs {
    CSV,
    JSON,
}

fn main() {
    // Read in arguments
    let opt = Opt::from_args();
    // Enable logger if debug is enabled
    if opt.debug {
        CombinedLogger::init(vec![TermLogger::new(
            LevelFilter::max(),
            Config::default(),
            TerminalMode::Mixed,
        )])
        .unwrap();
        debug!("Command line options provided: {:#?}", opt);
    }
    // Match based on the command line options to decide what todo.

    let stdin = StdinSource::default();
    //
    let simple_parser = SimpleParser::default();
    let json_parser = JsonParser::default();
    let mut agg = MaximumAggregator::default();
    //
    while let Ok(r) = stdin.read_data() {
        if r.is_empty() {
            break;
        }
        match opt.input {
            None => match simple_parser.parse_data(
                r,
                "",
                opt.date_format.as_ref(),
                opt.timezone.as_ref(),
            ) {
                Ok(d) => {
                    if let Err(e) = agg.update(&d) {
                        error!("Error occured in parsing: {:?}", e)
                    }
                }
                Err(e) => error!("Error occured in parsing: {:?}", e),
            },
            Some(ref input_format) => match input_format {
                Inputs::JSON => match json_parser.parse_data(
                    r,
                    "timestamp",
                    opt.date_format.as_ref(),
                    opt.timezone.as_ref(),
                ) {
                    Ok(d) => {
                        if let Err(e) = agg.update(&d) {
                            error!("Error occured in parsing: {:?}", e)
                        }
                    }
                    Err(e) => error!("Error occured in parsing: {:?}", e),
                },
                _ => (),
            },
        }
    }
    //
    debug!("{:?}", agg.output());
    //
}
