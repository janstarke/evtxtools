use clap::{Parser, ValueEnum};

use super::{Rfc3339Datetime, SystemField};
use regex::Regex;

#[derive(ValueEnum, Clone)]
pub (crate) enum SortOrder {
    /// don't change order, output records as they are stored
    Storage,

    /// sort by event record id
    RecordId,

    /// sort by date and time
    Time
}

/// Display one or more events from an evtx file
#[derive(Parser)]
#[clap(author,version,about,long_about=None)]
pub (crate) struct Cli {
    /// Name of the evtx file to read from
    pub (crate) evtx_file: Vec<String>,

    /// use this delimiter instead of generating fixed space columns
    #[clap(short('d'), long("delimiter"))]
    pub (crate) delimiter: Option<char>,

    /// produce bodyfile output (ignores the `delimiter` option)
    #[clap(short('b'), long("bodyfile"))]
    pub (crate) generate_bodyfile: bool,

    /// List events with only the specified event ids
    #[clap(short('i'), long("event-id"), use_value_delimiter=true, value_delimiter=',')]
    pub (crate) filter_event_ids: Vec<u16>,
    
    /// highlight interesting content using colors
    #[clap(short('c'), long("colors"))]
    pub (crate) display_colors: bool,

    /// hide events older than the specified date (hint: use RFC 3339 syntax)
    #[clap(short('f'), long("from"))]
    pub (crate) not_before: Option<Rfc3339Datetime>,

    /// hide events newer than the specified date (hint: use RFC 3339 syntax)
    #[clap(short('t'), long("to"))]
    pub (crate) not_after: Option<Rfc3339Datetime>,

    /// highlight event data based on this regular expression
    #[clap(short('r'), long("regex"))]
    pub (crate) highlight: Option<Regex>,

    /// sort order
    #[clap(short('s'), long("sort"), value_enum, default_value_t=SortOrder::Storage)]
    pub (crate) sort_order: SortOrder,

    /// display fields common to all events. multiple values must be separated by ','
    #[clap(short('y'), long("system-fields"), value_enum, use_value_delimiter=true, value_delimiter=',')]
    pub (crate) display_system_fields: Vec<SystemField>
}