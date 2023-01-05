use std::{
    io::{Read, Seek},
    path::PathBuf,
};

use anyhow::Result;
use colored::{control::SHOULD_COLORIZE, Colorize};
use data::EventId;
use evtx::{EvtxParser, ParserSettings, SerializedEvtxRecord};
mod data;

use serde_json::Value;

mod ls;
use clap::Parser;
use ls::{Cli, HighlightedStringBuilder};

struct EvtxLs {
    cli: Cli,
    hs_builder: HighlightedStringBuilder,
}

impl EvtxLs {
    fn new() -> Self {
        let cli = Cli::parse();
        let hs_builder = HighlightedStringBuilder::new(cli.highlight.clone());

        Self { cli, hs_builder }
    }

    fn run(self) -> Result<()> {
        for f_name in self.cli.evtx_file.iter() {
            let path = PathBuf::try_from(&f_name)?;

            let settings = ParserSettings::default().num_threads(0);
            let parser = EvtxParser::from_path(path)?.with_configuration(settings);

            self.display_results(parser)?;
        }

        Ok(())
    }

    fn display_results<T: Read + Seek>(&self, mut parser: EvtxParser<T>) -> Result<()> {
        if self.cli.display_colors {
            SHOULD_COLORIZE.set_override(true);
        }

        for result in parser.records_json_value() {
            match result {
                Err(_) => (),
                Ok(record) => {
                    if let Some(not_before) = self.cli.not_before.as_ref() {
                        if &record.timestamp < not_before {
                            continue;
                        }
                    }

                    if let Some(not_after) = self.cli.not_after.as_ref() {
                        if &record.timestamp < not_after {
                            continue;
                        }
                    }

                    if !self.cli.filter_event_ids.is_empty() {
                        let event_id = EventId::try_from(&record)?.into();
                        if !self.cli.filter_event_ids.contains(&event_id) {
                            continue;
                        }
                    }

                    self.display_record(&record)?
                }
            }
        }
        Ok(())
    }

    fn display_record(&self, record: &SerializedEvtxRecord<Value>) -> Result<()> {
        let event_id = EventId::try_from(record)?;
        let user_data = record.data["Event"]
            .get("UserData")
            .map(|user_data| self.hs_builder.highlight_data(user_data).to_string())
            .unwrap_or_else(|| "".to_owned());

        let event_data = record.data["Event"]
            .get("EventData")
            .map(|event_data| self.hs_builder.highlight_data(event_data).to_string())
            .unwrap_or(user_data)
            .normal();

        let event_data = event_data.replace("\\u001b", "\u{001b}");

        let event_id = if event_id == 4624.into() {
            event_id.to_string().bright_yellow()
        } else {
            event_id.to_string().normal()
        };

        let output = match self.cli.delimiter {
            None => format!(
                "{:12} {} {event_id:5} {event_data}",
                record.event_record_id,
                record.timestamp.format("%FT%T"),
            ),
            Some(d) => format!(
                "{}{d}{}{d}{event_id}{d}{event_data}",
                record.event_record_id,
                record.timestamp.format("%FT%T"),
            ),
        }
        .normal();

        println!("{output}");
        
        Ok(())
    }
}

fn main() -> Result<()> {
    sigpipe::reset();
    EvtxLs::new().run()
}
