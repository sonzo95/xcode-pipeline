use std::{
    fs::{self, FileType},
    path::{Path, PathBuf},
    process::Command,
};

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

    pub fn build_export(
        &self,
        schema: &str,
        archive_folder: &Path,
        export_options_plist: &Path,
    ) -> Command {
        let mut command = Command::new("xcodebuild");
        if self.dry_run {
            command.arg("-dry-run");
        }
        let mut archive_path_buf = archive_folder.to_path_buf();
        let mut export_path_buf = archive_path_buf.clone();
        let archive_name = format!("{}.xcarchive", schema);
        archive_path_buf.push(&archive_name);
        export_path_buf.push(&schema);

        command
            .arg("-exportArchive")
            .args(["-archivePath", &archive_path_buf.to_str().unwrap()])
            .args(["-exportPath", &export_path_buf.to_str().unwrap()])
            .args([
                "-exportOptionsPlist",
                export_options_plist.to_str().unwrap(),
            ]);
        command
    }

    pub fn build_upload(
        &self,
        schema: &str,
        export_folder: &Path,
        username: &str,
        password: &str,
    ) -> Command {
        let mut command = Command::new("xcrun");
        command.arg("altool").arg("--upload-app");

        if self.dry_run {
            command.arg("-dry-run");
        }

        let mut export_path_buf = export_folder.to_path_buf();
        export_path_buf.push(&schema);

        let mut export_dir = fs::read_dir(export_path_buf.clone()).expect("Couldn't open export directory");
        let ipa_file = export_dir
            .find_map(|item| {
                if let Ok(item) = item {
                    if item.path().to_str().unwrap().ends_with(".ipa") {
                        Some(item.path())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .expect("Couldn't find the ipa file");
        export_path_buf.push(ipa_file.components().last().unwrap());

        command
            .args(["-t", "ios"])
            .args(["-f", &export_path_buf.to_str().unwrap()])
            .args(["-u", username])
            .args(["-p", password]);
        command
    }
}
