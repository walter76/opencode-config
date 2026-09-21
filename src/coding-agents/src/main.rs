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
    /// List all panes in a zellij session.
    ListPanes {
        /// The zellij session to inspect.
        #[arg(short = 's', long, default_value = "coding-agents")]
        session: String,
    },
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
        Command::ListPanes { session } => match zellij::list_panes(&session) {
            Ok(panes) => print!("{panes}"),
            Err(error) => {
                eprintln!("Unable to list panes for zellij session '{session}': {error}");
                std::process::exit(1);
            }
        },
    }
}
