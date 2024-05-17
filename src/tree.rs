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
/// let tree: Tree<i32, i32> = Tree::new();
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tree<Q, T> where Q: PartialEq + Eq + Clone, T: PartialEq + Eq + Clone {
	nodes: Vec<Node<Q, T>>,
}

impl<Q, T> Tree<Q, T> where Q: PartialEq + Eq + Clone, T: PartialEq + Eq + Clone {
	/// Create a new tree.
	///
	/// This method creates a new tree with no nodes.
	///
	/// # Returns
	///
	/// A new tree with no nodes.
	///
	/// # Example
	///
	/// ```rust
	/// # use tree_ds::prelude::Tree;
	///
	/// let tree: Tree<i32, i32> = Tree::new();
	/// ```
	pub fn new() -> Self {
		Tree::default()
	}

	/// Add a node to the tree.
	///
	/// This method adds a node to the tree. The node is added as a child of the parent node with the
	/// given parent id. If the parent id is `None`, the node is added as a root node. The node id is
	/// used to identify the node and the value is the value of the node. The value can be used to store
	/// any data that you want to associate with the node.
	///
	/// # Arguments
	///
	/// * `node_id` - The id of the node.
	/// * `value` - The value of the node.
	/// * `parent_id` - The id of the parent node. If `None`, the node is added as a root node.
	///
	/// # Returns
	///
	/// The id of the node that was added to the tree.
	///
	/// # Example
	///
	/// ```rust
	/// # use tree_ds::prelude::{Tree, Node};
	///
	/// let mut tree: Tree<i32, i32> = Tree::new();
	/// let node_id = tree.add_node(Node::new(1, Some(2)), None);
	/// ```
	pub fn add_node(&mut self, node: Node<Q, T>, parent_id: Option<Q>) -> Q {
		if let Some(parent_id) = parent_id {
			if let Some(parent) = self.nodes
									  .iter_mut()
									  .find(|n| n.get_node_id() == parent_id) {
				parent.add_child(node.clone());
			}
		}
		self.nodes.push(node.clone());
		node.get_node_id()
	}

	/// Get a node in the tree.
	///
	/// This method gets the node with the given node id in the tree.
	///
	/// # Arguments
	///
	/// * `node_id` - The id of the node.
	///
	/// # Returns
	///
	/// The node with the given node id in the tree or `None` if the node is not found.
	///
	/// # Example
	///
	/// ```rust
	/// # use tree_ds::prelude::{Node, Tree};
	///
	/// let mut tree: Tree<i32, i32> = Tree::new();
	///
	/// let node = Node::new(1, Some(2));
	/// tree.add_node(node.clone(), None);
	///
	/// assert_eq!(tree.get_node(1), Some(node));
	/// ```
	pub fn get_node(&self, node_id: Q) -> Option<Node<Q, T>> {
		self.nodes
			.iter()
			.find(|n| n.get_node_id() == node_id).cloned()
	}
}

impl<Q, T> Default for Tree<Q, T> where Q: PartialEq + Eq + Clone, T: PartialEq + Eq + Clone {
	fn default() -> Self {
		Tree {
			nodes: Vec::new(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_tree_new() {
		let tree = Tree::<u32, u32>::new();
		assert_eq!(tree.nodes.len(), 0);
	}

	#[test]
	fn test_tree_add_node() {
		let mut tree = Tree::new();
		let node_id = tree.add_node(Node::new(1, Some(2)), None);
		assert_eq!(tree.nodes.len(), 1);
		assert_eq!(node_id, 1);
	}

	#[test]
	fn test_tree_get_node() {
		let mut tree = Tree::new();
		let node = Node::new(1, Some(2));
		tree.add_node(node.clone(), None);
		assert_eq!(tree.get_node(1), Some(node));
		assert_eq!(tree.get_node(2), None);
	}
}
