use std::{collections::HashMap, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use colored_json::to_colored_json_auto;
use evtx::{EvtxParser, SerializedEvtxRecord};

mod event_id;

mod range;
use term_table::{row::Row, table_cell::TableCell};

#[derive(Parser)]
struct Cli {
    evtx_file: String,

    #[clap(long)]
    min: u64,

    #[clap(long)]
    max: u64
}
fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut record_ids: Vec<u64> = Vec::new();
    let mut records: HashMap<u64, SerializedEvtxRecord<serde_json::Value>> = HashMap::new();

    let path = PathBuf::try_from(&cli.evtx_file)?;

    let mut parser = EvtxParser::from_path(path)?;
    for record in parser.records_json_value() {
        match record {
            Err(_) => (),
            Ok(evt) => {
                let id = evt.event_record_id;
                if id >= cli.min && id <= cli.max {
                    record_ids.push(id);
                    records.insert(id, evt);
                }
            }
        }
    }
    record_ids.sort();

    let mut table = term_table::Table::new();
    termsize::get().map(|size| {
        table.set_max_column_widths(vec![
            (0, 12),
            (1, (size.cols - 16).into()),
        ])
    });

    for id in record_ids.into_iter() {
        let record = &records[&id];
        table.add_row(Row::new(vec![
            TableCell::new(id),
            TableCell::new(to_colored_json_auto(&record.data).unwrap()),
        ]));
    }
    println!("{}", table.render());

    Ok(())
}