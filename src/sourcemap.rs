use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rbx_dom_weak::types::Ref;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::snapshot::RojoTree;

#[derive(Debug, Deserialize, Serialize)]
pub struct SourcemapNode {
    pub name: String,
    pub class_name: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub file_paths: Vec<PathBuf>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<SourcemapNode>,
}

pub(crate) fn recurse_create_node<'a>(
    tree: &'a RojoTree,
    referent: Ref,
    project_dir: &Path,
) -> Option<SourcemapNode> {
    let instance = tree.get_instance(referent).expect("instance did not exist");

    let children: Vec<_> = instance
        .children()
        .par_iter()
        .filter_map(|&child_id| recurse_create_node(tree, child_id, project_dir))
        .collect();

    // If this object has no children and doesn't pass the filter, it doesn't
    // contain any information we're looking for.
    if children.is_empty() {
        return None;
    }

    let file_paths = instance
        .metadata()
        .relevant_paths
        .iter()
        // Not all paths listed as relevant are guaranteed to exist.
        .filter(|path| path.is_file())
        .filter_map(|path| path.strip_prefix(project_dir).ok())
        .map(|path| path.to_path_buf())
        .collect();

    Some(SourcemapNode {
        name: instance.name().into(),
        class_name: instance.class_name().into(),
        file_paths,
        children,
    })
}
