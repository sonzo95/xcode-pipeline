use std::io::Result;

pub trait FileSystemRepository {
    fn create_directory(&self, abs_path: &str) -> Result<()>;
    fn delete_directory(&self, abs_path: &str) -> std::io::Result<()>;
}