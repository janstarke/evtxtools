use std::{
    io::{Read, Seek},
    path::PathBuf,
};

use anyhow::Result;
use colored::{Colorize, control::SHOULD_COLORIZE};
use data::EventId;
use evtx::{EvtxParser, SerializedEvtxRecord, ParserSettings};
mod data;

use serde_json::Value;

mod ls;
use ls::{Cli, HighlightedStringBuilder};
use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let hs_builder = HighlightedStringBuilder::new(cli.highlight.clone());
   
    for f_name in cli.evtx_file.iter() {
        let path = PathBuf::try_from(&f_name)?;

        let settings = ParserSettings::default().num_threads(0);
        let parser = EvtxParser::from_path(path)?.with_configuration(settings);

        display_results(parser, &cli, &hs_builder)?;
    }

    Ok(())
}

fn display_results<T: Read + Seek>(mut parser: EvtxParser<T>, cli: &Cli, hs_builder: &HighlightedStringBuilder) -> Result<()> {

    if cli.display_colors {
        SHOULD_COLORIZE.set_override(true);
    }

    for result in parser.records_json_value() {
        match result {
            Err(_) => (),
            Ok(record) => {

                if let Some(not_before) = cli.not_before.as_ref() {
                    if &record.timestamp < not_before {
                        continue;
                    }
                }

                if let Some(not_after) = cli.not_after.as_ref() {
                    if &record.timestamp < not_after {
                        continue;
                    }
                }

                if ! cli.filter_event_ids.is_empty() {
                    let event_id = EventId::try_from(&record)?.into();
                    if ! cli.filter_event_ids.contains(&event_id) {
                        continue;
                    }
                }

                display_record(&record, cli, hs_builder)?
            }
        }
    }
    Ok(())
}


fn display_record(record: &SerializedEvtxRecord<Value>, cli: &Cli, hs_builder: &HighlightedStringBuilder) -> Result<()> {
    let size = record.data.to_string().len();
    let event_id = EventId::try_from(record)?;
    let user_data = record.data["Event"]
        .get("UserData")
        .map(|user_data| hs_builder.highlight_data(user_data).to_string())
        .unwrap_or_else(|| "".to_owned());

    let event_data = record.data["Event"]
        .get("EventData")
        .map(|event_data| hs_builder.highlight_data(event_data).to_string())
        .unwrap_or(user_data)
        .normal();
    
    let event_data = event_data.replace("\\u001b", "\u{001b}");
    
    let event_id = if event_id == 4624.into() {
        event_id.to_string().bright_yellow()
    } else {
        event_id.to_string().normal()
    };

    let output = match cli.delimiter {
        None => format!(
            "{:12} {} {:8} {:5} {}",
            record.event_record_id,
            record.timestamp.format("%FT%T"),
            size,
            event_id,
            event_data
        ),
        Some(d) => format!(
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
    }.normal();

    println!("{output}");

    Ok(())
}
