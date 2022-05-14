use std::fs::DirBuilder;
use std::fs::remove_dir_all;

use super::filesystem_repository::FileSystemRepository;

pub struct FileSystemRepositoryFsImpl {}

impl FileSystemRepository for FileSystemRepositoryFsImpl {
    fn create_directory(&self, abs_path: &str) -> std::io::Result<()> {
        println!("Creating directory on path {}", abs_path);
        DirBuilder::new()
            .create(abs_path)
    }

    fn delete_directory(&self, abs_path: &str) -> std::io::Result<()> {
        println!("Deleting directory recursively on path {}", abs_path);
        remove_dir_all(abs_path)
    }
}