use clap::Parser;

use super::Rfc3339Datetime;
use regex::Regex;


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
    pub (crate) highlight: Option<Regex>
}
