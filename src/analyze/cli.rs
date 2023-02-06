use std::path::PathBuf;

use clap::{ValueEnum, Parser, Subcommand};

use super::sessions::SessionStore;

#[derive(ValueEnum, Clone)]
pub enum Format {
    Json,
    Markdown,

    #[clap(name="latex")]
    LaTeX,

    Dot
}

#[derive(Subcommand)]
pub enum Command {
    /// generate a process tree
    #[clap(name="pstree")]
    PsTree {
        /// display only processes of this user (case insensitive regex search)
        #[clap(short('U'), long("username"))]
        username: Option<String>,
    },

    #[clap(name="sessions")]
    Sessions {

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

impl Cli {
    pub fn display_sessions(&self) -> anyhow::Result<()> {
        let evtx_files = vec![PathBuf::from(&self.evtx_file)];
        let sessions = SessionStore::try_from(evtx_files)?;
        Ok(())
    }
}