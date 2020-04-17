use crate::{hir::NameId, HirDatabase};
use errors::{FileId, WithError};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ModuleGraph {
    nodes: HashSet<FileId>,
    edges: HashMap<FileId, HashMap<NameId, FileId>>,
}

impl ModuleGraph {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert_node(&mut self, node: FileId) {
        self.nodes.insert(node);
    }

    pub fn insert_edges(&mut self, from: FileId, to: FileId, weight: NameId) {
        {
            let edges = self.edges.entry(from).or_default();

            edges.insert(weight, to);
        }
        let _ = self.edges.entry(to).or_default();
    }

    pub fn try_get_node(&self, file: &FileId) -> Option<&HashMap<NameId, FileId>> {
        self.edges.get(file)
    }

    pub fn get_node(&self, file: &FileId) -> &HashMap<NameId, FileId> {
        &self.edges[file]
    }
}

pub(crate) fn module_graph_query(db: &impl HirDatabase, file: FileId) -> WithError<ModuleGraph> {
    let program = db.lower(file)?;

    let mut module_graph = ModuleGraph::new();

    for module in &program.modules {
        module_graph.insert_edges(file, db.resolve_modules(file, module.id)?, module.name);
    }

    Ok(module_graph)
}
