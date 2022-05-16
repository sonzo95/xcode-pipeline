use std::{
    env,
    path::{Path, PathBuf},
};

use git2::{Cred, Error, RemoteCallbacks};

use super::git_service::GitService;

pub struct GitServiceImpl {}

impl GitService for GitServiceImpl {
    fn clone(
        &self,
        url: &str,
        into: &std::path::Path,
        branch: Option<&str>,
        ssh_key_name: &str,
    ) -> Result<PathBuf, Error> {
        // Prepare callbacks.
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            Cred::ssh_key(
                username_from_url.unwrap(),
                None,
                Path::new(&format!("{}/.ssh/{}", env::var("HOME").unwrap(), ssh_key_name)),
                None,
            )
        });

        // Prepare fetch options.
        let mut fo = git2::FetchOptions::new();
        fo.remote_callbacks(callbacks);

        // Prepare builder.
        let mut builder = git2::build::RepoBuilder::new();
        builder.fetch_options(fo);

        // Add branch/tag if present.
        if let Some(branch) = branch {
            builder.branch(branch);
        }

        // Clone the project.
        builder
            .clone(url, into)
            .map(|repo| repo.path().to_path_buf())
    }
}
