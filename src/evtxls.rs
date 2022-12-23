use std::{
    io::{Read, Seek},
    path::PathBuf, net::Ipv4Addr, str::FromStr, collections::HashMap,
};

use anyhow::Result;
use clap::Parser;
use colored::{Colorize, control::SHOULD_COLORIZE, ColoredString};
use data::EventId;
use evtx::{EvtxParser, SerializedEvtxRecord};
mod data;

use serde::Serialize;
use serde_json::{Value, json};
use lazy_regex::regex;

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
    filter_event_ids: Vec<u16>,
    
    // highlight interesting content using colors
    #[clap(short('c'), long("colors"))]
    display_colors: bool
}
fn main() -> Result<()> {
    let cli = Cli::parse();

    for f_name in cli.evtx_file.iter() {
        let path = PathBuf::try_from(&f_name)?;

        let parser = EvtxParser::from_path(path)?;

        display_results(parser, &cli)?;
    }

    Ok(())
}

fn display_results<T: Read + Seek>(mut parser: EvtxParser<T>, cli: &Cli) -> Result<()> {

    if cli.display_colors {
        SHOULD_COLORIZE.set_override(true);
    }

    for result in parser.records_json_value() {
        match result {
            Err(_) => (),
            Ok(record) => {
                if ! cli.filter_event_ids.is_empty() {
                    let event_id = EventId::try_from(&record)?.into();
                    if ! cli.filter_event_ids.contains(&event_id) {
                        continue;
                    }
                }

                display_record(&record, cli)?
            }
        }
    }
    Ok(())
}

struct HighlightedString {
    cstring: ColoredString
}

impl From<ColoredString> for HighlightedString {
    fn from(cstring: ColoredString) -> Self {
        Self {
            cstring
        }
    }
}

impl Serialize for HighlightedString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let s = self.cstring.to_string();
        serializer.serialize_str(&s)
    }
}

fn highlight_string(s: &str) -> HighlightedString {
    let ip_regex = regex!(r"\b(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\b");
    if ip_regex.is_match(s) {
        for c in ip_regex.captures_iter(s) {
            for m in c.iter().flatten() {
                let ip_addr = Ipv4Addr::from_str(m.as_str()).unwrap();
                if ip_addr.is_link_local() || ip_addr.is_loopback() {
                    continue;
                }
                if ip_addr.is_private() {
                    return s.bright_purple().into();
                }
                //if ip_addr.is_global() {
                    return s.bright_red().on_yellow().into();
                //}
            }
        }
    }

    if s.to_lowercase().contains("admin") {
        return s.bright_yellow().on_red().into()
    }

    s.bright_blue().into()
}

fn highlight_data(data: &Value) -> Value {
    match data {
        Value::String(s) => json!(highlight_string(s)),
        Value::Array(o) => {
            let s: Vec<_> = o.iter().map(highlight_data).collect();
            json!(s)
        }
        Value::Object(o) => {
            let s: HashMap<_, _> = o.iter()
                .map(|(k,v)| (
                    k, json!(highlight_data(v))
                )).collect();
            json!(s)
        },
        _ => data.clone(),
    }
}

fn display_record(record: &SerializedEvtxRecord<Value>, cli: &Cli) -> Result<()> {
    let size = record.data.to_string().len();
    let event_id = EventId::try_from(record)?;
    let user_data = record.data["Event"]
        .get("UserData")
        .map(|user_data| highlight_data(user_data).to_string());

    let event_data = record.data["Event"]
        .get("EventData")
        .map(|event_data| highlight_data(event_data).to_string())
        .or(user_data.or(Some("".to_owned())))
        .unwrap().normal();
    
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
