use std::path::{Path, PathBuf};

use git2::Error;

pub trait GitService {
    fn clone(
        &self,
        url: &str,
        into: &Path,
        branch: Option<&str>,
        ssh_key_name: &str,
    ) -> Result<PathBuf, Error>;
}
