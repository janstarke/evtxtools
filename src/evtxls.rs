use std::{
    io::{Read, Seek},
    path::PathBuf,
};

use anyhow::Result;
use clap::Parser;
use evtx::{EvtxParser, SerializedEvtxRecord};

mod event_id;

mod range;
use serde_json::Value;

/// Display one or more events from an evtx file
#[derive(Parser)]
#[clap(author,version,name=env!("CARGO_BIN_NAME"))]
struct Cli {
    /// Name of the evtx file to read from
    evtx_file: Vec<String>,

    /// use this delimiter instead of generating fixed space columns
    #[clap(short('d'), long("delimiter"))]
    delimiter: Option<char>,

    /// produce bodyfile output (ignores the `delimiter` option)
    #[clap(short('b'), long("bodyfile"))]
    generate_bodyfile: bool,

    /// List events with only the specified event ids
    #[clap(short('i'), long("event-id"), use_value_delimiter=true, value_delimiter=',')]
    filter_event_ids: Vec<u64>,

    
}
fn main() -> Result<()> {
    let cli = Cli::parse();

    for f_name in cli.evtx_file.iter() {
        let path = PathBuf::try_from(&f_name)?;

        let parser = EvtxParser::from_path(path)?;

        display_results(parser, &cli);
    }

    Ok(())
}

fn display_results<T: Read + Seek>(mut parser: EvtxParser<T>, cli: &Cli) {
    for result in parser.records_json_value() {
        match result {
            Err(_) => (),
            Ok(record) => {
                if ! cli.filter_event_ids.is_empty() {
                    let event_id = &record.data["Event"]["System"]["EventID"];
                    if let Some(event_id) = event_id.as_u64() {
                        if ! cli.filter_event_ids.contains(&event_id) {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }

                display_record(&record, cli)
            }
        }
    }
}

fn display_record(record: &SerializedEvtxRecord<Value>, cli: &Cli) {
    let size = record.data.to_string().len();
    let event_id = &record.data["Event"]["System"]["EventID"];
    let user_data = record.data["Event"]
        .get("UserData")
        .map(|user_data| user_data.to_string());

    let event_data = record.data["Event"]
        .get("EventData")
        .map(|event_data| event_data.to_string())
        .or(user_data.or(Some("".to_owned())))
        .unwrap();

    match cli.delimiter {
        None => println!(
            "{:12} {} {:8} {:5} {}",
            record.event_record_id,
            record.timestamp.format("%FT%T"),
            size,
            event_id,
            event_data
        ),
        Some(d) => println!(
            "{}{}{}{}{}{}{}{}{}",
            record.event_record_id,
            d,
            record.timestamp.format("%FT%T"),
            d,
            size,
            d,
            event_id,
            d,
            event_data
        ),
    }
}
