use std::{
    collections::HashMap,
    io::{Read, Seek},
    path::PathBuf,
};

use anyhow::Result;
use clap::Parser;
use colored_json::to_colored_json_auto;
use evtx::{EvtxParser, SerializedEvtxRecord};

mod event_id;

mod range;
use term_table::{row::Row, table_cell::TableCell};

/// Display one or more events from an evtx file
#[derive(Parser)]
#[clap(author,version,name=env!("CARGO_BIN_NAME"))]
struct Cli {
    /// Name of the evtx file to read from
    evtx_file: String,

    /// filter: minimal event record identifier
    #[clap(long)]
    min: Option<u64>,

    /// filter: maximal event record identifier
    #[clap(long)]
    max: Option<u64>,

    /// show only the one event with this record identifier
    #[clap(short, long)]
    id: Option<u64>,
}
fn main() -> Result<()> {
    let cli = Cli::parse();

    let path = PathBuf::try_from(&cli.evtx_file)?;

    let parser = EvtxParser::from_path(path)?;

    let (record_ids, records) = if let Some(filter_id) = cli.id {
        filter_by_id(parser, filter_id)
    } else {
        let min = cli.min.or(Some(u64::MIN)).unwrap();
        let max = cli.max.or(Some(u64::MAX)).unwrap();
        filter_by_range(parser, min, max)
    };

    display_results(record_ids, records);

    Ok(())
}

fn filter_by_id<T: Read + Seek>(
    mut parser: EvtxParser<T>,
    filter_id: u64,
) -> (
    Vec<u64>,
    HashMap<u64, SerializedEvtxRecord<serde_json::Value>>,
) {
    let mut record_ids: Vec<u64> = Vec::new();
    let mut records: HashMap<u64, SerializedEvtxRecord<serde_json::Value>> = HashMap::new();
    match parser.records_json_value().find(|record| match record {
        Ok(evt) => evt.event_record_id == filter_id,
        _ => false,
    }) {
        Some(result) => {
            let evt = result.unwrap();
            record_ids.push(evt.event_record_id);
            records.insert(evt.event_record_id, evt);
        }
        None => (),
    }
    (record_ids, records)
}

fn filter_by_range<T: Read + Seek>(
    mut parser: EvtxParser<T>,
    min: u64,
    max: u64,
) -> (
    Vec<u64>,
    HashMap<u64, SerializedEvtxRecord<serde_json::Value>>,
) {
    let mut record_ids: Vec<u64> = Vec::new();
    let mut records: HashMap<u64, SerializedEvtxRecord<serde_json::Value>> = HashMap::new();

    for record in parser.records_json_value() {
        match record {
            Err(_) => (),
            Ok(evt) => {
                let id = evt.event_record_id;

                if id >= min && id <= max {
                    record_ids.push(id);
                    records.insert(id, evt);
                }
            }
        }
    }

    record_ids.sort();
    (record_ids, records)
}

fn display_results(
    record_ids: Vec<u64>,
    records: HashMap<u64, SerializedEvtxRecord<serde_json::Value>>,
) {
    let mut table = term_table::Table::new();
    termsize::get()
        .map(|size| table.set_max_column_widths(vec![(0, 12), (1, (size.cols - 16).into())]));

    for id in record_ids.into_iter() {
        let record = &records[&id];
        table.add_row(Row::new(vec![
            TableCell::new(id),
            TableCell::new(to_colored_json_auto(&record.data).unwrap()),
        ]));
    }
    println!("{}", table.render());
}
