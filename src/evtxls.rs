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
use ls::{Cli, FilterBySystemField, HighlightedStringBuilder, SortOrder};

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

        let mut records = Vec::new();

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

                    if matches!(self.cli.sort_order, SortOrder::Storage) {
                        self.display_record(&record)?
                    } else {
                        records.push(record);
                    }
                }
            }
        }

        match self.cli.sort_order {
            SortOrder::Storage => assert!(records.is_empty()),
            SortOrder::RecordId => {
                records.sort_by(|a, b| a.event_record_id.cmp(&b.event_record_id))
            }
            SortOrder::Time => records.sort_by(|a, b| a.timestamp.cmp(&b.timestamp)),
        }

        if !records.is_empty() {
            for record in records.into_iter() {
                self.display_record(&record)?;
            }
        }

        Ok(())
    }

    fn display_record(&self, record: &SerializedEvtxRecord<Value>) -> Result<()> {
        let system_fields = <SerializedEvtxRecord<Value> as FilterBySystemField>::filter_fields(
            record,
            &self.cli.display_system_fields[..],
        )?;

        let line_parts: Vec<String> = if self.cli.delimiter.is_none() {
            system_fields
                .iter()
                .map(|f| f.value_with_padding())
                .collect()
        } else {
            system_fields.iter().map(|f| f.to_string()).collect()
        };
        let system_fields = if line_parts.is_empty() {
            "".to_owned()
        } else {
            format!(
                "{}{}",
                line_parts.join(&self.cli.delimiter.unwrap_or(' ').to_string()),
                &self.cli.delimiter.unwrap_or(' ')
            )
        };

        let event_data = {
            let event = &record.data["Event"];

            let user_data = event
                .get("UserData")
                .map(|user_data| self.hs_builder.highlight_data(user_data).to_string())
                .unwrap_or_else(|| "".to_owned());

            let event_data = event
                .get("EventData")
                .map(|event_data| self.hs_builder.highlight_data(event_data).to_string())
                .unwrap_or(user_data)
                .normal();

            let mut event_data = event_data.replace("\\u001b", "\u{001b}");
            if event_data == "\"\"" {
                event_data = "".to_owned();
            }
            event_data
        };

        /*
        let event_id = if event_id == 4624.into() {
            event_id.to_string().bright_yellow()
        } else {
            event_id.to_string().normal()
        };
         */

        let output = match self.cli.delimiter {
            None => format!(
                "{} {system_fields}{event_data} {event_data}",
                record.timestamp.format("%FT%T%.3f")
            ),
            Some(d) => format!(
                "{}{d}{system_fields}{event_data}{d}{event_data}",
                record.timestamp.to_rfc3339()
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
