use std::{path::Path, process::Command};

pub struct XcodebuildCommandFactory {}

impl XcodebuildCommandFactory {
    pub fn build_clean_archive(workspace: &Path, schema: &str, dry_run: bool) -> Command {
        let mut command = Command::new("xcodebuild");
        if dry_run {
            command.arg("dry-run");
        }
        command
            .args(["-workspace", workspace.to_str().unwrap()])
            .args(["-scheme", schema])
            .args(["-destination", "'generic/platform=iOS'"])
            .args(["clean", "archive"]);
        command
    }
}
