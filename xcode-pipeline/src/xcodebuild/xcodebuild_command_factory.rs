use std::{path::Path, process::Command};

pub struct XcodebuildCommandFactory {
    dry_run: bool,
}

impl XcodebuildCommandFactory {
    pub fn new(dry_run: bool) -> Self {
        XcodebuildCommandFactory { dry_run }
    }

    pub fn build_clean_archive(&self, workspace: &Path, schema: &str) -> Command {
        let mut command = Command::new("xcodebuild");
        if self.dry_run {
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
