use std::process::{Command,Child};

pub fn build() -> Child {
    Command::new("echo")
        .arg("Build")
        .spawn()
        .expect("Couldn't run command 'echo'")
}