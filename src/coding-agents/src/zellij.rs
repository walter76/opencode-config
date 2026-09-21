use std::io;
use std::process::Command;

pub fn list_sessions() -> io::Result<String> {
    run_command(["list-sessions"])
}

pub fn list_panes(session: &str) -> io::Result<String> {
    run_command(["--session", session, "action", "list-panes"])
}

fn run_command<const N: usize>(args: [&str; N]) -> io::Result<String> {
    let output = Command::new("zellij").args(args).output()?;

    if !output.status.success() {
        let message = String::from_utf8_lossy(&output.stderr);
        let message = message.trim();
        let message = if message.is_empty() {
            format!("zellij exited with status {}", output.status)
        } else {
            message.to_owned()
        };

        return Err(io::Error::other(message));
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}
