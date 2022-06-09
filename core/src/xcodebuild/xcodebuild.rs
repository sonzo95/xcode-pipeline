use std::{path::{Path, PathBuf}, process::ExitStatus};

use rand::Rng;
use tracing::{event, Level};

use crate::{filesystem::FileSystemRepository, git::git_service::GitService};

use super::xcodebuild_command_factory::XcodebuildCommandFactory;

pub trait XcodebuildContext {
    fn setup(&self);
    fn archive(&self, schema: &str) -> ExitStatus;
    fn export(&self, schema: &str) -> ExitStatus;
    fn upload(&self, schema: &str, username: &str, password: &str) -> ExitStatus;
    fn tear_down(&self);
}

// LOCAL WORKSPACE

pub struct XcodebuildContextLocalWs {
    workspace: PathBuf,
    storage_folder: PathBuf,
    export_options_plist: PathBuf,
    filesystem_repository: Box<dyn FileSystemRepository>,
    command_factory: Box<XcodebuildCommandFactory>,
}

impl XcodebuildContextLocalWs {
    pub fn new(
        workspace: PathBuf,
        mut storage_folder_root: PathBuf,
        export_options_plist: PathBuf,
        filesystem_repository: Box<dyn FileSystemRepository>,
        command_factory: Box<XcodebuildCommandFactory>,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let rand_id: u32 = rng.gen();
        storage_folder_root.push(format!("xccd-{}", rand_id));

        Self {
            workspace,
            storage_folder: storage_folder_root,
            export_options_plist,
            filesystem_repository,
            command_factory,
        }
    }
}

impl XcodebuildContext for XcodebuildContextLocalWs {
    fn setup(&self) {
        event!(Level::TRACE, "Setup");
        event!(
            Level::DEBUG,
            "Creating storage directory {:?}", self.storage_folder.to_str()
        );
        self.filesystem_repository
            .create_directory(&self.storage_folder.to_str().unwrap())
            .expect("couldn't create storage folder");
    }

    fn archive(&self, schema: &str) -> ExitStatus {
        event!(Level::TRACE, "Archive");
        event!(
            Level::DEBUG,
            "Archiving schema '{}'", schema
        );
        self.command_factory
            .build_clean_archive(&self.workspace, schema, &self.storage_folder)
            .status()
            .expect("Couldn't run archive")
    }

    fn export(&self, schema: &str) -> ExitStatus {
        event!(Level::TRACE, "Export");
        event!(
            Level::DEBUG,
            "Exporting schema '{}'", schema
        );
        self.command_factory
            .build_export(schema, &self.storage_folder, &self.export_options_plist)
            .status()
            .expect("Couldn't run export")
    }

    fn upload(&self, schema: &str, username: &str, password: &str) -> ExitStatus {
        event!(Level::TRACE, "Upload");
        event!(
            Level::DEBUG,
            "Uploading schema '{}'", schema
        );
        self.command_factory
            .build_upload(schema, &self.storage_folder, username, password)
            .status()
            .expect("Couldn't run upload")
    }

    fn tear_down(&self) {
        event!(Level::TRACE, "Tear Down");
        event!(
            Level::DEBUG,
            "Deleting storage directory {:?}", self.storage_folder.to_str()
        );
        self.filesystem_repository
            .delete_directory(&self.storage_folder.to_str().unwrap())
            .expect("couldn't delete temp storage folder");
    }
}

// GIT WORKSPACE

pub struct XcodebuildContextGitWs<'a> {
    git_url: &'a str,
    git_root_folder: &'a str,
    branch: Option<&'a str>,
    storage_folder: PathBuf,
    filesystem_repository: &'a dyn FileSystemRepository,
    command_factory: &'a XcodebuildCommandFactory,
    git_service: &'a dyn GitService,
    ssh_key_name: String,
}

const DEFAULT_SSH_KEY_NAME: &'static str = "id_ed25519";

impl<'a> XcodebuildContextGitWs<'a> {
    pub fn new(
        git_url: &'a str,
        git_root_folder: &'a str,
        branch: Option<&'a str>,
        mut storage_folder_root: PathBuf,
        filesystem_repository: &'a dyn FileSystemRepository,
        command_factory: &'a XcodebuildCommandFactory,
        git_service: &'a dyn GitService,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let rand_id: u32 = rng.gen();
        storage_folder_root.push(format!("xc-cd-{}", rand_id));

        Self {
            git_url,
            git_root_folder,
            branch,
            storage_folder: storage_folder_root,
            filesystem_repository,
            command_factory,
            git_service,
            ssh_key_name: std::env::var("SSH_KEY_NAME").unwrap_or(DEFAULT_SSH_KEY_NAME.to_string()),
        }
    }
}

impl XcodebuildContext for XcodebuildContextGitWs<'_> {
    fn setup(&self) {
        println!(
            "Using storage directory {}",
            self.storage_folder.to_str().unwrap()
        );
        self.filesystem_repository
            .create_directory(&self.storage_folder.to_str().unwrap())
            .expect("couldn't create storage folder");
        println!("Cloning {} on branch {:?}", self.git_url, self.branch);
        self.git_service
            .clone(
                self.git_url,
                self.storage_folder.as_path(),
                self.branch,
                "id_",
            )
            .expect("Failed to clone repository");
    }

    fn archive(&self, schema: &str) -> ExitStatus {
        let mut workspace = self.storage_folder.clone();
        if let Some(git_dir) = std::fs::read_dir(self.storage_folder.clone())
            .expect("Could not open storage_folder")
            .find(|entry| entry.as_ref().unwrap().path().is_dir())
            .map(|res| res.unwrap())
        {
            workspace.push(git_dir.path());
        }

        self.command_factory
            .build_clean_archive(&workspace, schema, &self.storage_folder)
            .status()
            .expect("Couldn't run archive")
    }

    fn export(&self, schema: &str) -> ExitStatus {
        // export
        unimplemented!()
    }

    fn upload(&self, schema: &str, username: &str, password: &str) -> ExitStatus {
        todo!()
    }

    fn tear_down(&self) {
        println!(
            "Deleting storage directory {:?}",
            self.storage_folder.to_str()
        );
        self.filesystem_repository
            .delete_directory(&self.storage_folder.to_str().unwrap())
            .expect("couldn't delete temp storage folder");
    }
}
