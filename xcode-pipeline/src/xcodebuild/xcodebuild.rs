use std::process::{Command,Child, ExitStatus};
use std::fs::{DirBuilder, DirEntry};
use rand::Rng;

use super::xcodebuild_command_factory::XcodebuildCommandFactory;

pub trait XcodebuildContext {
    fn setup(&self);
    fn archive(&self);
    fn export(&self);
    fn tear_down(&self);
}

pub struct XcodebuildContextImpl {
    workspace: String,
    schema: String,
    dry_run: bool,
    storage_folder: String,
}

impl XcodebuildContextImpl {
    pub fn new(
        workspace: &str,
        schema: &str,
        dry_run: bool,
        storage_folder_root: &str // save some directory type
    ) -> Self {
        let mut storage_name = storage_folder_root.to_string();
        let mut rng = rand::thread_rng();
        let rand_id: i32 = rng.gen();
        storage_name.push_str(&rand_id.to_string());

        Self {
            workspace: workspace.to_string(),
            schema: schema.to_string(),
            dry_run,
            storage_folder: storage_name,
        }
    }
}

impl XcodebuildContext for XcodebuildContextImpl {
    fn setup(&self) {
        println!("Using storage directory {}", self.storage_folder);
        if !self.dry_run {
            let storage_folder = DirBuilder::new()
                .create(self.storage_folder.clone())
                .expect("couldn't create storage folder");
            
        }
    }

    fn archive(&self) {
        XcodebuildCommandFactory::build_clean_archive(&self.workspace, &self.schema, self.dry_run)
            .status()
            .expect("Couldn't run archive");
    }

    fn export(&self) {
        // export
        unimplemented!()
    }

    fn tear_down(&self) {
        // delete temp folders
    }
}
