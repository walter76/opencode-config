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
    /// Send a planning task to the Planner pane.
    Plan {
        /// The zellij session to send the planning task to.
        #[arg(short = 's', long, default_value = "coding-agents")]
        session: String,

        /// The slug identifying the task to plan.
        task_slug: String,
    },
    /// Send an implementation task to the Implementer pane.
    Implement {
        /// The zellij session to send the implementation task to.
        #[arg(short = 's', long, default_value = "coding-agents")]
        session: String,

        /// The slug identifying the task to implement.
        task_slug: String,
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
        Command::Plan { session, task_slug } => {
            let message = format!(
                "Create a new session for the task {task_slug}. Afterwards create an implementation \
                 plan for the task and persist it in the created session log file.\r"
            );

            if let Err(error) = zellij::send_to_pane(&session, "Planner", &message) {
                eprintln!("Unable to send plan to the Planner pane: {error}");
                std::process::exit(1);
            }
        }
        Command::Implement { session, task_slug } => {
            let message = format!(
                "Implement the plan provided in the session log for the task {task_slug}.\r"
            );

            if let Err(error) = zellij::send_to_pane(&session, "Implementer", &message) {
                eprintln!("Unable to send implementation task to the Implementer pane: {error}");
                std::process::exit(1);
            }
        }
    }
}
