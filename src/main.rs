#[allow(unused_imports)]
#[macro_use]
extern crate log;
extern crate date_time_aggregator;
extern crate simplelog;

// Conditional Imports
use date_time_aggregator::input::csv::CsvParser;
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
    #[structopt(long = "directory", short = "R")]
    glob: Option<String>,

    /// Parse CSV data, supply a valid position for the timestamp field (starting at 0).
    #[structopt(short, long)]
    csv: Option<u8>,

    /// Parse JSON data, field has to be the field name.
    #[structopt(short, long)]
    json: Option<String>,

    /// Select an aggregator.
    #[structopt(subcommand)]
    aggregator: Aggregators,

    /// Provide a TZ to be used. If the timefield includes a timestamp this field will be disregarded.
    #[structopt(long = "tz")]
    timezone: Option<String>,

    /// Provide a datetime format used to parse timestamps, if not specified dta will try to parse the format.
    #[structopt(short = "f", long = "datetime-format")]
    date_format: Option<String>,

    /// Convert timestamp being used by the aggregation into the provided format, if none is provided the format will not be changed.
    #[structopt(long = "convert")]
    convert_timestamp: Option<String>,
}

// /// Function provides parsing functionality to read Input options from argument.
// fn parse_input_format(s: &str) -> Result<Inputs, Box<dyn std::error::Error>> {
//     match s.to_ascii_lowercase().as_ref() {
//         "csv" => Ok(Inputs::CSV { level: 0 }),
//         "json" => Ok(Inputs::JSON),
//         _ => Err("Could not parse input-format string.".into()),
//     }
// }

// #[derive(Debug, PartialEq, StructOpt)]
// enum Inputs {
//     CSV {
//         #[structopt(short, long)]
//         level: u8,
//     },
//     JSON {
//         #[structopt(short, long)]
//         field: String,
//     },
// }

#[derive(Debug, PartialEq, StructOpt)]
enum Aggregators {
    Maximum,
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
    let mut source: Box<dyn Source> = match opt.glob {
        Some(g) => match FileSource::new(&g, true) {
            Ok(s) => (Box::new(s) as Box<dyn Source>),
            Err(e) => {
                error!("Error whilst creating source: {}", e.reason);
                std::process::exit(1);
            }
        },
        None => (Box::new(StdinSource::default()) as Box<dyn Source>),
    };

    let parser: Box<dyn Parser> = match (opt.csv, opt.json) {
        (Some(_c), Some(_j)) => {
            // Error
            error!(
                "Error whilst creating parser: {}",
                "You can select either CSV or JSON"
            );
            std::process::exit(1);
        }
        (Some(c), None) => (Box::new(CsvParser::new(c)) as Box<dyn Parser>),
        (None, Some(j)) => (Box::new(JsonParser::new(j)) as Box<dyn Parser>),
        (None, None) => (Box::new(SimpleParser::default()) as Box<dyn Parser>),
    };

    // match i {
    //     Inputs::JSON => (Box::new(JsonParser::new(field)) as Box<dyn Parser>),
    //     Inputs::CSV => {
    //         error!(
    //             "Error whilst creating parser: {}",
    //             "CSV Parser is not currently supported"
    //         );
    //         std::process::exit(1);
    //     }
    // }

    let mut aggregator: Box<dyn Aggregator> = match opt.aggregator {
        Aggregators::Maximum => (Box::new(MaximumAggregator::default()) as Box<dyn Aggregator>),
    };

    while let Ok(r) = source.read_data() {
        if r.is_empty() {
            break;
        }
        match parser.parse_data(r, opt.date_format.as_ref(), opt.timezone.as_ref()) {
            Ok(d) => {
                if let Err(e) = aggregator.update(&d) {
                    error!("Error occured in parsing: {:?}", e)
                }
            }
            Err(e) => error!("Error occured in parsing: {:?}", e),
        }
    }
    let _ = aggregator.output();
}
