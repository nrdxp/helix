use std::path::Path;

use git_repository::objs::tree::EntryMode;
use git_repository::{discover, Commit, ObjectId, Repository};

use crate::DiffProvider;

#[cfg(test)]
mod test;

pub struct Git;

impl DiffProvider for Git {
    fn get_file_head(&self, file: &Path) -> Option<Vec<u8>> {
        debug_assert!(!file.exists() || file.is_file());
        debug_assert!(file.is_absolute());

        // discover a repository, requires a directory so we call parent (should not fail but exit gracefully in that case)
        let repo = discover(file.parent()?).ok()?;
        let head = repo.head_commit().ok()?;
        let file_oid = find_file_in_commit(&repo, &head, file)?;

        let file_object = repo.find_object(file_oid).ok()?;
        Some(file_object.detach().data)
    }
}

/// Finds the object that contains the contents of a file at a specific commit.
fn find_file_in_commit(repo: &Repository, commit: &Commit, file: &Path) -> Option<ObjectId> {
    let repo_dir = repo.work_dir()?;
    let rel_path = file.strip_prefix(repo_dir).ok()?;
    let rel_path_components = byte_path_components(rel_path)?;
    let tree = commit.tree().ok()?;
    let tree_entry = tree.lookup_path(rel_path_components).ok()??;
    match tree_entry.mode {
        // not a file, everything is new, do not show diff
        EntryMode::Tree | EntryMode::Commit | EntryMode::Link => None,
        // found a file
        EntryMode::Blob | EntryMode::BlobExecutable => Some(tree_entry.oid),
    }
}

/// On unix paths are always raw bytes (that are usually utf-8) that can be passed to git directly.
/// This function is infallible
#[cfg(any(unix, target_os = "wasi"))]
fn byte_path_components(path: &Path) -> Option<impl Iterator<Item = &[u8]>> {
    #[cfg(unix)]
    use std::os::unix::ffi::OsStrExt;
    #[cfg(target_os = "wasi")]
    use std::os::wasi::ffi::OsStrExt;
    let components = path
        .components()
        .map(|component| component.as_os_str().as_bytes());
    Some(components)
}

/// On other platforms (windows) `OsStr` can only be converted to bytes if it is valid utf-8 (so fallible).
/// The path components need to be checked for invalid utf-8 before they can be passed to git.
/// Therefore this implementation collects the components into a temporary vector
#[cfg(not(any(unix, target_os = "wasi")))]
fn byte_path_components(path: &Path) -> Option<impl Iterator<Item = &[u8]>> {
    let components: Option<Vec<_>> = path
        .components()
        .map(|component| {
            component
                .as_os_str()
                .to_str()
                .map(|component| component.as_bytes())
        })
        .collect();
    components.map(Vec::into_iter)
}
