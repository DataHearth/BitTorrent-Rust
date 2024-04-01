mod torrent;
mod tracker;

use std::io;

use clap::{Command, CommandFactory, Parser, Subcommand, ValueHint};
use clap_complete::{generate, Generator, Shell};
use torrent::{create, metadata, raw};
use tracker::check;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    commands: Option<Cmds>,
    #[arg(short, long, value_name = "SHELL", value_enum)]
    complete: Option<Shell>,
}

#[derive(Subcommand)]
enum Cmds {
    /// Torrent tooling
    Torrent {
        #[command(subcommand)]
        commands: TorrentCmds,
    },
    Tracker {
        #[command(subcommand)]
        commands: TrackerCmds,
    },
}

#[derive(Subcommand)]
enum TorrentCmds {
    /// Retrieve metadata from a ".torrent" file
    Metadata {
        /// Path to an existing torrent file
        #[arg(value_hint = ValueHint::FilePath)]
        path: String,
        /// BitTorrent specification V1
        #[arg(long, default_value_t = true)]
        v1: bool,
        /// BitTorrent specification V2
        #[arg(long)]
        v2: bool,
    },
    Raw {
        /// Path to an existing torrent file
        #[arg(value_hint = ValueHint::FilePath)]
        path: String,
    },
    /// Create a torrent file
    Create {
        /// Path to an existing torrent file
        #[arg(value_hint = ValueHint::FilePath)]
        path: String,
    },
}

#[derive(Subcommand)]
enum TrackerCmds {
    Check {
        #[arg(value_hint = ValueHint::FilePath)]
        path: String,
    },
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Some(sh) = cli.complete {
        print_completions(sh, &mut Cli::command())
    }

    if let Some(cmds) = cli.commands {
        match cmds {
            Cmds::Torrent { commands } => match commands {
                TorrentCmds::Metadata { path, v1, v2 } => metadata(v1, v2, path),
                TorrentCmds::Create { path } => create(path, String::new()),
                TorrentCmds::Raw { path } => raw(path),
            },
            Cmds::Tracker { commands } => match commands {
                TrackerCmds::Check { path } => check(path).await,
            },
        }
    }
}
