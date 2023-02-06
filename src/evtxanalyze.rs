use clap::Parser;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

use crate::analyze::{pstree::display_pstree, Cli};

mod analyze;
mod data;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    TermLogger::init(
        cli.verbose.log_level_filter(),
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto,
    )?;

    match &cli.command {
        //TODO: move `display_pstree` into `impl Cli`
        analyze::Command::PsTree { username } => display_pstree(&cli, username),
        analyze::Command::Sessions { } => cli.display_sessions(),
    }
}
