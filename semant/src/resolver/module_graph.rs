use errors::FileId;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Default)]
pub struct ModuleGraph {
    edges: HashMap<FileId, HashSet<FileId>>,
}

impl ModuleGraph {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert_edges(&mut self, from: FileId, to: FileId) {
        let edges = self.edges.entry(from).or_default();
        edges.insert(to);
    }

    pub fn get_node(&self, file: &FileId) -> &HashSet<FileId> {
        &self.edges[file]
    }
}
