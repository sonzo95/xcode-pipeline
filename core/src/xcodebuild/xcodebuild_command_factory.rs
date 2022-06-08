use std::{path::Path, process::Command};

pub struct XcodebuildCommandFactory {
    dry_run: bool,
}

impl XcodebuildCommandFactory {
    pub fn new(dry_run: bool) -> Self {
        XcodebuildCommandFactory { dry_run }
    }

    pub fn build_clean_archive(
        &self,
        workspace: &Path,
        schema: &str,
        archive_folder: &Path,
    ) -> Command {
        let mut command = Command::new("xcodebuild");
        if self.dry_run {
            command.arg("-dry-run");
        }
        let archive_name = format!("{}.xcarchive", schema);
        let mut archive_path_buf = archive_folder.to_path_buf();
        archive_path_buf.push(&archive_name);

        command
            .args(["-workspace", workspace.to_str().unwrap()])
            .args(["-scheme", schema])
            .args(["-destination", "generic/platform=iOS"])
            .args(["-archivePath", &archive_path_buf.to_str().unwrap()])
            .args(["clean", "archive"]);
        command
    }
}
