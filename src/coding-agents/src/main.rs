use clap::{Parser, Subcommand};

mod zellij;

#[derive(Debug, Parser)]
#[command(version, about = "Manage coding agents in zellij sessions")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// List the available zellij sessions.
    ListSessions,
}

fn main() {
    match Cli::parse().command {
        Command::ListSessions => match zellij::list_sessions() {
            Ok(sessions) => print!("{sessions}"),
            Err(error) => {
                eprintln!("Unable to list zellij sessions: {error}");
                std::process::exit(1);
            }
        },
    }
}
