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
    aggregators::{
        count::{CountAggregator, CountsAggregator},
        max::{MaximumAggregator, MaximumsAggregator},
        min::{MinimumAggregator, MinimumsAggregator},
        range::RangeAggregator,
        split::SplitAggregator,
        Aggregator,
    },
    input::{simple::SimpleParser, Parser, Source},
};
use log::LevelFilter;
use simplelog::*;
use std::path::PathBuf;
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

    /// Provide either a YAML file or a directory containing YAML files (.yml) that match the FormatDictionary structure.
    #[structopt(long = "formats", short = "F")]
    formats: Option<PathBuf>,

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
    #[structopt(short, long)]
    transform: Option<String>,
}

#[derive(Debug, PartialEq, StructOpt, Clone)]
enum Aggregators {
    /// Maximum aggregation, returns the most recent date.
    Maximum,
    /// Maximums aggregation, returns the most recent date for a given increment.
    Maximums {
        /// Increment format string (Increment formats YYYY-MM-DD or HH:MM:SS or YYYY-MM-DD HH:MM:SS).
        #[structopt(short, long)]
        increment: String,
    },
    /// Minimum aggregation, returns the earliest date.
    Minimum,
    /// Minimums aggregation, returns the earliest date for a given increment.
    Minimums {
        /// Increment format string (Increment formats YYYY-MM-DD or HH:MM:SS or YYYY-MM-DD HH:MM:SS).
        #[structopt(short, long)]
        increment: String,
    },
    /// Count aggregation, returns the total number of entires with a valid date.
    Count,
    /// Counts aggregation, returns the counts of data for a given increment.
    Counts {
        /// Increment format string (Increment formats YYYY-MM-DD or HH:MM:SS or YYYY-MM-DD HH:MM:SS).
        #[structopt(short, long)]
        increment: String,
    },
    /// Split aggregate function, reads data and splits it into multiple files.
    Split {
        /// The directory split files are saved to.
        #[structopt(short = "d", long = "directory", parse(from_os_str))]
        output_directory: PathBuf,
        /// Provide a filename including date time format options, this is run against the relevant timestamp.
        /// The resulting string is used as the filename that data is sent to.
        #[structopt(short = "i", long)]
        filename: String,
    },
    Range {
        /// The start of the range being selected.
        #[structopt(short, long)]
        start: String,
        /// The end of the range being selected.
        #[structopt(short, long)]
        end: Option<String>,
        /// Match on everything outside of the range provided.
        #[structopt(short, long)]
        inverted: bool,
    },
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

    let mut formats = match opt.formats {
        None => None,
        Some(f) => match date_time_aggregator::FormatDictionary::from_file(f) {
            Ok(f) => Some(f),
            Err(e) => {
                eprintln!("Error whilst creating Format Dictionary: {}", e.reason);
                std::process::exit(1);
            }
        },
    };

    // Match based on the command line options to decide what todo.
    let mut source: Box<dyn Source> = match opt.glob {
        Some(ref g) => match FileSource::new(&g, true) {
            Ok(s) => (Box::new(s) as Box<dyn Source>),
            Err(e) => {
                eprintln!("Error whilst creating source: {}", e.reason);
                std::process::exit(1);
            }
        },
        None => (Box::new(StdinSource::default()) as Box<dyn Source>),
    };

    let parser: Box<dyn Parser> = match (opt.csv.as_ref(), opt.json.as_ref()) {
        (Some(_c), Some(_j)) => {
            // Error
            eprintln!("Error whilst creating parser: You can select either CSV or JSON");
            std::process::exit(1);
        }
        (Some(c), None) => (Box::new(CsvParser::new(*c)) as Box<dyn Parser>),
        (None, Some(j)) => (Box::new(JsonParser::new(j)) as Box<dyn Parser>),
        (None, None) => (Box::new(SimpleParser::default()) as Box<dyn Parser>),
    };

    let mut aggregator: Box<dyn Aggregator> = match opt.aggregator.clone() {
        Aggregators::Maximum => Box::new(MaximumAggregator::default()),
        Aggregators::Maximums { increment } => match MaximumsAggregator::new(increment) {
            Ok(a) => Box::new(a),
            Err(e) => {
                eprintln!("Error whilst creating aggregator: {}", e);
                std::process::exit(1);
            }
        },
        Aggregators::Minimum => Box::new(MinimumAggregator::default()),
        Aggregators::Minimums { increment } => match MinimumsAggregator::new(increment) {
            Ok(a) => Box::new(a),
            Err(e) => {
                eprintln!("Error whilst creating aggregator: {}", e);
                std::process::exit(1);
            }
        },
        Aggregators::Count => Box::new(CountAggregator::default()),
        Aggregators::Counts { increment } => match CountsAggregator::new(increment) {
            Ok(a) => Box::new(a),
            Err(e) => {
                eprintln!("Error whilst creating aggregator: {}", e);
                std::process::exit(1);
            }
        },
        Aggregators::Split {
            output_directory,
            filename,
        } => match SplitAggregator::new(output_directory, filename) {
            Ok(a) => Box::new(a),
            Err(e) => {
                eprintln!("Error whilst creating aggregator: {}", e);
                std::process::exit(1);
            }
        },
        Aggregators::Range {
            start,
            end,
            inverted,
        } => match RangeAggregator::new(start, end, inverted) {
            Ok(a) => Box::new(a) as Box<dyn Aggregator>,
            Err(e) => {
                eprintln!("Error whilst creating aggregator: {}", e);
                std::process::exit(1);
            }
        },
    };
    while let Ok(r) = source.read_data() {
        if r.is_empty() {
            break;
        }
        match parser.parse_data(
            r,
            opt.date_format.as_ref(),
            opt.timezone.as_ref(),
            formats.as_mut(),
            opt.transform.as_ref(),
        ) {
            Ok(d) => {
                if let Err(e) = aggregator.update(&d) {
                    eprintln!("Error occured in parsing: {:?}", e)
                }
            }
            Err(e) => eprintln!("Error occured in parsing: {:?}", e),
        }
    }
    match aggregator.return_value() {
        Ok(r) => {
            if !r.is_empty() {
                println!("{}", r)
            }
        }
        Err(e) => eprintln!("{}", e),
    }
}
