use std::io;
use std::process::Command;

pub fn list_sessions() -> io::Result<String> {
    let output = Command::new("zellij").arg("list-sessions").output()?;

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
