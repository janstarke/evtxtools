use clap::Parser;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

use crate::analyze::{pstree::display_pstree, Cli};

mod analyze;

fn main() -> anyhow::Result<()> {
    let cli = Cli::try_parse()?;

    TermLogger::init(
        cli.verbose.log_level_filter(),
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto,
    )?;

    match &cli.command {
        analyze::Command::PsTree { username } => display_pstree(&cli, username),
    }
}
