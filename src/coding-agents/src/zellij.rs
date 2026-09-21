use std::io;
use std::process::Command;

const PANE_NOT_FOUND: &str = "pane was not found";

pub fn list_sessions() -> io::Result<String> {
    run_command(["list-sessions"])
}

pub fn list_panes(session: &str) -> io::Result<String> {
    run_command(["--session", session, "action", "list-panes"])
}

pub fn send_to_pane(session: &str, pane_name: &str, chars: &str) -> io::Result<()> {
    let panes = list_panes_json(session)?;
    let panes: serde_json::Value =
        serde_json::from_str(&panes).map_err(|error| io::Error::other(error.to_string()))?;
    let pane_id = find_pane_id(&panes, pane_name)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, PANE_NOT_FOUND))?;

    run_command([
        "--session",
        session,
        "action",
        "write-chars",
        "--pane-id",
        &pane_id,
        chars,
    ])?;

    Ok(())
}

fn list_panes_json(session: &str) -> io::Result<String> {
    run_command([
        "--session",
        session,
        "action",
        "list-panes",
        "--json",
        "--all",
    ])
}

fn find_pane_id(value: &serde_json::Value, pane_name: &str) -> Option<String> {
    match value {
        serde_json::Value::Object(object) => {
            if object.get("title").and_then(serde_json::Value::as_str) == Some(pane_name) {
                return object
                    .get("id")
                    .and_then(|s| Some(format!("terminal_{}", s)))
                    .as_deref()
                    .map(str::to_owned);
            }

            object
                .values()
                .find_map(|value| find_pane_id(value, pane_name))
        }
        serde_json::Value::Array(values) => values
            .iter()
            .find_map(|value| find_pane_id(value, pane_name)),
        _ => None,
    }
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
