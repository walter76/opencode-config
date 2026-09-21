mod zellij;

fn main() {
    match zellij::list_sessions() {
        Ok(sessions) => print!("{sessions}"),
        Err(error) => {
            eprintln!("Unable to list zellij sessions: {error}");
            std::process::exit(1);
        }
    }
}
