//! COPC hierarchy wrapper.

use crate::copc::{Entry, VoxelKey};
use std::collections::HashMap;

/// The loaded COPC hierarchy — a map from voxel keys to their entries.
pub struct Hierarchy {
    entries: HashMap<VoxelKey, Entry>,
}

impl Hierarchy {
    pub(crate) fn new(entries: HashMap<VoxelKey, Entry>) -> Self {
        Hierarchy { entries }
    }

    /// Look up the entry for a given voxel key.
    pub fn get(&self, key: &VoxelKey) -> Option<&Entry> {
        self.entries.get(key)
    }

    /// Returns a reference to the underlying entries map.
    pub fn entries(&self) -> &HashMap<VoxelKey, Entry> {
        &self.entries
    }

    /// Number of entries in the hierarchy.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns `true` if the hierarchy contains no entries.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Iterator over all voxel keys in the hierarchy.
    pub fn keys(&self) -> impl Iterator<Item = &VoxelKey> {
        self.entries.keys()
    }
}
