use std::{collections::HashMap, path::Path};

pub use git::Git;

mod git;

// TODO: Move to helix_core once we have a generic diff mode
#[derive(Copy, Clone, Debug)]
pub enum LineDiff {
    Added,
    Deleted,
    Modified,
}

/// Maps line numbers to changes
pub type LineDiffs = HashMap<usize, LineDiff>;

trait DiffProvider {
    fn get_file_head(&self, file: &Path) -> Option<Vec<u8>>;
}
pub struct DiffProviderRegistry {
    providers: Vec<Box<dyn DiffProvider>>,
}

impl DiffProviderRegistry {
    pub fn new() -> DiffProviderRegistry {
        // currently only git is supported
        // TODO make this configurable when more providers are added
        let git: Box<dyn DiffProvider> = Box::new(Git);
        let providers = vec![git];
        DiffProviderRegistry { providers }
    }
    pub fn get_file_head(&self, file: &Path) -> Option<Vec<u8>> {
        self.providers
            .iter()
            .find_map(|provider| provider.get_file_head(file))
    }
}

impl Default for DiffProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}
