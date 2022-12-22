use clap::{ValueEnum, Parser, Subcommand};

#[derive(ValueEnum, Clone)]
pub enum Format {
    Json,
    Markdown,
}

#[derive(Subcommand)]
pub enum Command {
    /// generate a process tree
    #[clap(name="pstree")]
    PsTree {
        /// display only processes of this user (case insensitive regex search)
        #[clap(short('U'), long("username"))]
        username: Option<String>,
    }
}

#[derive(Parser)]
pub (crate) struct Cli {
    /// Name of the evtx file to parse
    pub (crate) evtx_file: String,

    #[command(subcommand)]
    pub (crate) command: Command,

    #[clap(short('F'), long("format"), value_enum, default_value_t=Format::Json)]
    pub (crate) format: Format,

    #[command(flatten)]
    pub (crate) verbose: clap_verbosity_flag::Verbosity,
}