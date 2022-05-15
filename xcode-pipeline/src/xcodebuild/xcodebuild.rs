use rand::Rng;

use crate::filesystem::filesystem_repository::FileSystemRepository;

use super::xcodebuild_command_factory::XcodebuildCommandFactory;

pub trait XcodebuildContext {
    fn setup(&self);
    fn archive(&self);
    fn export(&self);
    fn tear_down(&self);
}

pub struct XcodebuildContextImpl<'a> {
    workspace: String,
    schema: String,
    dry_run: bool,
    storage_folder: String,
    filesystem_repository: &'a dyn FileSystemRepository,
}

impl<'a> XcodebuildContextImpl<'a> {
    pub fn new(
        workspace: &str,
        schema: &str,
        dry_run: bool,
        storage_folder_root: &str, // save some directory type
        filesystem_repository: &'a dyn FileSystemRepository,
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
            filesystem_repository: filesystem_repository,
        }
    }
}

impl XcodebuildContext for XcodebuildContextImpl<'_> {
    fn setup(&self) {
        println!("Using storage directory {}", self.storage_folder);
        if !self.dry_run {
            self.filesystem_repository
                .create_directory(&self.storage_folder)
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
        if !self.dry_run {
            self.filesystem_repository
                .delete_directory(&self.storage_folder)
                .expect("couldn't delete temp storage folder");
        }
    }
}
