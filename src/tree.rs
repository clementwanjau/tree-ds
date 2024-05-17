use crate::prelude::Node;

/// A tree data structure.
///
/// This struct represents a tree data structure. A tree is a data structure that consists of nodes
/// connected by edges. Each node has a parent node and zero or more child nodes. The tree has a root
/// node that is the topmost node in the tree. The tree can be used to represent hierarchical data
/// structures such as file systems, organization charts, and family trees.
///
/// # Example
///
/// ```rust
/// # use tree_ds::prelude::Tree;
///
/// let tree = Tree::new();
/// ```
#[derive(Clone, Debug)]
pub struct Tree<Q, T> where Q: PartialEq + Eq + Clone, T: PartialEq + Eq + Clone {
	nodes: Vec<Node<Q, T>>,
}
