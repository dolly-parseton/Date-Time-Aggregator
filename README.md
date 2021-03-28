# Date-Time-Aggregator
Date-Time-Aggregator or `dta` is a command line datetime aggregator tool for doing easy timelining of structured data. Utilising [`chrono`](https://github.com/chronotope/chrono), [`serde`](https://github.com/serde-rs/serde).

## Usage
There are multiple ways to use `dta` either as a tool to pipe data into or as one to read directly from file. What `dta` does not do is transform data, parsing options are purely as a means of pulling the date field from a structured format (For transforming data please see [`xsv`](https://github.com/BurntSushi/xsv) or [`jq`](https://github.com/stedolan/jq)) 

### Examples

<!-- Taking a date slice of some new line delimted JSON data, selecting the "timestamp" field as the field to read the datetime from.
```
$ cat data.json | dta --in-format jsonl -f "timestamp" range 2010-2020 > slice.json 
```
Splitting a dataset based on the month, selecting the second field as the date time field.
```
$ cat data.csv | dta -i csv -f 2 split 1m -o "data_split_%Y-%m.csv" 
```
Finding the earliest time value in a dataset, that dataset being a line delimted list of timestamps. `maximum` also available. The `d` flag is specifying the format of the timestamps being provided using the [strftime](https://docs.rs/chrono/*/chrono/format/strftime/index.html#specifiers) syntax.
```
$ cat timestamps | dta minimum -d "%Y-%m-%dT%H:%M:%S"
``` -->

Features included:
* Range
* Split
* Minimum, Maximum
* Count

*WIP
**Flatten a timestamp to a certain level, ie. 2021-01-01 12:01:02 to hour returns 12:01:02 

## Building

## Features
`dta` has several features when being used as a library including:
* csv, a feature enabling the parsing of CSV data.
* json, a feature enabling the parsing of JSON data.
