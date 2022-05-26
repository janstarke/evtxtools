use std::{
    io::{Read, Seek},
    path::PathBuf,
};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

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
    evtx_file: String,

    /// use this delimiter instead of generating fixed space columns
    #[clap(short('d'), long("delimiter"))]
    delimiter: Option<char>

}
fn main() -> Result<()> {
    let cli = Cli::parse();

    let path = PathBuf::try_from(&cli.evtx_file)?;

    let parser = EvtxParser::from_path(path)?;

    display_results(parser, cli);

    Ok(())
}

fn display_results<T: Read+Seek>(
    mut parser: EvtxParser<T>,
    _cli: Cli,
) {
    let (tx, rx): (Sender<SerializedEvtxRecord<Value>>, Receiver<SerializedEvtxRecord<Value>>) = mpsc::channel();
    let printer = thread::spawn(move || {
        printer_worker(rx, _cli);
    });
    for result in parser.records_json_value() {
        match result {
            Err(_) => (),
            Ok(record) => {
                if tx.send(record).is_err() {
                    break;
                }
            }
        }
    }
    drop(tx);
    printer.join().unwrap();
}

fn printer_worker(rx: Receiver<SerializedEvtxRecord<Value>>, cli: Cli) {
    loop {
        match rx.recv() {
            Err(_) => return,
            Ok(record) => display_record(&record, &cli)
        }
    }
}

fn display_record(record: &SerializedEvtxRecord<Value>, cli: &Cli) {
    let size = record.data.to_string().len();
    let event_id = &record.data["Event"]["System"]["EventID"];
    let user_data = match record.data["Event"].get("UserData") {
        None => None,
        Some(user_data) => {
            Some(user_data.to_string())
        }
    };

    let event_data = match record.data["Event"].get("EventData") {
        None => None,
        Some(event_data) => {
            Some(event_data.to_string())
        }
    };

    match cli.delimiter {
        None => println!("{:12} {} {:8} {:5} {}",
        record.event_record_id,
        record.timestamp.format("%FT%T"),
        size,
        event_id,
        event_data.or(user_data.or(Some("".to_owned()))).unwrap()
    ),
        Some(d) => println!("{}{}{}{}{}{}{}{}{}",
        record.event_record_id, d,
        record.timestamp.format("%FT%T"), d,
        size, d,
        event_id, d,
        event_data.or(user_data.or(Some("".to_owned()))).unwrap()
    )
    }

    
}

