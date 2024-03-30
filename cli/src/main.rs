use clap::{self, Parser};

#[derive(clap::Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    Info {
        path: String,
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Info { path } => {
            match brs::torrent::parse(path) {
                Ok(v) => println!("{v}"),
                Err(e) => println!("{e}"),
            }
        }
    }
}
