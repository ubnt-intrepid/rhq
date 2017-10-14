use std::process::{Command, Stdio};

pub fn inherit(name: &str) -> Command {
    let mut command = Command::new(name);
    command.stdin(Stdio::inherit());
    command.stdout(Stdio::inherit());
    command.stderr(Stdio::inherit());
    command
}

pub fn piped(name: &str) -> Command {
    let mut command = Command::new(name);
    command.stdin(Stdio::null());
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());
    command
}
