use std::process::{Command, Stdio};

pub fn make_command(name: &str) -> Command {
  let mut command = Command::new(name);
  command.stdin(Stdio::inherit());
  command.stdout(Stdio::inherit());
  command.stderr(Stdio::inherit());
  command
}
