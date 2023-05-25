use std::{io::stdout, path::PathBuf};

use clap::{Parser, Subcommand, ValueEnum};

use super::sessions::SessionStore;

#[derive(ValueEnum, Clone)]
pub enum Format {
    Json,
    Markdown,
    Csv,

    #[clap(name = "latex")]
    LaTeX,

    Dot,
}

#[derive(Subcommand)]
pub enum Command {
    /// generate a process tree
    #[clap(name = "pstree")]
    PsTree {
        /// display only processes of this user (case insensitive regex search)
        #[clap(short('U'), long("username"))]
        username: Option<String>,

        /// Name of the evtx file to parse
        evtx_file: PathBuf,
    },

    /// display sessions
    #[clap(name = "sessions")]
    Sessions {
        /// Names of the evtx files to parse
        evtx_files: Vec<PathBuf>,

        /// include anonymous sessions
        #[clap(long("include-anonymous"))]
        include_anonymous: bool,
    },
}

#[derive(Parser)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,

    #[clap(short('F'), long("format"), value_enum, default_value_t=Format::Csv)]
    pub(crate) format: Format,

    #[command(flatten)]
    pub(crate) verbose: clap_verbosity_flag::Verbosity,
}

impl Cli {
    pub fn display_sessions(&self) -> anyhow::Result<()> {
        match &self.command {
            Command::Sessions {
                evtx_files,
                include_anonymous,
            } => {
                let sessions = SessionStore::import(evtx_files, *include_anonymous)?;

                match self.format {
                    Format::Json => {
                        for session in sessions {
                            session.into_json(&mut stdout().lock())?;
                        }
                    }
                    Format::Markdown => {
                        for session in sessions {
                            println!("{}", session.into_markdown());
                        }
                    }
                    Format::LaTeX => {
                        for session in sessions {
                            println!("{}", session.into_latex());
                        }
                    }
                    Format::Dot => {
                        for session in sessions {
                            println!("{}", session.into_dot());
                        }
                    }
                    Format::Csv => {
                        let mut csv_writer = csv::Writer::from_writer(stdout());
                        for session in sessions {
                            session.into_csv(&mut csv_writer)?;
                        }
                        csv_writer.flush()?;
                    }
                }

                Ok(())
            }
            _ => unreachable!(),
        }
    }
}
