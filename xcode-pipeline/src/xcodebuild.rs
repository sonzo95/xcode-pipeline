use std::process::{Command,Child};
use std::fs::{DirBuilder, DirEntry};

trait XcodebuildContext {
    fn setup(&self);
    fn archive(&self) -> Child;
    fn export(&self) -> Child;
    fn tear_down(&self);
}

struct XcodebuildContextImpl {
    schema: String,
    dry_run: bool,
    storage_folder: String,
}

impl XcodebuildContextImpl {
    pub fn new(
        schema: String, 
        dry_run: bool, 
        storage_folder_root: &String // save some directory type
    ) -> Self {
        let mut storage_name = storage_folder_root.clone();
        storage_name.push_str("some-random-guid");

        println!("Using storage directory {}", storage_name);
        if !dry_run {
            let storage_folder = DirBuilder::new()
                .create(storage_name)
                .expect("couldn't create storage folder");
            // how to get new dir? what to save if dry run? trait for Context and separate dry run impl?
        }

        Self {
            schema,
            dry_run,
            storage_folder: storage_name,
        }
    }
}

impl XcodebuildContext for XcodebuildContextImpl {
    fn setup(&self) {
        // clone repo, create temp folders...
    }

    fn archive(&self) -> Child {
        let command = Command::new("xcodebuild")
            .arg("Build");
        if self.dry_run {
            command.arg("dry-run");
        }
        command
            .spawn()
            .expect("Couldn't run command 'echo'")
    }

    fn export(&self) -> Child {
        // export
        unimplemented!()
    }

    fn tear_down(&self) {
        // delete temp folders
    }
}
